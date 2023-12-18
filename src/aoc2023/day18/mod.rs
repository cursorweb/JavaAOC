use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{read, DIRS};

#[derive(Debug)]
enum Instr {
    Up(i64, String),
    Right(i64, String),
    Down(i64, String),
    Left(i64, String),
}

use Instr::*;

pub fn run() {
    let file = read!();

    let instrs = file
        .map(|line| {
            let mut instrs = line.split(" ");
            let dir = instrs.next().unwrap();
            let num = instrs.next().unwrap().parse().unwrap();
            let code = instrs.next().unwrap();
            // remove (#)
            let code = String::from(&code[2..code.len() - 1]);

            match dir {
                "U" => Up(num, code),
                "R" => Right(num, code),
                "D" => Down(num, code),
                "L" => Left(num, code),
                _ => unreachable!("{dir}"),
            }
        })
        .collect_vec();

    let mut pos = (0, 0);
    let mut dug = HashSet::new();

    for instr in &instrs {
        match instr {
            &Up(n, _) => {
                for _ in 0..n {
                    pos.0 -= 1;
                    dug.insert(pos);
                }
            }
            &Right(n, _) => {
                for _ in 0..n {
                    pos.1 += 1;
                    dug.insert(pos);
                }
            }
            &Down(n, _) => {
                for _ in 0..n {
                    pos.0 += 1;
                    dug.insert(pos);
                }
            }
            &Left(n, _) => {
                for _ in 0..n {
                    pos.1 -= 1;
                    dug.insert(pos);
                }
            }
        }
    }

    // let us assume (1, 1) is inside
    assert!(!dug.contains(&(1, 1)));
    println!("Part1: {}", fill(&dug) + dug.len() as i64);

    let instrs = instrs
        .into_iter()
        .map(|instr| {
            let hex = match instr {
                Up(_, v) => v,
                Right(_, v) => v,
                Down(_, v) => v,
                Left(_, v) => v,
            };

            let (rest, last) = hex.split_at(hex.len() - 1);
            let rest = i64::from_str_radix(rest, 16).unwrap();

            let s = String::new();

            // 0 means R, 1 means D, 2 means L, and 3 means U
            match last {
                "0" => Right(rest, s),
                "1" => Down(rest, s),
                "2" => Left(rest, s),
                "3" => Up(rest, s),
                _ => unreachable!("{last}"),
            }
        })
        .collect_vec();

    let mut points = Vec::new();
    let mut perimeter = 0;
    let mut pos = (0, 0);

    for instr in &instrs {
        match instr {
            &Up(n, _) => {
                pos.0 -= n;
                perimeter += n;
                points.push(pos);
            }
            &Right(n, _) => {
                pos.1 += n;
                perimeter += n;
                points.push(pos);
            }
            &Down(n, _) => {
                pos.0 += n;
                perimeter += n;
                points.push(pos);
            }
            &Left(n, _) => {
                pos.1 -= n;
                perimeter += n;
                points.push(pos);
            }
        }
    }
    let mut area = 0;

    for i in 1..=points.len() - 1 {
        let c1 = points[i - 1];
        let c2 = points[i];

        area += det(c1, c2);
    }

    // cool links:
    // shoelace: https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_form,_determinant_form
    // pick's: https://en.wikipedia.org/wiki/Pick's_theorem

    // have to divide area by 2 as 2A = ...
    // so we have: 2A / 2 + (P + 4) / 2 - 1
    // because we include 0 in the perimeter, the size is actually 1 bigger (so + 4)
    println!("Part2: {}", area / 2 + perimeter / 2 + 1);
}

fn fill(dug: &HashSet<(i32, i32)>) -> i64 {
    let start = (1, 1);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_front(start);

    while let Some(pos) = queue.pop_front() {
        if dug.contains(&pos) || visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        for dir in DIRS {
            let npos = (pos.0 + dir.0, pos.1 + dir.1);
            queue.push_front(npos);
        }
    }

    visited.len() as i64
}

// |col11 col21|
// |col10 col20|
fn det(col1: (i64, i64), col2: (i64, i64)) -> i64 {
    col1.1 * col2.0 - col2.1 * col1.0
}
