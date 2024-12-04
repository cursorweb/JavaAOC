use itertools::Itertools;

use crate::{read, DIRS_EXTRA};

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

    for (y, x) in &interests2 {
        sum2 += solve2(&grid, (*y, *x));
    }

    println!("Part1: {sum}");
    println!("Part2: {sum2}");
}

fn solve1(grid: &Vec<Vec<i32>>, (sy, sx): (i32, i32)) -> i32 {
    let mut count = 0;

    'main: for (dy, dx) in DIRS_EXTRA {
        let (mut y, mut x) = (sy, sx);

        for _ in 0..3 {
            let (ny, nx) = (dy + y, dx + x);

            // if out of bounds
            if ny < 0 || nx < 0 || ny >= grid.len() as i32 || nx >= grid[0].len() as i32 {
                continue 'main;
            }

            // each letter has to be successively greater
            if grid[ny as usize][nx as usize] <= grid[y as usize][x as usize] {
                continue 'main;
            }

            (y, x) = (ny, nx);
        }

        count += 1;
    }

    count
}

const XMAS: [&str; 4] = ["X", "M", "A", "S"];

fn solve2(grid: &Vec<Vec<i32>>, (sy, sx): (usize, usize)) -> i32 {
    // if A is edged, then ignore it
    if sy < 1 || sx < 1 || sy == grid.len() - 1 || sx == grid.len() - 1 {
        return 0;
    }

    let tl_br =
        XMAS[grid[sy - 1][sx - 1] as usize].to_owned() + XMAS[grid[sy + 1][sx + 1] as usize];
    let bl_tr =
        XMAS[grid[sy + 1][sx - 1] as usize].to_owned() + XMAS[grid[sy - 1][sx + 1] as usize];

    // M == 1, S == 3
    if (tl_br == "MS" || tl_br == "SM") && (bl_tr == "SM" || bl_tr == "MS") {
        1
    } else {
        0
    }
}
