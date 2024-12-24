use itertools::Itertools;

use crate::read;

/*
0 -> 1.
even digits -> first half, second half
else -> x * 2024.
*/

pub fn run() {
    let file = read!(str)
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();

    let mut nums = file;

    println!("{:?}", split_num(55));

    for i in 0..nums.len() {
        let num = nums[i];
        if num == 0 {
            nums[i] = 1;
        } else if count_digits(num) % 2 == 0 {
        }
    }
}

pub fn count_digits(num: i32) -> u32 {
    (num.ilog10() + 1u32)
}

pub fn split_num(num: i32) -> (i32, i32) {
    let digits = count_digits(num) / 2;
    let left = num / 10u32.pow(digits);
    let right = num % 10iu32.pow(digits);

    (left, right)
}
