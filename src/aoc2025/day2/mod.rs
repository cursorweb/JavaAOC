use itertools::Itertools;

use crate::{num_digits, read};

#[derive(Clone, Copy, Debug)]
struct IdRange {
    /// Actual range of ids that are valid
    valid: (i64, i64),
    /// Range of ids to check
    iter_range: (i64, i64),
}

pub fn run() {
    let file = read!(str).split(",");
    let ranges = file
        .map(|line| {
            line.split('-')
                .map(|x| x.parse().unwrap())
                .collect_tuple::<(i64, i64)>()
                .map(|x| {
                    let (min, max) = change_range(x);
                    IdRange {
                        valid: (x.0 as i64, x.1 as i64),
                        iter_range: (first_half(min), first_half(max)),
                    }
                })
                .unwrap()
        })
        .collect_vec();

    let total: i64 = ranges
        .iter()
        .map(
            |&IdRange {
                 valid: (min, max),
                 iter_range: (iter_start, iter_end),
             }| {
                let x = iter_start..=iter_end;

                x.map(|n| {
                    let digits = num_digits(n);
                    let id = 10i64.pow(digits) * n + n;
                    id
                })
                .filter(|&id| min <= id && id <= max)
                .sum::<i64>()
            },
        )
        .sum();

    println!("Part1: {total}");
}

/// Gets the iter range, which is where all ids could have invalid
fn change_range(x: (i64, i64)) -> (i64, i64) {
    let left = num_digits(x.0);
    let right = num_digits(x.1);

    let min = if left % 2 == 1 {
        10i64.pow(right - 1)
    } else {
        x.0 as i64
    };
    let max = if right % 2 == 1 {
        10i64.pow(left) - 1
    } else {
        x.1 as i64
    };

    (min, max)
}

fn first_half(x: i64) -> i64 {
    x / 10i64.pow(num_digits(x) / 2)
}
