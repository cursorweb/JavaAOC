use itertools::Itertools;

use crate::{in_grid_bounds, read, DIRS_EXTRA};

pub fn run() {
    let mut file = read!().map(|line| line.chars().collect_vec()).collect_vec();

    let mut part1 = 0;
    for (y, line) in file.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != '@' {
                continue;
            }

            let mut count = 0;
            for (dy, dx) in DIRS_EXTRA {
                if in_grid_bounds((y as i32 + dy, x as i32 + dx), &file)
                    && file[(y as i32 + dy) as usize][(x as i32 + dx) as usize] == '@'
                {
                    count += 1;
                }
            }

            if count < 4 {
                part1 += 1;
            }
        }
    }

    println!("Part1: {part1}");

    let mut part2 = 0;
    while let Some(x) = remove(&mut file) {
        part2 += x;
    }

    println!("Part2: {part2}");
}

fn remove(file: &mut Vec<Vec<char>>) -> Option<usize> {
    let mut to_remove = vec![];

    for (y, line) in file.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '@' {
                continue;
            }

            let mut count = 0;
            for (dy, dx) in DIRS_EXTRA {
                if in_grid_bounds((y as i32 + dy, x as i32 + dx), &file)
                    && file[(y as i32 + dy) as usize][(x as i32 + dx) as usize] == '@'
                {
                    count += 1;
                }
            }

            if count < 4 {
                to_remove.push((y, x));
            }
        }
    }

    for &(y, x) in &to_remove {
        file[y][x] = '.';
    }

    if to_remove.len() > 0 {
        Some(to_remove.len())
    } else {
        None
    }
}
