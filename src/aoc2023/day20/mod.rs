use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::read;

#[derive(Debug, Clone)]
enum Module {
    /// Flip-flop modules (%) state = off
    /// ```txt
    /// High pulse: nothing happens
    /// Low pulse:
    ///     If off: sends a high pulse.
    ///     If on: sends a low pulse.
    ///     Flip state.
    /// ```
    FlipFlop(bool),

    /// Conjuction modules (&):
    /// Stores type of most recent received from each of connected modules (default low)
    /// ```txt
    /// Update memory, then
    ///     if all high: low
    ///     else: high
    /// ```
    Conjunction(HashMap<String, bool>),
}

use Module::*;

pub fn run() {
    let file = read!();

    let mut broadcast = Vec::new();

    let old_instrs: HashMap<String, (Module, Vec<String>)> = file
        .filter_map(|line| {
            let (name, dest) = line.split_once(" -> ").unwrap();

            let dests = dest.split(", ").map(String::from).collect_vec();

            if name.starts_with("%") {
                Some(
                    /* */ (name[1..].to_string(), (FlipFlop(false), dests)), /* */
                )
            } else if name.starts_with("&") {
                Some(
                    (name[1..].to_string(), (Conjunction(HashMap::new()), dests)), /* */
                )
            } else {
                broadcast = dests;
                None
            }
        })
        .collect();

    // sets all the default values
    let mut instrs = old_instrs.clone();

    /*
    P2 Observations:
    rx <-- &yy <-- multiple &xx
    */

    // & sends a low when all are high
    // all the &xx should be high (so should receive at least one high)
    // let mut lows = 0;
    let mut yy = "";
    let mut xxs = HashMap::new();

    for (from, (_, dests)) in &old_instrs {
        for dest in dests {
            if dest == "rx" {
                // should only have one
                assert_eq!(yy, "");
                yy = from;
            }

            if let Some((Conjunction(map), _)) = instrs.get_mut(dest) {
                map.insert(from.clone(), false);
            }
        }
    }

    for (from, (_, dests)) in &old_instrs {
        for dest in dests {
            if dest == yy {
                xxs.insert(from.clone(), 0);
            }
        }
    }

    let mut total_low @ mut total_high = 0;

    for i in 1.. {
        let (low, high) = send_pulse(&broadcast, &mut instrs, &mut xxs, yy, i);
        total_low += low;
        total_high += high;

        if i == 1000 {
            println!("Part1: {}", total_low * total_high);
        }

        if xxs.values().all(|&x| x > 0) {
            println!("{:?}", xxs.values());
            break;
        }
    }
}

fn send_pulse(
    locs: &[String],
    map: &mut HashMap<String, (Module, Vec<String>)>,
    cycles: &mut HashMap<String, i64>,
    yy: &str,
    i: i64,
) -> (i64, i64) {
    let mut queue = VecDeque::new();

    let mut low = 0;
    let mut high = 0;

    for loc in locs {
        queue.push_back(("broadcaster".to_string(), loc.clone(), false));
    }

    while let Some((from, curr, high_pulse)) = queue.pop_front() {
        if high_pulse {
            high += 1;
        } else {
            low += 1;
        }

        if curr == yy && high_pulse {
            if cycles[&from] == 0 {
                cycles.insert(from.clone(), i);
            }
        }

        let Some(module) = map.get_mut(&curr) else {
            continue;
        };

        match &mut module.0 {
            FlipFlop(state) => {
                /*
                Flip-flop modules (prefix %): on/off = off.
                    High pulse: nothing happens
                    Low pulse: flip on/off.
                        If off: it turns on, sends a high pulse.
                        If on: it turns off, sends a low pulse.
                */
                if !high_pulse {
                    *state = !*state;

                    for dest in module.1.clone() {
                        queue.push_back((curr.clone(), dest.clone(), *state));
                    }
                }
            }
            Conjunction(mem) => {
                /*
                Conjunction modules (prefix &):
                    Previous: high/low = low.
                    Update memory, then
                        if all high: low
                        else: high
                */
                mem.insert(from, high_pulse);
                let val = !mem.values().all(|&v| v == true);

                for dest in module.1.clone() {
                    queue.push_back((curr.clone(), dest.clone(), val));
                }
            }
        }
    }

    // include the button press
    (low + 1, high)
}
