#![allow(unused)]
use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use crate::{read, gcd};

const MIN: f64 = 200_000_000_000_000.0;
const MAX: f64 = 400_000_000_000_000.0;

// px py pz @ vx vy vz

pub fn run() {
    let file = read!();
    let vels = file
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();

            let pos: (f64, f64, f64) = pos
                .split(", ")
                .map(|n| n.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();
            let vel: (f64, f64, f64) = vel
                .split(", ")
                .map(|n| n.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();

            // normalize
            let vel_mag = 1f64;
            // let vel_mag = f64::sqrt(vel.0 * vel.0 + vel.1 * vel.1 + vel.2 * vel.2);

            (pos, (vel.0 / vel_mag, vel.1 / vel_mag, vel.2 / vel_mag))
        })
        .collect_vec();

    let mut count = 0;

    /*
    sys of equations:
    x1 + m1 * t = x2 + r1 * s
    y1 + m2 * t = y2 + r2 * s

    m1, m2 are vels of (x1, y1)
    r1, r2 are vels of (x2, y2)

    t, s = time
    */
    for &(pos1, vel1) in &vels {
        for &(pos2, vel2) in &vels {
            if pos1 == pos2 && vel1 == vel2 {
                continue;
            }

            let (x1, y1, _) = pos1;
            let (m1, m2, _) = vel1;

            let (x2, y2, _) = pos2;
            let (r1, r2, _) = vel2;

            let r = r1 / r2;
            let t = (x1 - x2 - r * y1 + r * y2) / (r * m2 - m1);
            let s = (y1 + m2 * t - y2) / r2;

            // can't go back to the past
            if t < 0f64 || s < 0f64 {
                continue;
            }

            let (x, y) = (x1 + m1 * t, y1 + m2 * t);

            if MIN <= x && x <= MAX && MIN <= y && y <= MAX {
                count += 1;
            }
        }
    }

    // double counting
    println!("Part1: {}", count / 2);

    let mut theset = HashMap::new();

    for &(pos, (vx, vy, vz)) in &vels {
        for &(pos2, (vx1, vy1, vz1)) in &vels {
            if pos == pos2 {
                continue;
            }

            if vx == vx1 {
                let entry = theset.entry(vx as i64).or_insert(HashSet::new());
                entry.insert(pos.0 as i64);
                entry.insert(pos2.0 as i64);
                // theset.insert(vx as i64);
            }

            // if vy == 61.0 {
            //     let diff = (pos.0 - pos2.1).abs();
            //     println!("dx -- {} {:?}", diff, factor(diff));
            //     break;
            // }

            // vx = -177 -176 125, 128

            // if vx == 123.0 {
            //     let diff = (pos.0 - pos2.0).abs();
            //     println!("dx -- {} {:?}", diff, factor(diff));
            // }
        }
    }

    theset = theset.into_iter().filter(|(_, v)| v.len() >= 3).collect();

    // for (key, val) in theset {
    //     let mut rocks = val.into_iter();
    //     let (r1, r2, r3) = (rocks.next().unwrap(), rocks.next().unwrap(), rocks.next().unwrap());
    //     let dx = gcd(r2 - r1, r3 - r1);
        
    // }

    println!("{:?}", theset.keys());
}
