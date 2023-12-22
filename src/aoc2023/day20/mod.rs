use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::read;

/*
Flip-flop modules (prefix %): on/off = off.
    High pulse: nothing happens
    Low pulse: flip on/off.
        If off: it turns on, sends a high pulse.
        If on: it turns off, sends a low pulse.

Conjunction modules (prefix &):
    Previous: high/low = low.
    Update memory, then send remembered.

Button sends low pulse
*/

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
    /// Update memory, then send remembered
    Conjunction(HashMap<String, bool>),
}

// trait Module {
//     fn send(from: String, is_high: bool) -> Option<bool>;
// }

// impl Module for FlipFlop {
//     fn send(from: String, is_high: bool) -> Option<bool> {
//         todo!()
//     }
// }

// impl Module for Conjunction {
//     fn send(from: String, is_high: bool) -> Option<bool> {
//         todo!()
//     }
// }

use Module::*;

pub fn run() {
    let file = read!();

    let mut broadcast = Vec::new();

    let mut instrs: HashMap<String, (Module, Vec<String>)> = file
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

    /*
    let mut low = 0;
    let mut high = 0;

    low += 1; // button press
    for dest in broadcast {
        let (dl, dh) = send_pulse(&dest, &mut instrs, false);
        low += 1; // the actual broadcast itself
        low += dl;
        high += dh;
    }
    */

    let (low, high) = send_pulse(&broadcast, &mut instrs);

    println!("{low} * {high} = {}", low * high);
}

fn send_pulse(locs: &[String], map: &mut HashMap<String, (Module, Vec<String>)>) -> (i64, i64) {
    // long term stack, vs short term stack (short gets added back to long)
    let mut queue = VecDeque::new();

    let mut low = 0;
    let mut high = 0;

    for loc in locs {
        println!("broadcaster -false-> {loc}");
        queue.push_front((loc.clone(), false))
    }

    while let Some((loc, high_pulse)) = queue.pop_front() {
        if high_pulse {
            high += 1;
        } else {
            low += 1;
        }

        let Some(module) = map.get_mut(&loc) else {
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
                        println!("%{loc} -{state}-> {dest}");
                        queue.push_front((dest, *state));
                    }
                }
            }
            Conjunction(prev) => {
                /*
                Conjunction modules (prefix &):
                    Previous: high/low = low.
                    Update memory, then send remembered.
                */
                let entry = prev.entry(loc.clone()).or_insert(false);
                let prev = *entry;
                *entry = high_pulse;

                for dest in module.1.clone() {
                    println!("&{loc} -{prev}-> {dest}");
                    queue.push_front((dest, prev));
                }
            }
        }
    }

    // include the button press
    (low + 1, high)
}

/*
/// (low, high)
fn send_pulse(loc: &str, map: &mut HashMap<String, Module>, high_pulse: bool) -> (i64, i64) {
    let Some(module) = map.get_mut(loc) else {
        return (0, 0);
    };

    if module.is_flipflop {
        if !high_pulse {
            module.state = !module.state;
            let state = module.state;

            let mut low = 0;
            let mut high = 0;

            for dest in &module.dests.clone() {
                println!("{loc} -{state}-> {dest}");
                let (dl, dh) = send_pulse(dest, map, state);
                low += dl;
                high += dh;
            }

            if state {
                high += 1;
            } else {
                low += 1;
            }

            (low, high)
        } else {
            (0, 0)
        }
    } else {
        let prev = module.state;
        module.state = !module.state;

        let mut low = 0;
        let mut high = 0;

        for dest in &module.dests.clone() {
            println!("{loc} -{prev}-> {dest}");
            let (dl, dh) = send_pulse(dest, map, prev);
            low += dl;
            high += dh;
        }

        if prev {
            high += 1;
        } else {
            low += 1;
        }

        (low, high)
    }
}
*/
