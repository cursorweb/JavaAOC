use std::collections::HashSet;

use crate::read;

pub fn run() {
    let file = read!();

    let part1: i32 = file
        .map(|line| {
            let half = line.len() / 2;
            let left: HashSet<_> = line[..half].chars().collect();
            let right: HashSet<_> = line[half..].chars().collect();

            priority(left.intersection(&right).next().unwrap())
        })
        .sum();

    let lines: Vec<_> = read!().collect();

    let mut part2 = 0;
    for i in (0..lines.len()).step_by(3) {
        let slice = lines[i..i + 3].iter();
        
        // convert to sets
        let mut sets = slice.map(|line| line.chars().collect::<HashSet<_>>());

        // find intersection
        let set = sets.next().map(|set| {
            sets.fold(set, |prev, curr| {
                prev.intersection(&curr).cloned().collect()
            })
        }).unwrap();

        part2 += priority(set.iter().next().unwrap());
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

fn priority(&c: &char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => unreachable!(),
    }
}
