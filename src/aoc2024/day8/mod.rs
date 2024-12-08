use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();

    let mut points: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in file.enumerate() {
        height += 1;
        width = line.len() as i32;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                points.entry(c).or_default().push((y as i32, x as i32));
            }
        }
    }

    // make const
    let width = width;
    let height = height;
    let points = points;

    let mut antinodes = HashSet::new(); // p1
    let mut resinodes = HashSet::new(); // p2

    for list in points.values() {
        for point in list.iter().combinations(2) {
            let (p0y, p0x) = *point[0];
            let (p1y, p1x) = *point[1];

            let dx = p1x - p0x;
            let dy = p1y - p0y;

            let an1 = (p0y - dy, p0x - dx);
            let an2 = (p1y + dy, p1x + dx);

            if in_range(an1, width, height) {
                antinodes.insert(an1);
            }

            if in_range(an2, width, height) {
                antinodes.insert(an2);
            }

            let (p0y, p0x) = *point[0];
            let (p1y, p1x) = *point[1];

            let dx = p1x - p0x;
            let dy = p1y - p0y;

            let mut an1 = (p0y, p0x);
            let mut an2 = (p1y, p1x);

            resinodes.insert(an1);
            resinodes.insert(an2);

            loop {
                an1 = (an1.0 - dy, an1.1 - dx);
                if !in_range(an1, width, height) {
                    break;
                }

                resinodes.insert(an1);
            }

            loop {
                an2 = (an2.0 + dy, an2.1 + dx);
                if !in_range(an2, width, height) {
                    break;
                }

                resinodes.insert(an2);
            }
        }
    }

    println!("Part1: {}", antinodes.len());
    println!("Part2: {}", resinodes.len());
}

fn in_range((y, x): (i32, i32), width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}
