use std::collections::HashMap;

use itertools::Itertools;

use crate::{lcm, read};

pub fn run() {
    let file = read!(str);
    let (instr_str, rest) = file.split_once("\n\n").unwrap();

    let dirs: HashMap<String, (String, String)> = rest
        .split("\n")
        .map(|line| {
            let (node, next) = line.split_once(" = ").unwrap();
            (
                node.to_string(),
                next[1..next.len() - 1]
                    .split(", ")
                    .map(String::from)
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect();

    let mut instrs = instr_str.chars().cycle();
    let mut curr = "AAA";
    let mut count = 0;
    while let Some(instr) = instrs.next() {
        if curr == "ZZZ" {
            break;
        }

        count += 1;
        match instr {
            'L' => curr = &dirs[curr].0,
            'R' => curr = &dirs[curr].1,
            _ => unreachable!("{instr}"),
        }
    }

    println!("Part1: {count}");

    let mut instrs = instr_str.chars().cycle();

    // (name), (curr, num)
    let mut currs: HashMap<String, (String, i64)> = dirs
        .keys()
        .filter_map(|k| {
            if k.ends_with("A") {
                Some((k.clone(), (k.clone(), 0i64)))
            } else {
                None
            }
        })
        .collect();

    // it's a cycle!
    // count how many times it takes for each number to get there
    // then get least common multiple
    let mut finished = HashMap::new();

    while let Some(instr) = instrs.next() {
        if currs.is_empty() {
            break;
        }
        let mut new_currs = HashMap::new();
        for (key, val) in currs {
            if val.0.ends_with("Z") {
                finished.insert(key, val);
            } else {
                new_currs.insert(key, val);
            }
        }
        currs = new_currs;

        match instr {
            'L' => {
                let mut new_currs = HashMap::new();
                for (key, value) in currs {
                    new_currs.insert(key, (dirs[&value.0].0.clone(), value.1 + 1));
                }
                currs = new_currs;
            }
            'R' => {
                let mut new_currs = HashMap::new();
                for (key, value) in currs {
                    new_currs.insert(key, (dirs[&value.0].1.clone(), value.1 + 1));
                }
                currs = new_currs;
            }
            _ => unreachable!("{instr}"),
        }
    }

    println!("Part2: {}", finished.values().fold(1, |p, c| lcm(p, c.1)));
}
