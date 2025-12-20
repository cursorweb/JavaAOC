use std::collections::HashSet;

use itertools::Itertools;

use crate::read;

pub fn run() {
    let mut file = read!();

    let start = file.next().unwrap().chars().position(|c| c == 'S').unwrap();

    let width = file.next().unwrap().len();

    let rest = file.step_by(2);

    let splitters = rest
        .map(|x| {
            x.char_indices()
                .filter_map(|(i, c)| (c == '^').then_some(i))
                .collect::<HashSet<usize>>()
        })
        .collect_vec();

    let mut beams = HashSet::from([start]);
    let mut beams_dup = vec![0; width];
    beams_dup[start] = 1;

    let mut splits = 0;

    for row in splitters {
        let mut next = HashSet::new();
        let mut next_dup = vec![0; width];

        for beam in beams {
            if row.contains(&beam) {
                splits += 1;
                next.insert(beam - 1);
                next.insert(beam + 1);
            } else {
                next.insert(beam);
            }
        }

        for (beam, &count) in beams_dup.iter().enumerate() {
            if row.contains(&beam) {
                next_dup[beam - 1] += count;
                next_dup[beam + 1] += count;
            } else {
                next_dup[beam] += count;
            }
        }

        // println!("{next_dup:?}");
        // for i in 0..next_dup.len() {
        //     if next_dup[i] != 0 {
        //         assert!(!row.contains(&i));
        //         print!("({})", next_dup[i]);
        //     } else if row.contains(&i) {
        //         print!("^");
        //     } else {
        //         print!(".");
        //     }
        // }
        // println!();
        // input!();

        beams = next;
        beams_dup = next_dup;
    }

    println!("Part1: {splits}");
    println!("Part2: {}", beams_dup.iter().sum::<i64>());
}
