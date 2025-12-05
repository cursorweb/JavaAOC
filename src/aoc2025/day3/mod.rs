use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let banks: Vec<Vec<i32>> = file
        .map(|x| x.chars().map(|c| c as i32 - '0' as i32).collect_vec())
        .collect_vec();

    let mut sum = 0;

    for bank in &banks {
        let mut first_digit = 0;
        let mut index = 0;
        for j in 0..bank.len() - 1 {
            if bank[j] > first_digit {
                index = j;
                first_digit = bank[j];
            }
        }

        let second_digit = bank[index + 1..].iter().max().unwrap();

        sum += 10 * first_digit + second_digit;
    }

    println!("Part1: {sum}");
}
