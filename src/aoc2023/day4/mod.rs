use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let cards = file
        .map(|line| {
            let rest = line.split_once(": ").unwrap().1;
            let (winning, have) = rest.split_once(" | ").unwrap();
            let winning: HashSet<i32> = winning.split(" ").filter_map(|x| x.parse().ok()).collect();
            let have: Vec<i32> = have.split(" ").filter_map(|x| x.parse().ok()).collect_vec();

            (winning, have)
        })
        .collect_vec();

    let mut total = 0;

    for (winning, have) in &cards {
        let mut score: i32 = -1;
        for num in have {
            if winning.contains(num) {
                score += 1;
            }
        }

        if score > -1 {
            total += 2i64.pow(score as u32);
        }
    }

    println!("Part1: {total}");

    let mut cards_won = HashMap::new();

    for i in 0..cards.len() {
        cards_won.insert((i + 1) as i64, 1);
    }

    for (i, (winning, have)) in cards.iter().enumerate() {
        let i = i as i64;
        let mut score = 0;
        for num in have {
            if winning.contains(num) {
                score += 1;
            }
        }

        if score == 0 {
            continue;
        }

        let instances = cards_won[&(i + 1)];

        for j in i + 2..i + score + 2 {
            cards_won
                .entry(j)
                .and_modify(|count| {
                    *count += instances;
                })
                .or_insert(1 as i64);
        }
    }

    println!("Part2: {}", cards_won.values().sum::<i64>());
}
