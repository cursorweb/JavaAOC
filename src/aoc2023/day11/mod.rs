use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let map = file.map(|line| line.chars().collect_vec()).collect_vec();

    assert_eq!(map.len(), map[0].len());

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

    let points = new_map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(|(x, &c)| {
            if c == '#' {
                Some((y as i64, x as i64))
            } else {
                None
            }
        }).collect_vec()
    }).collect_vec();

    let mut dist = 0;

    for &point in &points {
        for &op in &points {
            if point == op {
                continue;
            }
            dist += (op.1 - point.1).abs() + (op.0 - point.0).abs();
        }
    }

    // this should overcount it by 2
    println!("{:?}", dist / 2);
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
