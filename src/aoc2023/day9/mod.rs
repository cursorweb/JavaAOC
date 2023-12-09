use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let nums: Vec<Vec<i64>> = file
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect_vec())
        .collect_vec();

    let nums = nums
        .into_iter()
        .map(|sequence| {
            let mut deriv = vec![sequence.clone(), diffs(&sequence)];
            while !all_zero(deriv.last().unwrap()) {
                deriv.push(diffs(deriv.last().unwrap()));
            }
            deriv
        })
        .collect_vec();

    let part1: i64 = nums
        .iter()
        .map(|deriv| deriv.iter().map(|x| x[x.len() - 1]).sum::<i64>())
        .sum();

    println!("Part1: {part1}");

    let part2: i64 = nums
        .iter()
        .map(|deriv| {
            // [0, 2, 0, 3, 10]
            //  ^p ^c
            deriv
                .into_iter()
                .rev()
                .map(|x| x[0])
                .reduce(|p, c| c - p)
                .unwrap()
        })
        .sum();

    println!("Part1: {part2}");
}

fn diffs(nums: &Vec<i64>) -> Vec<i64> {
    let mut out = Vec::new();
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        out.push(diff);
    }
    out
}

fn all_zero(nums: &Vec<i64>) -> bool {
    for &n in nums {
        if n != 0 {
            return false;
        }
    }
    true
}
