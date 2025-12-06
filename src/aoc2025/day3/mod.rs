use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let banks: Vec<Vec<_>> = file
        .map(|x| {
            x.chars()
                .map(|c| (c as i32 - '0' as i32) as i64)
                .collect_vec()
        })
        .collect_vec();

    let mut part1 = 0;

    for bank in &banks {
        let (first_digit, index) = get_max(&bank[..bank.len() - 1], 0);
        let second_digit = bank[index + 1..].iter().max().unwrap();

        part1 += 10 * first_digit + second_digit;
    }

    println!("Part1: {part1}");

    let mut part2 = 0;

    for bank in &banks {
        let mut start = 0;
        let mut end = bank.len() - 11;
        let mut number = 0;
        for i in 0..12 {
            let (digit, nindex) = get_max(&bank[start..end], start);
            start = nindex + 1;
            end += 1;
            number += 10i64.pow(11 - i) * digit;
        }

        part2 += number;
    }

    println!("Part2: {part2}");
}

/// Rust usually returns the LAST largest, so here we return the FIRST largest
fn get_max(bank: &[i64], offset: usize) -> (i64, usize) {
    let mut digit = 0;
    let mut index = 0;

    for (j, &value) in bank.iter().enumerate() {
        if value > digit {
            index = j;
            digit = bank[j];
        }
    }

    (digit, offset + index)
}
