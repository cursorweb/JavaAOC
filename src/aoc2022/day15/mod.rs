use std::collections::{HashMap, HashSet};

use crate::read;

const DEBUG: bool = false;
const ROW: i32 = if DEBUG { 10 } else { 2_000_000 };
const MAXY: i32 = if DEBUG { 20 } else { 4_000_000 };

pub fn run() {
    let file = read!();

    // { [sensor]: beacon }
    let objects: HashMap<(i32, i32), (i32, i32)> = file
        .flat_map(|line| {
            let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();

            let sensor = sensor.replace("Sensor at ", "");
            let (sx, sy) = sensor.split_once(", ").unwrap();
            let sx: i32 = sx.split_once("=").unwrap().1.parse().unwrap();
            let sy: i32 = sy.split_once("=").unwrap().1.parse().unwrap();

            let (bx, by) = beacon.split_once(", ").unwrap();
            let bx: i32 = bx.split_once("=").unwrap().1.parse().unwrap();
            let by: i32 = by.split_once("=").unwrap().1.parse().unwrap();

            vec![((sy, sx), (by, bx))]
        })
        .collect();

    let range = find_ranges(ROW, &objects);

    // there *may* be a chance where the length isn't 1, and may possibly be 2
    // in that case, use this code:
    // let sum: i32 = new_ranges.into_iter().map(|(a, b)| b - a).sum();
    assert_eq!(range.len(), 1);
    let sum = range[0].1 - range[0].0;

    println!("Part1: {}", sum);

    for row in 0..=MAXY {
        let range = find_ranges(row, &objects);
        if range.len() == 2 {
            // x should be in the middle
            let x = ((range[0].1 + range[1].0) / 2) as i64;
            println!("Part2: {}", 4_000_000 * x + row as i64);
            break;
        }
    }
    // each one has an intersection ...
}

/// sens: (y, x), beac: (y, x)
fn find_ranges(row: i32, objects: &HashMap<(i32, i32), (i32, i32)>) -> Vec<(i32, i32)> {
    let mut ranges = Vec::new();
    for (&sens, &beac) in objects {
        if let Some(range) = row_used(row, sens, beac) {
            ranges.push(range);
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut new_ranges = Vec::new();
    for range in ranges {
        let mut found = false;

        for nrange in new_ranges.iter_mut() {
            if intersect(range, *nrange) {
                *nrange = merge(range, *nrange);
                found = true;
                break;
            }
        }

        if !found {
            new_ranges.push(range);
        }
    }

    new_ranges
}

/// finds the interval that a certain sensor covers up at a row
/// sens: (y, x), beac: (y, x)
/// returns (min, max) inclusive
fn row_used(row: i32, sens: (i32, i32), beac: (i32, i32)) -> Option<(i32, i32)> {
    // distance between sensor and beacon
    let dist = (sens.1 - beac.1).abs() + (sens.0 - beac.0).abs();
    // after each row, the 'radius' decreases by 1
    let drow = (sens.0 - row).abs();

    let sens_col = sens.1;

    let radius = (dist - drow).abs();
    if (sens.0 - row).abs() <= dist {
        Some((sens_col - radius, sens_col + radius))
    } else {
        None
    }
}

/// check if 2 ranges intersect
fn intersect(r1: (i32, i32), r2: (i32, i32)) -> bool {
    //   |----|
    // |---|
    // |--|
    //     |---|
    // range2 start - 1 > range1 end or range1 start - 1 > range2 end
    return !(r2.0 - 1 > r1.1 || r1.0 - 1 > r2.1);
}

/// merge 2 ranges into 1
fn merge(r1: (i32, i32), r2: (i32, i32)) -> (i32, i32) {
    let start = r1.0.min(r2.0);
    let end = r1.1.max(r2.1);

    (start, end)
}

// [sens]: beac
fn _dot(objects: &HashMap<(i32, i32), (i32, i32)>, used: &HashSet<(i32, i32)>) {
    let sensors: HashSet<(i32, i32)> = objects.keys().copied().collect();
    let beacons: HashSet<(i32, i32)> = objects.values().copied().collect();
    for y in -2..22 {
        for x in -2..25 {
            if sensors.contains(&(y, x)) {
                print!("S");
            } else if beacons.contains(&(y, x)) {
                print!("B");
            } else if used.contains(&(y, x)) {
                print!("#");
            } else if y == 11 && x == 14 {
                print!("0");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
