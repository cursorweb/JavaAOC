use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let old_map: HashMap<String, Vec<&str>> = file
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            (left.to_string(), right.split(" ").collect_vec())
        })
        .collect();

    let mut map = old_map.clone();

    for (k, v) in &old_map {
        for n in v {
            map.entry(n.to_string()).or_default().push(k);
        }
    }

    // println!("{map:?}");

    // answers: bmd -- ngp, grd -- tqr, dlv -- tqh

    let caucus_one = ["bmd", "grd", "dlv"];
    let caucus_two = ["ngp", "tqr", "tqh"];

    // let caucus_one = ["hfx", "bvb", "jqt"];
    // let caucus_two = ["pzl", "cmg", "nvd"];

    let mut size_one = HashSet::from(caucus_one);
    for name in caucus_one {
        if let Some(vals) = map.get(name) {
            let mut stack = vals.clone();
            while let Some(name) = stack.pop() {
                if caucus_two.contains(&name) {
                    continue;
                }

                size_one.insert(name);

                let Some(next) = map.get(name) else {
                    continue;
                };

                for x in next {
                    if !size_one.contains(x) {
                        stack.push(x);
                    }
                }
            }
        }
    }

    let mut size_two = HashSet::from(caucus_two);
    for name in caucus_two {
        if let Some(vals) = map.get(name) {
            let mut stack = vals.clone();
            while let Some(name) = stack.pop() {
                if caucus_one.contains(&name) {
                    continue;
                }

                size_two.insert(name);

                let Some(next) = map.get(name) else {
                    continue;
                };

                for x in next {
                    if !size_two.contains(x) {
                        stack.push(x);
                    }
                }
            }
        }
    }

    println!("{} {}", size_one.len(), size_two.len());
    println!("{}", size_one.len() * size_two.len());
}
