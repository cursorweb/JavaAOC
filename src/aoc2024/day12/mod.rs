use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{in_grid_bounds, read, DIRS, DIRS_DIAG};

pub fn run() {
    let grid = read!().map(|line| line.chars().collect_vec()).collect_vec();

    // a list of all the visited points
    // if you are in this territory, you aren't in new territory
    let mut all_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut regions: Vec<(char, HashSet<(i32, i32)>)> = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if all_visited.contains(&(y as i32, x as i32)) {
                continue;
            }

            let visited = flood_fill((y as i32, x as i32), c, &grid);
            regions.push((c, visited.clone()));
            all_visited.extend(visited);
        }
    }

    // TODO: use iterators
    let mut sum = 0;
    for (c, region) in &regions {
        let peri = perimeter(*c, region, &grid);
        let area = region.len() as i32;
        let price = peri * area;
        sum += price;
    }

    println!("Part1: {sum}");

    let mut sum = 0;
    for (_, region) in &regions {
        let edges = edger(region);
        let area = region.len() as i32;
        let price = area * edges;
        sum += price;
    }

    println!("Part2: {sum}");
}

fn perimeter(c: char, region: &HashSet<(i32, i32)>, grid: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for &(y, x) in region {
        let mut peri = 0;
        for (dy, dx) in DIRS {
            let (ny, nx) = (y + dy, x + dx);
            if !in_grid_bounds((ny, nx), &grid) || grid[ny as usize][nx as usize] != c {
                peri += 1;
            }
        }

        sum += peri;
    }

    sum
}

fn edger(region: &HashSet<(i32, i32)>) -> i32 {
    // a corner is:
    // - sides are nonregion
    // - sides region but corner is nonregion

    let mut corners = 0;
    for &(y, x) in region {
        for (dy, dx) in DIRS_DIAG {
            let vert = (y + dy, x);
            let horiz = (y, x + dx);
            let corner = (y + dy, x + dx);

            if !region.contains(&vert) && !region.contains(&horiz) {
                corners += 1;
            }

            if region.contains(&vert) && region.contains(&horiz) && !region.contains(&corner) {
                corners += 1;
            }
        }
    }

    corners
}

fn flood_fill(start_pos: (i32, i32), c: char, grid: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_front(start_pos);
    visited.insert(start_pos);

    loop {
        let Some((y, x)) = queue.pop_back() else {
            break;
        };

        for (dy, dx) in DIRS {
            let npos = (y + dy, x + dx);
            let (ny, nx) = npos;
            if in_grid_bounds(npos, &grid)
                && !visited.contains(&npos)
                && grid[ny as usize][nx as usize] == c
            {
                visited.insert(npos);
                queue.push_front(npos);
            }
        }
    }

    visited
}
