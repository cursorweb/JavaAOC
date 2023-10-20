use std::collections::HashSet;

use crate::{name_to_dirs, read};

pub fn run() {
    let mut head = (0, 0);
    let mut tail = [(0, 0); 9];
    let mut part1visited = HashSet::new();
    let mut part2visited = HashSet::new();
    part1visited.insert(tail[0]);
    part2visited.insert(tail[8]);

    let file = read!();
    for ((dx, dy), num) in file.map(|line| {
        let mut instr = line.split(" ");
        let text = instr.next().unwrap();
        let num: i32 = instr.next().unwrap().parse().unwrap();

        let dir = name_to_dirs(text);
        (dir, num)
    }) {
        for _ in 0..num {
            head.0 += dx;
            head.1 += dy;

            for i in 0..9 {
                let head = if i == 0 { head } else { tail[i - 1] };
                let tail = &mut tail[i];

                if dist(tail.0, head.0) > 1 || dist(tail.1, head.1) > 1 {
                    tail.0 += sign(tail.0, head.0);
                    tail.1 += sign(tail.1, head.1);

                    if i == 0 {
                        part1visited.insert(*tail);
                    } else if i == 8 {
                        part2visited.insert(*tail);
                    }
                }
            }
        }
    }

    println!("Part1: {}", part1visited.len());
    println!("Part2: {}", part2visited.len());
}

fn dist(t: i32, h: i32) -> i32 {
    (t - h).abs()
}

fn sign(t: i32, h: i32) -> i32 {
    (h - t).signum()
}

/*
fn dot(h: (i32, i32), t: (i32, i32)) {
    let mut string = String::new();
    for y in 0..6 {
        let mut s = String::new();

        for x in 0..6 {
            if h.1 == y && h.0 == x {
                s += "H";
            } else if t.1 == y && t.0 == x {
                s += "T";
            } else {
                s += ".";
            }
        }

        string = s + "\n" + &string;
    }

    println!("{string}\n");
}
*/
