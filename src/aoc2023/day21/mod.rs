use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{read, DIRS};

const STEPS: i32 = 64;
const STEPS2: i64 = 26_501_365;

pub fn run() {
    let file = read!();

    let mut start = (0, 0);

    let map = file
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (y as i32, x as i32);
                        '.'
                    } else {
                        c
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    // crate::dot!(map);
    println!("{}", bfs(&map, start));
}

fn bfs(map: &Vec<Vec<char>>, (sy, sx): (i32, i32)) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_front(((sy, sx), STEPS));
    visited.insert((sy, sx));

    let mut ans = HashSet::new();

    while let Some(((y, x), steps)) = queue.pop_front() {
        if steps % 2 == 0 {
            ans.insert((y, x));
        }

        if steps == 0 {
            continue;
        }

        for (dy, dx) in DIRS {
            let npos @ (ny, nx) = (y + dy, x + dx);
            if !visited.contains(&npos)
                && (ny >= 0 && ny < map.len() as i32 && nx >= 0 && nx < map[0].len() as i32)
                && map[ny as usize][nx as usize] != '#'
            {
                visited.insert(npos);
                
                // go to all the ones that have steps before going on to steps - 1
                queue.push_back((npos, steps - 1));
            }
        }
    }

    ans.len() as i32
}
