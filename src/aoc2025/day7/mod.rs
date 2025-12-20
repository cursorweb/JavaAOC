use std::collections::HashSet;

use itertools::Itertools;

use crate::read;

pub fn run() {
    let mut file = read!();

    let start = file.next().unwrap().chars().position(|c| c == 'S').unwrap();

    let rest = file.skip(1).step_by(2);

    let splitters = rest
        .map(|x| {
            x.char_indices()
                .filter_map(|(i, c)| (c == '^').then_some(i))
                .collect::<HashSet<usize>>()
        })
        .collect_vec();

    let mut beams = HashSet::from([start]);

    let mut splits = 0;

    for row in splitters {
        let mut next = HashSet::new();

        for beam in beams {
            if row.contains(&beam) {
                splits += 1;
                next.insert(beam - 1);
                next.insert(beam + 1);
            } else {
                next.insert(beam);
            }
        }

        beams = next;
    }

    println!("Part1: {splits}");
    println!("Part2: {}", (beams.len() + 1) * (splits - 1));
}
