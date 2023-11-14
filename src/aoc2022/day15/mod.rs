#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::read;

#[derive(Debug)]
enum Pos {
    /// The closest beacon
    Sens((i32, i32)),
    Beac,
}

use Pos::*;

pub fn run() {
    let file = read!();

    // let objects: HashMap<(i32, i32), Pos> = file
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
            // vec![((sy, sx), Sens((by, bx))), ((by, bx), Beac)]
        })
        .collect();

    // number of unavailable on row {row}
    let mut cols = HashSet::new();
    let row = 2000000;

    for (sens, beac) in objects {
        find_used(row, sens, beac, &mut cols);
    }

    println!("{}", cols.len());
    // dot(&objects);
}

/// sens: (y, x), beac: (y, x)
fn find_used(row: i32, sens: (i32, i32), beac: (i32, i32), cols: &mut HashSet<i32>) {
    // -- println!("Reach: ({row}, {}) == {beac:?}", sens.1 - i);

    // this is the distance between the sensor and the beacon
    let dist = (sens.1 - beac.1).abs() + (sens.0 - beac.0).abs();
    // after each row, the 'radius' decreases by 1
    let drow = (sens.0 - row).abs();

    // sensor column
    let scol = sens.1;

    // the radius should be inclusive (this is a CLOSED ball!!)
    let radius = (dist - drow).abs();

    if (sens.0 - row).abs() <= dist {
        for i in 0..=radius {
            // don't count beacons
            if (row, sens.1 + i) != beac {
                cols.insert(sens.1 + i);
            }

            if i > 0 && (row, sens.1 - i) != beac {
                // this way we don't have to insert 0 twice
                cols.insert(sens.1 - i);
            }
        }
    }
}

fn dot(objects: &HashMap<(i32, i32), Pos>) {
    for y in -2..22 {
        for x in -2..25 {
            let res = objects.get(&(y, x));
            if let Some(Sens(_)) = res {
                print!("S");
            } else if let Some(Beac) = res {
                print!("B");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
