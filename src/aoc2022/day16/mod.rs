/*
Most of the valves are broken
If we create a new map, that ignores the broken valves
And is weighted based on distance
Then we can use a DFS to get to next best node
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

    // println!("{dist:#?}");
    println!("Part1: {}", bfs(&dist, &valves));
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
fn bfs(dist: &HashMap<String, HashMap<String, i32>>, valves: &HashMap<String, Valve>) -> i32 {
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
            .and_modify(|current_best: &mut i32| {
                *current_best = state.released.max(*current_best);
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

    *results.values().max().unwrap()
}
