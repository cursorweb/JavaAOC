use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::read;

pub fn run() {
    let (rules, orders) = read!(str).split_once("\n\n").unwrap();

    let rules_vec = rules
        .split("\n")
        .map(|x| {
            x.split("|")
                .map(|x| x.parse().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect_vec();

    let orders = orders
        .split("\n")
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for (prev, next) in rules_vec {
        rules.entry(next).or_default().push(prev);
    }

    let mut sum1 = 0;
    let mut sum2 = 0;
    for order in orders {
        if check_valid(&order, &rules) {
            sum1 += order[order.len() / 2];
        } else {
            let mut x = order;
            x.sort_by(|a, b| {
                // [next]: [prev...]
                if !rules.contains_key(&a) {
                    return Ordering::Equal;
                }
                if rules[a].contains(b) {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            });
            sum2 += x[x.len() / 2];
        }
    }

    println!("Part1: {sum1}");
    println!("Part2: {sum2}");
}

/// rules: [next]: [prev...]
fn check_valid(order: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    // go from end to current
    for i in 0..order.len() {
        let x = order[i];
        for j in i..order.len() {
            let y = order[j];
            if rules.contains_key(&x) && rules[&x].contains(&y) {
                return false;
            }
        }
    }

    true
}

// fn dfs_x(num: i32, prev: i32, rules: &HashMap<i32, Vec<i32>>) -> bool {
//     let mut stack = Vec::new();
//     let mut visited = HashSet::new();
//     stack.push(num);

//     loop {
//         let Some(top) = stack.pop() else {
//             break;
//         };

//         if !visited.contains(&top) {
//             visited.insert(top);

//             let Some(xs) = rules.get(&top) else { continue };
//             for &x in xs {
//                 if x == prev {
//                     return true;
//                 } else {
//                     stack.push(x);
//                 }
//             }
//         }
//     }

//     false
// }
