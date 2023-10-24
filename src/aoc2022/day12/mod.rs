use std::collections::{HashMap, VecDeque};

use crate::{read, DIRS};

/// (y, x)
type Point = (i32, i32);
/// point, height
type Grid = HashMap<Point, i32>;

pub fn run() {
    let file = read!();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut starting_points = Vec::new();

    let grid: Grid = file
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        start = (y as i32, x as i32);
                        return (start, 0);
                    } else if char == 'E' {
                        end = (y as i32, x as i32);
                        return (end, 'z' as i32 - 'a' as i32);
                    }

                    if char == 'a' {
                        starting_points.push((y as i32, x as i32));
                    }

                    ((y as i32, x as i32), char as i32 - 'a' as i32)
                })
                .collect::<Vec<(Point, i32)>>()
        })
        .collect();

    let mut shortest_path = solve(&grid, start, end);
    println!("Part1: {shortest_path}");

    for point in starting_points {
        let dist = solve(&grid, point, end);
        if dist > 0 && dist < shortest_path {
            shortest_path = dist;
        }
    }

    println!("Part2: {shortest_path}");
}

// easy bfs
fn solve(grid: &Grid, start: Point, end: Point) -> i32 {
    // [(point)]: Option(previous point) <-- in case it's the starting point
    let mut visited: HashMap<Point, Option<Point>> = HashMap::new();

    let mut q = VecDeque::new();

    visited.insert(start, None);
    q.push_front(start);

    loop {
        let Some(point) = q.pop_back() else {
            break;
        };

        let curr_height = grid[&point];

        if point == end {
            // includes the current (end) so 1
            // but sub 1 because don't include the starting position
            let mut steps = 0;
            let mut curr = point;
            loop {
                let Some(prev) = visited[&curr] else {
                    break;
                };

                steps += 1;

                curr = prev;
            }

            return steps;
        } else {
            for (dy, dx) in DIRS {
                let next_point = (point.0 + dy, point.1 + dx);

                if visited.contains_key(&next_point) {
                    continue;
                }

                let Some(&next_height) = grid.get(&next_point) else {
                    continue;
                };

                // you can drop down from really high places!
                if next_height - curr_height <= 1 {
                    visited.insert(next_point, Some(point));
                    q.push_front(next_point);
                }
            }
        }
    }

    -1 // in part 2 some of them can't move
}
