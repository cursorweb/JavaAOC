use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

use crate::{read, DIRS};

pub fn run() {
    let file = read!();
    let map = file
        .map(|line| line.chars().map(|c| c as i32 - '0' as i32).collect_vec())
        .collect_vec();

    println!("Part1: {}", dijkstra(&map, 1, 3));
    println!("Part2: {}", dijkstra(&map, 4, 10));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
    heat_loss: i32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse order because heap chooses max
        // but we want min
        other.heat_loss.cmp(&self.heat_loss).then_with(|| {
            self.pos
                .cmp(&other.pos)
                .then_with(|| self.dir.cmp(&other.dir))
        })
    }
}

fn dijkstra(map: &Vec<Vec<i32>>, minstep: i32, maxstep: i32) -> i32 {
    let mut dist = HashMap::new();

    let mut heap = BinaryHeap::new();

    // dist.insert(((0, 0), (0, 0)), 0);
    heap.push(State {
        pos: (0, 0),
        dir: (0, 0),
        heat_loss: 0,
    });

    let end = (map.len() as i32 - 1, map[0].len() as i32 - 1);

    while let Some(State {
        pos,
        dir,
        heat_loss,
    }) = heap.pop()
    {
        // sad fact: you can't print out heap because it isn't printed in order
        // happy debugging ;)

        if pos == end {
            return heat_loss;
        }

        if heat_loss > *dist.get(&(pos, dir)).unwrap_or(&i32::MAX) {
            continue;
        }

        let (y, x) = pos;
        let (ody, odx) = dir;

        for (ndy, ndx) in DIRS {
            if (ndy, ndx) == (ody, odx) || (ndy, ndx) == (-ody, -odx) {
                // can't go backwards, and you already did the max steps
                // you must turn now!!
                continue;
            }

            let mut heat_loss = heat_loss;
            for k in 1..=maxstep {
                let ny = y + ndy * k;
                let nx = x + ndx * k;

                if ny < 0 || nx < 0 || ny >= map.len() as i32 || nx >= map[0].len() as i32 {
                    break;
                }

                heat_loss += map[ny as usize][nx as usize];
                if k >= minstep
                    && heat_loss < *dist.get(&((ny, nx), (ndy, ndx))).unwrap_or(&i32::MAX)
                {
                    heap.push(State {
                        pos: (ny, nx),
                        dir: (ndy, ndx),
                        heat_loss,
                    });
                    dist.insert(((ny, nx), (ndy, ndx)), heat_loss);
                }
            }
        }
    }

    unreachable!("skill issue lmao")
}

/*
fn bfs(map: &Vec<Vec<i32>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let endy = map.len() as i32 - 1;
    let endx = map[0].len() as i32 - 1;

    queue.push_front(State {
        pos: (0, 1),
        dir: (0, 1),
        heat_loss: 0,
        count: 3,
    });

    queue.push_front(State {
        pos: (1, 0),
        dir: (1, 0),
        heat_loss: 0,
        count: 3,
    });

    let mut min = i32::MAX;

    while let Some(state) = queue.pop_front() {
        let State {
            pos,
            dir,
            mut heat_loss,
            count,
        } = state;

        if visited.contains(&state) {
            continue;
        }

        visited.insert(state);

        if pos == (endy, endx) {
            min = min.min(heat_loss);
        }

        let (y, x) = pos;
        if y >= 0 && x >= 0 && y < map.len() as i32 && x < map[0].len() as i32 {
            heat_loss += map[y as usize][x as usize];
        } else {
            continue;
        }

        if count > 0 {
            // can move
            let ny = pos.0 + dir.0;
            let nx = pos.1 + dir.1;

            queue.push_front(State {
                pos: (ny, nx),
                dir,
                heat_loss,
                count: count - 1,
            });
        }

        if dir.0 != 0 {
            queue.push_front(State {
                pos: (pos.0, pos.1 + 1),
                dir: (0, 1),
                heat_loss,
                count: 3,
            });

            queue.push_front(State {
                pos: (pos.0, pos.1 - 1),
                dir: (0, -1),
                heat_loss,
                count: 3,
            });
        }

        if dir.1 != 0 {
            queue.push_front(State {
                pos: (pos.0 + 1, pos.1),
                dir: (1, 0),
                heat_loss,
                count: 3,
            });

            queue.push_front(State {
                pos: (pos.0 - 1, pos.1),
                dir: (-1, 0),
                heat_loss,
                count: 3,
            });
        }

        // dot!(
        //     map,
        //     |y, x, c: i32| if x == pos.1 && y == pos.0 {
        //         "#".to_string()
        //     } else {
        //         ".".to_string()
        //     },
        //     true
        // );
        // println!("{queue:?}");
    }

    min
}
*/
