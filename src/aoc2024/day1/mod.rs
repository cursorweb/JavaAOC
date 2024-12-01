use crate::read;

pub fn run() {
    let file = read!();
    let file: Vec<(i32, i32)> = file
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = file.into_iter().unzip();

    left.sort();
    right.sort();

    let part1 = left
        .iter()
        .zip(&right)
        .fold(0, |acc, (l, r)| acc + (l - r).abs());

    println!("Part1: {part1}");

    let part2 = left.iter().fold(0, |acc, i| {
        let count = right.iter().filter(|&n| n == i).count();
        acc + i * count as i32
    });

    println!("Part2: {part2}");
}
