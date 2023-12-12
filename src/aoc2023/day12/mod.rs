use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let records: Vec<(Vec<char>, Vec<i32>)> = file
        .map(|line| {
            let (left, right) = line.split_once(" ").unwrap();
            (
                left.chars().collect(),
                right.split(",").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect_vec();

    println!("{records:?}");
}
