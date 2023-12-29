use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{read, DIRS};

pub fn run() {
    let file = read!();
    let map = file.map(|line| line.chars().collect_vec()).collect_vec();

    // find all the points where it's surrounded by >, and use them to shorten paths
    // since there are very few crossroads
    let mut points: HashSet<(i32, i32)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, lines)| {
            lines
                .iter()
                .enumerate()
                .filter_map(|(x, &char)| {
                    if char == '#' {
                        return None;
                    }

                    let mut neighbors = 0;
                    for (dy, dx) in DIRS {
                        let (ny, nx) = (y as i32 + dy, x as i32 + dx);

                        if ny >= 0
                            && nx >= 0
                            && ny < map.len() as i32
                            && nx < map[0].len() as i32
                            && map[ny as usize][nx as usize] != '#'
                        {
                            neighbors += 1;
                        }
                    }

                    if neighbors >= 3 {
                        Some((y as i32, x as i32))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect();

    // add start and end
    let end = (map.len() as i32 - 1, map[0].len() as i32 - 2);

    points.insert((0, 1));
    points.insert(end);

    let graph = create_graph(points, &map);

    println!("Part1: {}", dfs(&graph, end));
}

// point to many points of dist
fn create_graph(
    points: HashSet<(i32, i32)>,
    map: &Vec<Vec<char>>,
) -> HashMap<(i32, i32), HashMap<(i32, i32), i32>> {
    let mut graph = HashMap::new();

    for &start_point in &points {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push((start_point, 0));
        visited.insert(start_point);

        while let Some((point, dist)) = stack.pop() {
            if dist > 0 && points.contains(&(point)) {
                graph
                    .entry(start_point)
                    .or_insert(HashMap::new())
                    .insert(point, dist);
                continue;
            }

            let (y, x) = point;

            // constraints on where you can go
            let dirs = match map[y as usize][x as usize] {
                '>' => vec![(0, 1)],
                '<' => vec![(0, -1)],
                'v' => vec![(1, 0)],
                '.' => Vec::from(DIRS),
                _ => unreachable!("{}", map[y as usize][x as usize]),
            };

            for (dy, dx) in dirs {
                let (ny, nx) = (y + dy, x + dx);

                if ny >= 0
                    && nx >= 0
                    && ny < map.len() as i32
                    && nx < map[0].len() as i32
                    && map[ny as usize][nx as usize] != '#'
                    && !visited.contains(&(ny, nx))
                {
                    stack.push(((ny, nx), dist + 1));
                    visited.insert((ny, nx));
                }
            }
        }
    }

    graph
}

fn dfs(map: &HashMap<(i32, i32), HashMap<(i32, i32), i32>>, dest: (i32, i32)) -> i32 {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    let start = (0, 1);

    let mut max_dist = 0;

    stack.push((start, 0));

    while let Some((pos, dist)) = stack.pop() {
        if pos == dest {
            max_dist = max_dist.max(dist);
            continue;
        }

        // trying everything, so we need to remove at the end to allow
        // another path
        visited.insert(pos);
        for (&next, d) in &map[&pos] {
            if !visited.contains(&next) {
                stack.push((next, dist + d));
            }
        }
        visited.remove(&pos);
    }

    max_dist
}
