// use std::collections::{HashSet, VecDeque};

use std::collections::HashSet;

use itertools::Itertools;

use crate::{dot, read, DIRS_EXTRA};

/// (y, x, prev)
// struct Point(i32, i32, HashSet<(i32, i32)>);

pub fn run() {
    let grid = read!()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'X' => 0,
                    'M' => 1,
                    'A' => 2,
                    'S' => 3,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut interests = vec![];
    let mut interests2 = vec![];

    for (y, r) in grid.iter().enumerate() {
        for (x, &c) in r.iter().enumerate() {
            if c == 0 {
                // X
                interests.push((y as i32, x as i32));
            }

            if c == 2 {
                // A
                interests2.push((y as usize, x as usize));
            }
        }
    }

    let mut sum = 0;
    let mut sum2 = 0;
    for (y, x) in interests {
        sum += solve1(&grid, (y, x));
    }

    let mut xx = HashSet::new();
    for (y, x) in &interests2 {
        sum2 += solve2(&grid, (*y, *x), &mut xx);
    }

    let arr = "XMAS".chars().collect_vec();
    dot!(grid, |y, x, c| if xx.contains(&(y, x)) {
        arr[c as usize]
    } else {
        '.'
    });
    println!("Part1: {sum}");
    println!("Part2: {sum2}");
}

fn solve1(grid: &Vec<Vec<i32>>, (sy, sx): (i32, i32)) -> i32 {
    let mut count = 0;

    'main: for (dy, dx) in DIRS_EXTRA {
        let (mut y, mut x) = (sy, sx);

        for _ in 0..3 {
            let (ny, nx) = (dy + y, dx + x);
            if ny < 0 || nx < 0 || ny >= grid.len() as i32 || nx >= grid[0].len() as i32 {
                continue 'main;
            }

            if grid[ny as usize][nx as usize] <= grid[y as usize][x as usize] {
                continue 'main;
            }

            (y, x) = (ny, nx);
        }

        count += 1;
    }

    count
}

fn solve2(grid: &Vec<Vec<i32>>, (sy, sx): (usize, usize), x: &mut HashSet<(usize, usize)>) -> i32 {
    if sy < 1 || sx < 1 || sy == grid.len() - 1 || sx == grid.len() - 1 {
        return 0;
    }

    // let vecarr = vec![
    //     grid[sy - 1][sx - 1],
    //     grid[sy + 1][sx - 1],
    //     grid[sy - 1][sx + 1],
    //     grid[sy + 1][sx + 1],
    // ];

    let arr = ["X", "M", "A", "S"];

    let top = arr[grid[sy - 1][sx - 1] as usize].to_owned() + arr[grid[sy + 1][sx + 1] as usize];
    let bottom = arr[grid[sy + 1][sx - 1] as usize].to_owned() + arr[grid[sy - 1][sx + 1] as usize];
    println!("{top} {bottom}");

    // M == 1, S == 3
    if (top == "MS" || top == "SM") && (bottom == "SM" || bottom == "MS") {
        x.insert((sy, sx));
        x.insert((sy - 1, sx - 1));
        x.insert((sy + 1, sx - 1));
        x.insert((sy - 1, sx + 1));
        x.insert((sy + 1, sx + 1));
        1
    } else {
        0
    }
}

fn count(arr: &Vec<i32>, x: i32) -> usize {
    arr.iter().filter(|&n| *n == x).count()
}
