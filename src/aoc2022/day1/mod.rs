use crate::read;

pub fn run() {
    let file = read!(str);
    let iter = file
        .split("\n\n")
        .map(|elf| elf.lines().map(|n| n.parse::<i32>().unwrap()).sum::<i32>());

    let part1 = iter.clone().max().unwrap();

    let mut part2 = iter.collect::<Vec<_>>();
    part2.sort();
    let part2 = part2.iter().rev().take(3).sum::<i32>();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
