use itertools::Itertools;

use crate::read;

pub fn run() {
    let file: Vec<Vec<i32>> = read!()
        .map(|x| x.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    let sum = file.iter().fold(0, |acc, l| {
        let incr = l[1] > l[0];

        for x in l.windows(2) {
            if x[1] > x[0] && !incr {
                return acc;
            }

            if x[1] < x[0] && incr {
                return acc;
            }

            if x[1] == x[0] {
                return acc;
            }

            if (x[1] - x[0]).abs() > 3 {
                return acc;
            }
        }

        acc + 1
    });

    println!("Part1: {sum}");

    let sum = file.iter().fold(0, |acc, l| {
        let result = l.iter().combinations(l.len() - 1).find(|l| {
            let incr = l[1] > l[0];

            for x in l.windows(2) {
                if x[1] > x[0] && !incr {
                    return false;
                }

                if x[1] < x[0] && incr {
                    return false;
                }

                if x[1] == x[0] {
                    return false;
                }

                if (x[1] - x[0]).abs() > 3 {
                    return false;
                }
            }

            true
        });

        if result.is_some() {
            acc + 1
        } else {
            acc
        }
    });

    println!("Part2: {sum}");
}
