use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::read;

const DELTA1: i64 = 2;
const DELTA2: i64 = 1_000_000;

pub fn run() {
    let file = read!();
    let map = file.map(|line| line.chars().collect_vec()).collect_vec();

    assert_eq!(map.len(), map[0].len());
    let mut x_gaps = HashSet::new();
    let mut y_gaps = HashSet::new();

    for i in (0..map.len()).rev() {
        let row = &map[i];
        if is_empty(row) {
            y_gaps.insert(i);
        }
    }

    for i in (0..map.len()).rev() {
        let col = get_col(i, &map);
        if is_empty(&col) {
            x_gaps.insert(i);
        }
    }

    let points = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, &c)| {
                    if c == '#' {
                        Some((y as i64, x as i64))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut visited = HashMap::new();

    // go dir, and then count the number of empty whatevers
    // then multiply by 1 million
    for &(py, px) in &points {
        for &(oy, ox) in &points {
            if (py, px) == (oy, ox) {
                continue;
            }

            if visited.contains_key(&((py, px), (oy, ox)))
                || visited.contains_key(&((oy, ox), (py, px)))
            {
                continue;
            }

            let diry = (oy - py).signum();
            let dirx = (ox - px).signum();

            let mut y = py;
            let mut x = px;

            // for the different parts
            let mut dy1 = 0;
            let mut dy2 = 0;

            let mut dx1 = 0;
            let mut dx2 = 0;

            // base distance
            let mut bdx = 0;
            let mut bdy = 0;

            while y != oy {
                bdy += 1;

                if y_gaps.contains(&(y as usize)) {
                    dy1 += DELTA1 - 1; // don't count the extra above ^
                    dy2 += DELTA2 - 1;
                }

                y += diry;
            }

            while x != ox {
                bdx += 1;

                if x_gaps.contains(&(y as usize)) {
                    dx1 += DELTA1 - 1;
                    dx2 += DELTA2 - 1;
                }

                x += dirx;
            }

            visited.insert(
                ((py, px), (oy, ox)),
                (bdx + bdy + dy1 + dx1, bdx + bdy + dy2 + dx2),
            );
        }
    }

    let (part1, part2): (Vec<i64>, Vec<i64>) = visited.values().copied().unzip();

    println!("Part1: {}", part1.into_iter().sum::<i64>());
    println!("Part2: {}", part2.into_iter().sum::<i64>());
}

fn _dot(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}

fn get_col(x: usize, map: &Vec<Vec<char>>) -> Vec<char> {
    let mut out = Vec::new();
    for c in map {
        out.push(c[x]);
    }

    out
}

fn is_empty(row: &Vec<char>) -> bool {
    for &c in row {
        if c != '.' {
            return false;
        }
    }

    true
}
