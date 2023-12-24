use itertools::Itertools;

use crate::read;

const MIN: f64 = 200_000_000_000_000.0;
const MAX: f64 = 400_000_000_000_000.0;

// px py pz @ vx vy vz

pub fn run() {
    let file = read!();
    let vels: Vec<((_, _, _), (_, _, _))> = file
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();

            let pos = pos
                .split(", ")
                .map(|n| n.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();
            let vel = vel
                .split(", ")
                .map(|n| n.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();

            (pos, vel)
        })
        .collect_vec();

    let mut count = 0;

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

            // println!("MIN <= {x} == {}", MIN <= x);

            if MIN <= x && x <= MAX && MIN <= y && y <= MAX {
                // println!("{pos1:?} {pos2:?}");
                // println!("{x} {y}\n");
                count += 1;
            }
        }
    }

    println!("{}", count / 2);
}
