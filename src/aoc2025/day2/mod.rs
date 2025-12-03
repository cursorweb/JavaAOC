use std::collections::HashSet;

use itertools::Itertools;

use crate::{num_digits, read};

pub fn run() {
    // let mut id = 0;
    // let x = 12;
    // let digits = num_digits(x);
    // for i in 0..3 {
    //     id += x * 10i32.pow(i * digits);
    // }
    // println!("{id}");
    // return;

    let file = read!(str).split(",");
    let ranges = file
        .map(|line| {
            line.split('-')
                .map(|x| x.parse().unwrap())
                .collect_tuple::<(i64, i64)>()
                .map(|x| split_range(x))
                .unwrap()
        })
        .flatten()
        .collect_vec();

    let part1: i64 = ranges
        .iter()
        .map(|&x| invalid_ids_repeat(x, 2).iter().sum::<i64>())
        .sum();

    let part2: i64 = ranges
        .iter()
        .map(|&x| {
            let digis = num_digits(x.0);

            let mut sum = HashSet::new();

            for size in 1..digis {
                if digis % size != 0 {
                    continue;
                }

                let repeats = digis / size;
                sum.extend(invalid_ids_repeat(x, repeats));
            }

            sum.iter().sum::<i64>()
        })
        .sum();

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

/// Gets the iter range, which is where all ids could have invalid
fn split_range(x: (i64, i64)) -> Vec<(i64, i64)> {
    let left = num_digits(x.0);
    let right = num_digits(x.1);

    if left == right {
        return vec![x];
    }

    let middle = 10i64.pow(left);

    vec![(x.0, middle - 1), (middle, x.1)]
}

fn first_n(x: i64, n: u32) -> i64 {
    x / 10i64.pow(num_digits(x) - n)
}

fn invalid_ids_repeat((min, max): (i64, i64), repeats: u32) -> HashSet<i64> {
    let size = num_digits(min) / repeats;
    let x = first_n(min, size)..=first_n(max, size);
    let max_num_digits = num_digits(min);

    x.filter_map(|n| {
        let mut id = 0;

        for i in 0..repeats {
            id += 10i64.pow(i * size) * n;
        }

        Some(id)
    })
    .filter(|&id| min <= id && id <= max)
    .collect()
}
