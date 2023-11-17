use std::collections::HashMap;

use crate::read;

#[derive(Debug)]
struct Valve {
    flow: i32,
    tunnels: Vec<String>,
}

pub fn run() {
    let file = read!();
    let out: HashMap<String, Valve> = file
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

    println!("{out:?}");
}
