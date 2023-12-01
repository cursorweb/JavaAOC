use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let nums = "one, two, three, four, five, six, seven, eight, nine"
        .split(", ")
        .collect_vec();
    let (part1, part2): (Vec<i32>, Vec<i32>) = file
        .map(|line| {
            let chars = line.chars().collect_vec();
            let mut part1 = vec![];
            let mut part2 = vec![];

            for &char in &chars {
                if '0' <= char && char <= '9' {
                    part1.push(char as i32 - '0' as i32);
                }
            }

            for (i, &char) in chars.iter().enumerate() {
                if '0' <= char && char <= '9' {
                    part2.push(char as i32 - '0' as i32);
                }

                for j in 0..nums.len() {
                    if i + nums[j].len() <= line.len() && &line[i..i + nums[j].len()] == nums[j] {
                        part2.push((j + 1) as i32);
                    }
                }
            }

            (
                part1[0] * 10 + part1[part1.len() - 1],
                part2[0] * 10 + part2[part2.len() - 1],
            )
        })
        .unzip();
    println!("Part1: {}", part1.into_iter().sum::<i32>());
    println!("Part2: {}", part2.into_iter().sum::<i32>());
}
