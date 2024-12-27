use itertools::Itertools;

use crate::read;

/// Points are in the form
/// `(x, y)`
#[derive(Debug, Clone, Copy)]
struct Mech {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

const SKIBIDI_CONSTANT: f64 = 10_000_000_000_000.0;

pub fn run() {
    let file = read!(str).split("\n\n");

    let mechs = file
        .map(|t| {
            let mut lines = t.split("\n");

            // btna == Button A: X+94, Y+34
            // 94, Y+34
            let btna_str = lines.next().unwrap().split_once(": X+").unwrap().1;
            let (btna_left, btna_right) = btna_str.split_once(", Y+").unwrap();
            let a = (btna_left.parse().unwrap(), btna_right.parse().unwrap());

            let btnb_str = lines.next().unwrap().split_once(": X+").unwrap().1;
            let (btnb_left, btnb_right) = btnb_str.split_once(", Y+").unwrap();
            let b = (btnb_left.parse().unwrap(), btnb_right.parse().unwrap());

            // prize == Prize: X=8400, Y=5400
            // 8400, Y=5400
            let prize = lines.next().unwrap().split_once(": X=").unwrap().1;
            let (pl, pr) = prize.split_once(", Y=").unwrap();
            let prize = (pl.parse().unwrap(), pr.parse().unwrap());

            Mech { a, b, prize }
        })
        .collect_vec();

    // button A == 3
    // button B == 1
    let (part1, part2) = mechs.iter().fold((0, 0), |(sum1, sum2), mech| {
        (
            sum1 + if let Some((a, b)) = solve_mech(mech) {
                3 * a + b
            } else {
                0
            },
            sum2 + if let Some((a, b)) = solve_mech2(mech) {
                3 * a + b
            } else {
                0
            },
        )
    });

    println!("Part1: {part1}");
    println!("Part2: {}", part2);
}

/*
Equation:
b = (tx - (x1 / y1) * ty) / (x2 - (x1 * y2) / y1)
a = (ty - y2 * b) / (y1)
*/
fn solve_mech(mech: &Mech) -> Option<(i32, i32)> {
    let tx = mech.prize.0 as f32;
    let ty = mech.prize.1 as f32;

    let x1 = mech.a.0 as f32;
    let y1 = mech.a.1 as f32;

    let x2 = mech.b.0 as f32;
    let y2 = mech.b.1 as f32;

    let b = ((tx - (x1 / y1) * ty) / (x2 - (x1 * y2) / y1)).round();
    let a = ((ty - y2 * b) / (y1)).round();

    let check = a * x1 + b * x2 == tx && a * y1 + b * y2 == ty;

    if a > 100.0 || b > 100.0 || !check {
        return None;
    }

    Some((a as i32, b as i32))
}

fn solve_mech2(mech: &Mech) -> Option<(i64, i64)> {
    let tx = SKIBIDI_CONSTANT + mech.prize.0 as f64;
    let ty = SKIBIDI_CONSTANT + mech.prize.1 as f64;

    let x1 = mech.a.0 as f64;
    let y1 = mech.a.1 as f64;

    let x2 = mech.b.0 as f64;
    let y2 = mech.b.1 as f64;

    let b = ((tx - (x1 / y1) * ty) / (x2 - (x1 * y2) / y1)).round();
    let a = ((ty - y2 * b) / (y1)).round();

    let check = a * x1 + b * x2 == tx && a * y1 + b * y2 == ty;

    if !check {
        return None;
    }

    Some((a as i64, b as i64))
}
