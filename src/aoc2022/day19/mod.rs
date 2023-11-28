use std::collections::{HashSet, VecDeque};

use crate::read;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    /// ```txt
    /// costs: [
    ///     ore: [ore, clay, obby],
    ///     clay: ...
    ///     obby: ...
    ///     geode: ...
    /// ]
    /// ```
    recipes: [[i32; 3]; 4],

    /// You can only make 1 robot per minute, so the max bots
    /// would be the max cost for each resource
    /// (no max for the geodes, duh)
    max: [i32; 4],
}

pub fn run() {
    let file = read!();

    // note: blueprint ids start at 1
    let blueprints: Vec<Blueprint> = file
        .map(|line| {
            let info_data = line.split_once(": ").unwrap().1;

            // each [robot] costs [x]
            let mut costs = info_data
                .split(". ")
                // costs {x ore and ...}
                .map(|sent| sent.split_once("costs ").unwrap().1.split(" "));

            // [x] "ore"
            let ore = costs.next().unwrap().next().unwrap().parse().unwrap();
            let clay = costs.next().unwrap().next().unwrap().parse().unwrap();

            // [x] "ore" "and" [y] "clay"
            let mut obby_costs = costs.next().unwrap();
            let obby_ore = obby_costs.next().unwrap().parse().unwrap();
            let obby_clay = obby_costs.skip(2).next().unwrap().parse().unwrap();

            let mut geode_costs = costs.next().unwrap();
            let geode_ore = geode_costs.next().unwrap().parse().unwrap();
            let geode_obby = geode_costs.skip(2).next().unwrap().parse().unwrap();

            Blueprint {
                recipes: [
                    [ore, 0, 0],
                    [clay, 0, 0],
                    [obby_ore, obby_clay, 0],
                    [geode_ore, 0, geode_obby],
                ],
                max: [
                    ore.max(clay).max(obby_ore).max(geode_ore),
                    obby_clay,
                    geode_obby,
                    i32::MAX,
                ],
            }
        })
        .collect();

    let out: i32 = blueprints
        .iter()
        .enumerate()
        .map(|(i, &bp)| (i + 1) as i32 * bfs(bp, 24))
        .sum();

    println!("Part1: {out}");

    let out: i32 = blueprints.iter().take(3).map(|&bp| bfs(bp, 32)).product();

    println!("Part2: {out}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    /// ore, clay, obby, geode
    bots: [i32; 4],
    /// ore, clay, obby, geode
    resources: [i32; 4],
    time: i32,
}

impl State {
    fn new(time: i32) -> Self {
        Self {
            bots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
            time,
        }
    }
}

/// optimizations:
/// - we only need to have max of [max cost] robots
///     - we can only make 1 robot / minute after all
/// - throw away extra ore
///     - you can't even spend it anyways
fn bfs(bp: Blueprint, time: i32) -> i32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let state = State::new(time);

    queue.push_back(state);
    visited.insert(state);

    let mut max = 0;

    while let Some(State {
        bots,
        resources,
        time,
    }) = queue.pop_front()
    {
        // value if you don't do anything for the remainder
        // of this state
        max = max.max(resources[3] + bots[3] * time);

        for (bot_kind, recipe) in bp.recipes.into_iter().enumerate() {
            if bots[bot_kind] == bp.max[bot_kind] {
                // don't make more bots than this point
                // because you can't spend that much
                continue;
            }

            // time it takes to mine resources to make the bot
            let mut wait_time = 0;
            for (r_kind, amt) in recipe.into_iter().enumerate() {
                if amt > 0 && bots[r_kind] == 0 {
                    // don't have enough (existing bots) to make this bot
                    wait_time = -1;
                    break;
                }

                let time = ((amt - resources[r_kind]) as f32 / bots[r_kind] as f32).ceil() as i32;
                wait_time = wait_time.max(time);
            }

            if wait_time != -1 {
                let elapsed = wait_time + 1; // takes 1 more minute to create the bot
                let rem_time = time - elapsed;

                if rem_time <= 0 {
                    // exceeds time limit to create the bot
                    // so you can't "jump" to the next state
                    continue;
                }

                let mut bots = bots;
                let mut resources = resources;

                // bots produce resources during that time

                for r_kind in 0..resources.len() {
                    resources[r_kind] += bots[r_kind] * elapsed;
                }

                // take the costs to make the new bot
                for (r_kind, amt) in recipe.into_iter().enumerate() {
                    resources[r_kind] -= amt;
                }

                bots[bot_kind] += 1;

                // OPTIMIZATION: throw away excess resources
                // max_bots * rem_time would be how much you can produce and still use
                // all resources efficiently
                // this reduces the # of state
                // since excess is essentially the same as ample
                for r_kind in 0..(bp.max.len() - 1) {
                    resources[r_kind] = resources[r_kind].min(bp.max[r_kind] * rem_time);
                }

                let state = State {
                    bots,
                    resources,
                    time: rem_time,
                };

                if !visited.contains(&state) {
                    visited.insert(state);
                    queue.push_front(state);
                }
            }
        }
    }

    max
}

/*
/// time = 24
/// optimizations:
/// - we only need to have max of [cost] robots
///     - we can only make 1 robot per minute
///
/// just bought is = max amt of ore of obsidian robot, to give chance to buy those
fn dfs(bp: Blueprint) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back(State::new());

    let mut max = 0;

    while let Some(State {
        resources,
        bots,
        time,
    }) = queue.pop_front()
    {
        if time <= 0 {
            max = max.max(resources[3]);
        }

        // choose which to buy, and calculate the amount of time it will take
        // to create that bot

        // (robot_kind, [ore, clay, obby])
        for (kind, recipe) in bp.recipes.iter().enumerate() {
            // println!("{kind} eq => {} {}", bots[kind], bp.max[kind]);
            if bots[kind] >= bp.max[kind] {
                assert_eq!(bots[kind], bp.max[kind]);
                // don't make more than max
                continue;
            }

            let mut wait_time = 0;
            for (r_type, &r_needed) in recipe.iter().enumerate() {
                if r_needed == 0 {
                    continue;
                }

                if bots[r_type] == 0 {
                    // can't make this robot, no robots to
                    // get resource
                    wait_time = -1;
                    break;
                }

                let time =
                    ((r_needed - resources[r_type]) as f32 / bots[r_type] as f32).ceil() as i32;
                wait_time = wait_time.max(time);
            }

            if wait_time != -1 {
                let elapsed = wait_time + 1; // takes 1 more minute for robot to get ready
                let remaining_time = time - elapsed;
                let mut bots = bots;
                let mut resources = resources;

                for (rkind, bot_count) in bots.into_iter().enumerate() {
                    println!(
                        "** {rkind} {bot_count} {elapsed} {remaining_time} {:?}",
                        bots
                    );
                    resources[rkind] += bot_count * elapsed;
                }

                for (rkind, cost) in recipe.iter().enumerate() {
                    resources[rkind] -= cost;
                }

                bots[kind] += 1;

                queue.push_front(State {
                    bots,
                    resources,
                    time: remaining_time,
                });

                // println!("{queue:?}");
                // input!();
            }
        }
    }

    max
}
*/
