use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!()
        .map(|line| {
            let o: (i64, i64) = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();

            o
        })
        .collect_vec();

    let mut part1 = 0;

    for i in (0..file.len()).rev() {
        let (x1, y1) = file[i];
        for j in 0..i {
            let (x2, y2) = file[j];

            let area = (x2 - x1 + 1).abs() * (y2 - y1 + 1).abs();
            if area > part1 {
                part1 = area;
            }
        }
    }

    println!("Part1: {part1}");
}
