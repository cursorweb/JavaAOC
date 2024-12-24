use std::collections::HashMap;

use itertools::Itertools;

use crate::read;

/*
0 -> 1.
even digits -> first half, second half
else -> x * 2024.
*/

// moral of the story: ALWAYS used checked additions for things like this
// yucky yucky rusty

pub fn run() {
    let file = read!(str)
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let nums = file;
    let mut cache = HashMap::new();

    let part1: i64 = nums.iter().map(|x| count_times(*x, 25, &mut cache)).sum();
    println!("Part1: {}", part1);

    let part2: i64 = nums.iter().map(|x| count_times(*x, 75, &mut cache)).sum();
    println!("Part2: {}", part2);
}

fn count_times(num: i64, it: i32, cache: &mut HashMap<(i64, i32), i64>) -> i64 {
    if it == 0 {
        return 1;
    }

    if let Some(num) = cache.get(&(num, it)) {
        return *num;
    }

    let out = if num == 0 {
        count_times(1, it - 1, cache)
    } else if count_digits(num) % 2 == 0 {
        let (left, right) = split_num(num);
        count_times(left, it - 1, cache) + count_times(right, it - 1, cache)
    } else {
        count_times(num * 2024, it - 1, cache)
    };

    cache.insert((num, it), out);
    out
}

fn count_digits(num: i64) -> u32 {
    num.ilog10() + 1u32
}

fn split_num(num: i64) -> (i64, i64) {
    let digits = count_digits(num) / 2;
    let left = num as u64 / 10u64.pow(digits);
    let right = num as u64 % 10u64.pow(digits);

    (left as i64, right as i64)
}
