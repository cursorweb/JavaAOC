use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::{dot, read};

pub fn run() {
    let file = read!();
    let map = file.map(|line| line.chars().collect_vec()).collect_vec();
    println!("Part1: {}", bfs(&map));
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    /// (y, x)
    pos: (i32, i32),
    /// (dy, dx)
    dir: (i32, i32),
}

fn bfs(map: &Vec<Vec<char>>) -> i32 {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // /
    let rotate_pos = HashMap::from([
        ((1, 0), (0, -1)),
        ((0, 1), (-1, 0)),
        ((-1, 0), (0, 1)),
        ((0, -1), (1, 0)),
    ]);

    // \
    let rotate_neg = HashMap::from([
        ((1, 0), (0, 1)),
        ((0, 1), (1, 0)),
        ((-1, 0), (0, -1)),
        ((0, -1), (-1, 0)),
    ]);

    queue.push_front(State {
        pos: (0, 0),
        dir: match map[0][0] {
            '/' => rotate_pos[&(0, 1)],
            '\\' => rotate_neg[&(0, 1)],
            '.' | '-' => (0, 1),
            '|' => todo!("ur screwed lmao"),
            _ => unreachable!(),
        },
    });

    while let Some(state) = queue.pop_front() {
        let State {
            dir: (dy, dx),
            pos: (y, x),
        } = state;
        if visited.contains(&state) {
            continue;
        }

        visited.insert(state);

        let next_pos = (y + dy, x + dx);
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= height || next_pos.1 >= width {
            continue;
        }

        match map[next_pos.0 as usize][next_pos.1 as usize] {
            '.' => {
                queue.push_front(State {
                    pos: next_pos,
                    dir: (dy, dx),
                });
            }
            '|' => {
                if dx != 0 {
                    queue.push_front(State {
                        pos: next_pos,
                        dir: (1, 0),
                    });

                    queue.push_front(State {
                        pos: next_pos,
                        dir: (-1, 0),
                    });
                } else {
                    queue.push_front(State {
                        pos: next_pos,
                        dir: (dy, dx),
                    });
                }
            }
            '-' => {
                if dy != 0 {
                    queue.push_front(State {
                        pos: next_pos,
                        dir: (0, 1),
                    });

                    queue.push_front(State {
                        pos: next_pos,
                        dir: (0, -1),
                    });
                } else {
                    queue.push_front(State {
                        pos: next_pos,
                        dir: (dy, dx),
                    });
                }
            }
            '/' => {
                queue.push_front(State {
                    pos: next_pos,
                    dir: rotate_pos[&(dy, dx)],
                });
            }
            '\\' => {
                queue.push_front(State {
                    pos: next_pos,
                    dir: rotate_neg[&(dy, dx)],
                });
            }
            _ => unreachable!(),
        }
    }

    let visited: HashSet<(i32, i32)> = visited
        .into_iter()
        .map(|State { pos: (y, x), .. }| (y, x))
        .collect();

    dot!(map, |y, x, c| {
        match c {
            _ if visited.contains(&(y, x)) => '#',
            _ => c,
        }
    });

    visited.len() as i32
}
