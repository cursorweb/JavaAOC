use std::collections::{HashSet, HashMap};

use itertools::Itertools;

use crate::{read};

const DELTA: i64 = 1_000_000;

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

    /*

    let mut new_map = map.clone();
    for i in (0..map.len()).rev() {
        let row = &map[i];
        if is_empty(row) {
            new_map.insert(i, row.clone());
        }
    }

    // go through the x (which is still map.len)
    for i in (0..map.len()).rev() {
        let col = get_col(i, &map);
        if is_empty(&col) {
            for y in 0..new_map.len() {
                new_map[y].insert(i, '.');
            }
        }
    }
    */

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

    // let mut dist1 = 0;

        let mut visited = HashMap::new();

    // go dir, and then count the number of empty whatevers
    // then multiply by 1 million
    for &(py, px) in &points {
        for &(oy, ox) in &points {
            if (py, px) == (oy, ox) {
                continue;
            }

            let diry = (oy - py).signum();
            let mut y = py;
            let mut disty = 0;
            while y != oy {
                disty += 1;
                if y_gaps.contains(&(y as usize)) {
                    disty += DELTA - 1;
                }
                y += diry;
            }

            let dirx = (ox - px).signum();
            let mut x = px;
            let mut distx = 0;
            while x != ox {
                distx += 1;
                if x_gaps.contains(&(x as usize)) {
                    distx += DELTA - 1; // don't count prev
                }
                x += dirx;
            }

            if !visited.contains_key(&((py, px), (oy, ox))) || !visited.contains_key(&((oy, ox), (py, px))) {
                visited.insert(((py, px), (oy, ox)), disty + distx);
            }
        }
    }

    // this should overcount it by 2
    println!("{:?}", visited.values().sum::<i64>() / 2);
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
