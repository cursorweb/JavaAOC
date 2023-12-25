use itertools::Itertools;

use crate::read;

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

    // Vi - Vr
}
