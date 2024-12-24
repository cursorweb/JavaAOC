use std::collections::{HashSet, VecDeque};

use crate::{in_grid_bounds, read, DIRS};

pub fn run() {
    let file = read!();
    let grid: Vec<Vec<i32>> = file
        .map(|r| r.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect();

    let mut zeroes = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v == 0 {
                zeroes.push((y as i32, x as i32));
            }
        }
    }

    let sum = zeroes.iter().fold(0, |acc, point| acc + bfs(point, &grid));

    println!("Part1: {sum}");

    let sum = zeroes.iter().fold(0, |acc, point| acc + bfs2(point, &grid));

    println!("Part2: {sum}");
}

pub fn bfs(start: &(i32, i32), grid: &Vec<Vec<i32>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(*start);
    queue.push_front(*start);

    let mut out = 0;

    loop {
        let Some((y, x)) = queue.pop_back() else {
            break;
        };

        let elevation = grid[y as usize][x as usize];

        if elevation == 9 {
            out += 1;
        }

        for (dy, dx) in DIRS {
            let ny = y + dy;
            let nx = x + dx;

            let npos = (ny, nx);

            if in_grid_bounds(npos, grid)
                && grid[ny as usize][nx as usize] - elevation == 1
                && !visited.contains(&npos)
            {
                visited.insert(npos);
                queue.push_front(npos);
            }
        }
    }

    out
}

pub fn bfs2(start: &(i32, i32), grid: &Vec<Vec<i32>>) -> i32 {
    // Remove visited to let the path retrace itself
    let mut queue = VecDeque::new();

    queue.push_front(*start);

    let mut out = 0;

    loop {
        let Some((y, x)) = queue.pop_back() else {
            break;
        };

        let elevation = grid[y as usize][x as usize];

        if elevation == 9 {
            out += 1;
        }

        for (dy, dx) in DIRS {
            let ny = y + dy;
            let nx = x + dx;

            let npos = (ny, nx);

            if in_grid_bounds(npos, grid) && grid[ny as usize][nx as usize] - elevation == 1 {
                queue.push_front(npos);
            }
        }
    }

    out
}
