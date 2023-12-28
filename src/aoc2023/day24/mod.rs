use std::collections::HashMap;

use itertools::Itertools;

use crate::read;

const MIN: f64 = 200_000_000_000_000.0;
const MAX: f64 = 400_000_000_000_000.0;

const SEARCH_SPACE: i64 = 1000;

// type F = fraction::GenericFraction<i128>;

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

            (pos, (vel.0, vel.1, vel.2))
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
    for i in 1..vels.len() {
        let (pos1, vel1) = vels[i];
        for j in 0..i {
            let (pos2, vel2) = vels[j];

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

    println!("Part1: {}", count);

    /*
    x1 + t1 * (v - vr) = x2 + t2 * (v - vr)
    (x2 - x1) = (t2 - t1) * (v - vr)

    Therefore, (v - vr) is a divisor of +-(x2 - x1)

    all considered (x2 - x1) % (v - vr) == 0
    */

    let vels = vels
        .into_iter()
        .map(|((a, b, c), (d, e, f))| {
            (
                (a as i64, b as i64, c as i64),
                (d as i64, e as i64, f as i64),
            )
        })
        .collect_vec();

    let mut possible_x_vels = HashMap::new();

    let mut possible_y_vels = HashMap::new();

    let mut possible_z_vels = HashMap::new();

    for i in 1..vels.len() {
        let (pos1, vel1) = vels[i];
        for j in 0..i {
            let (pos2, vel2) = vels[j];

            if vel1.0 == vel2.0 {
                // (x2 - x1)
                let dist = pos2.0 - pos1.0;
                for r_x_vel in -SEARCH_SPACE..=SEARCH_SPACE {
                    // can't be the same velocity, (division by 0)
                    if r_x_vel == vel1.0 {
                        continue;
                    }

                    if dist % (vel1.0 - r_x_vel) == 0 {
                        *possible_x_vels.entry(r_x_vel).or_insert(0) += 1;
                    }
                }
            }

            if vel1.1 == vel2.1 {
                let dist = pos2.1 - pos1.1;
                for r_y_vel in -SEARCH_SPACE..=SEARCH_SPACE {
                    if r_y_vel == vel1.1 {
                        continue;
                    }

                    if dist % (vel1.1 - r_y_vel) == 0 {
                        *possible_y_vels.entry(r_y_vel).or_insert(0) += 1;
                    }
                }
            }

            if vel1.2 == vel2.2 {
                let dist = pos2.2 - pos1.2;
                for r_z_vel in -SEARCH_SPACE..=SEARCH_SPACE {
                    if r_z_vel == vel1.2 {
                        continue;
                    }

                    if dist % (vel1.2 - r_z_vel) == 0 {
                        *possible_z_vels.entry(r_z_vel).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    // classic works for input but not for example D:

    // (-3, 1, 2)
    let x_vel = possible_x_vels
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _)| k)
        .unwrap();
    let y_vel = possible_y_vels
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _)| k)
        .unwrap();
    let z_vel = possible_z_vels
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _)| k)
        .unwrap();

    let (x, y) = {
        // you can choose any, but vels[0].0 == r_vel for me so ... D:
        let ((x1, y1, _), (vx1, vy1, _)) = vels[1];
        let ((x2, y2, _), (vx2, vy2, _)) = vels[2];

        intersect(
            (x1, y1),
            (vx1 - x_vel, vy1 - y_vel),
            (x2, y2),
            (vx2 - x_vel, vy2 - y_vel),
        )
    };

    let (y1, z) = {
        // you can choose any, but vels[0].0 == r_vel for me so ... D:
        let ((_, y1, z1), (_, vy1, vz1)) = vels[1];
        let ((_, y2, z2), (_, vy2, vz2)) = vels[2];

        intersect(
            (y1, z1),
            (vy1 - y_vel, vz1 - z_vel),
            (y2, z2),
            (vy2 - y_vel, vz2 - z_vel),
        )
    };

    assert_eq!(y, y1);

    println!("Part2: {}", x + y + z);
}

fn intersect(pos1: (i64, i64), vel1: (i64, i64), pos2: (i64, i64), vel2: (i64, i64)) -> (i64, i64) {
    // cramer's rule
    /*
    Dt1 / D, etc...
    */
    let (m1, m2) = vel1;
    let (r1, r2) = vel2;

    let (x1, y1) = pos1;
    let (x2, y2) = pos2;

    let det = (m1 * -r2) - (m2 * -r1);
    let dett1 = ((x2 - x1) * -r2) - (-r1 * (y2 - y1));
    let dett2 = (m1 * (y2 - y1)) - ((x2 - x1) * m2);

    if det == 0 {
        panic!("can't solve :(");
    }

    let t = dett1 / det;
    let t2 = dett2 / det;

    if !(t > 0 && t2 > 0) {
        panic!("not in future");
    }

    (x1 + t * m1, y1 + t * m2)
}
