/*
Observation: Most of the valves are broken

If we create a new graph, that ignores the broken valves
    and is weighted based on distance

Then, we can do a BF 'loop' to get a hash map of
    the max pressure released after x number of valves opened

And use this map to calculate the most number of pressure released possible
*/
use std::collections::{HashMap, HashSet, VecDeque};

use crate::read;

#[derive(Debug)]
struct Valve {
    flow: i32,
    tunnels: Vec<String>,
}

pub fn run() {
    let file = read!();
    let valves: HashMap<String, Valve> = file
        .map(|line| {
            let (valve, tunnels) = line.split_once("; ").unwrap();
            let (valve_label, flow_rate) = valve.split_once(" has flow rate=").unwrap();
            let valve_name = valve_label.replace("Valve ", "");
            let flow: i32 = flow_rate.parse().unwrap();

            let tunnels: Vec<String> = tunnels
                .split(" ") // get words
                .skip(4) // skip ~"tunnels" ~"lead" "to" ~"valves" (4)
                .collect::<String>()
                .split(",")
                .map(|x| x.to_string())
                .collect();

            (valve_name, Valve { flow, tunnels })
        })
        .collect();

    /*
    {
        "a": { "b": dist, "c": dist },
        "b": { ... }
    }
    */
    let mut dist: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for (name, valve) in &valves {
        if valve.flow == 0 && name != "AA" {
            // skip empty valves
            // except for the starting valve
            continue;
        }

        dist.insert(name.to_string(), dists(&name, &valves));
    }

    println!(
        "Part1: {}",
        bfs(&dist, &valves).values().max().copied().unwrap()
    );

    // since we already have all the paths
    // just choose the top 2 that don't intersect

    // not even my code anymore :(
    let mut nonempty = Vec::new();
    for (name, valve) in &valves {
        if valve.flow > 0 {
            nonempty.push(name.as_str());
        }
    }

    let nonemptylen = nonempty.len();

    let indices: HashMap<String, usize> = nonempty
        .iter()
        .enumerate()
        .map(|(i, n)| (n.to_string(), i))
        .collect();

    let mut cache = HashMap::new();

    // all tunnels: 111 ...
    let all = (1 << nonemptylen) - 1;

    let mut max = 0;

    // 0001, 00010, 000011 ... to 10000
    // highest + 1 / 2 == 100 ...
    for i in 0..((all + 1) / 2) {
        // ^ means exclusive or
        // which is basically set subtraction
        // 11111 ^
        // 00101 =
        // 11010
        max = max.max(
            dfs(26, "AA", i, &valves, &dist, &indices, &mut cache)
                + dfs(26, "AA", all ^ i, &valves, &dist, &indices, &mut cache),
        )
    }

    println!("Part2: {max}");
}

/// shortest dist to every other nonbroken valve
fn dists(from: &str, valves: &HashMap<String, Valve>) -> HashMap<String, i32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let mut out = HashMap::new();

    visited.insert(from);

    // (name, dist)
    queue.push_back((from, 0));

    while let Some((name, dist)) = queue.pop_front() {
        let valve = &valves[name];

        if valve.flow != 0 && name != from {
            // everyone is a goal!
            // the only criteria is you aren't a 0 (in distance or flow)
            out.insert(name.to_string(), dist);
        }

        for tunnel in &valve.tunnels {
            if !visited.contains(tunnel.as_str()) {
                visited.insert(&tunnel);
                queue.push_back((tunnel, dist + 1));
            }
        }
    }

    out
}

struct State {
    pos: String,
    time: i32,
    opened: HashSet<String>,
    released: i32,
}

/// go through all possible paths
/// and then, if the pressure released is better than for number of valves opened
/// record it
fn bfs(
    dist: &HashMap<String, HashMap<String, i32>>,
    valves: &HashMap<String, Valve>,
) -> HashMap<usize, i32> {
    // [open valves]: pressure
    let mut results = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(State {
        pos: "AA".into(),
        time: 30,
        opened: HashSet::new(),
        released: 0,
    });

    while let Some(state) = queue.pop_front() {
        // everyone is a winner!
        // as long as set biggest released for each # of steps
        results
            .entry(state.opened.len())
            .and_modify(|curr: &mut i32| {
                *curr = state.released.max(*curr);
            })
            .or_insert(state.released);

        let tunnels = &dist[&state.pos];
        for (next_pos, dist) in tunnels {
            if state.opened.contains(next_pos) {
                continue;
            }

            let new_time = state.time - dist - 1; // 1 minute to open valve
            if new_time >= 0 {
                let released = valves[next_pos].flow * new_time;
                let mut opened = state.opened.clone();
                opened.insert(next_pos.to_string());

                queue.push_back(State {
                    pos: next_pos.to_string(),
                    time: new_time,
                    opened,
                    released: state.released + released,
                });
            }
        }
    }

    results
}

/// Using bits, we can treat them as `HashSet`'s.
/// Use | to append:
/// ```txt
/// 000000 |
/// 000010 =
/// 000010
/// ```
/// Use & to check intersection:
/// ```txt
/// 000100 &
/// 001000 =
/// 000000 (doesn't intersect)
///
/// 001001 &
/// 010101 =
/// 000001 (intersects)
/// ```
///
/// The cache is:
/// `[(time, valve, opened)] = max pressure`
fn dfs(
    time: i32,
    name: &str,
    opened: i32,
    valves: &HashMap<String, Valve>,
    dists: &HashMap<String, HashMap<String, i32>>,
    indices: &HashMap<String, usize>,
    cache: &mut HashMap<(i32, String, i32), i32>,
) -> i32 {
    if let Some(&pressure) = cache.get(&(time, name.to_string(), opened)) {
        pressure
    } else {
        let mut max = 0;

        for (next, dist) in &dists[name] {
            let pressure = valves[next].flow;
            let remtime = time - dist - 1;
            let bit = 1 << indices[next];
            if opened & bit != 0 || remtime <= 0 {
                continue;
            }

            max = max.max(
                dfs(remtime, &next, opened | bit, valves, dists, indices, cache)
                    + pressure * remtime,
            );
        }

        cache.insert((time, name.to_string(), opened), max);

        max
    }
}
