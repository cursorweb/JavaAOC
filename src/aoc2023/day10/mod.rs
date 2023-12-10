use std::collections::{HashMap, HashSet, VecDeque};

use crate::{input, read};

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal;
*/
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Kind {
    /// |
    Vert,

    /// -
    Horiz,

    /// L
    DownRight,

    /// J
    DownLeft,

    /// 7
    UpLeft,

    /// F
    UpRight,

    /// .
    Ground,

    /// S
    Start,
}

use itertools::Itertools;
use Kind::*;

pub fn run() {
    let file = read!();

    let mut start = (0, 0);

    let map = file
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, col)| match col {
                    '|' => Vert,
                    '-' => Horiz,
                    'L' => DownRight,
                    'J' => DownLeft,
                    '7' => UpLeft,
                    'F' => UpRight,
                    '.' => Ground,
                    'S' => {
                        start = (y as i32, x as i32);
                        Start
                    }
                    _ => unreachable!("{col}"),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut solver = MapSolver {
        map,
        queue: VecDeque::new(),
        visited: HashSet::new(),
        max: 0,
        valid: HashMap::from([
            (Vert, vec![(-1, 0), (1, 0)]),
            (Horiz, vec![(0, -1), (0, 1)]),
            (UpLeft, vec![(1, 0), (0, -1)]),
            (UpRight, vec![(1, 0), (0, 1)]),
        ]),
    };

    println!("{}", solver.bfs(start));
}

#[derive(Clone, Copy, Debug, Eq)]
struct State {
    kind: Kind,
    pos: (i32, i32),
    dist: i32,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

struct MapSolver {
    map: Vec<Vec<Kind>>,
    queue: VecDeque<State>,
    visited: HashSet<State>,
    max: i32,
    valid: HashMap<Kind, Vec<(i32, i32)>>,
}

impl MapSolver {
    fn _dot(&self, y: i32, x: i32) {
        for (cy, row) in self.map.iter().enumerate() {
            for (cx, &col) in row.iter().enumerate() {
                if (cy as i32, cx as i32) == (y, x) {
                    print!("#");
                } else {
                    print!(
                        "{}",
                        match col {
                            Vert => "|",
                            Horiz => "-",
                            DownRight => "L",
                            DownLeft => "J",
                            UpLeft => "7",
                            UpRight => "F",
                            Ground => ".",
                            Start => "S",
                        }
                    );
                }
            }
            println!();
        }
        println!();
    }

    fn get(&self, y: i32, x: i32) -> Option<Kind> {
        self.map.get(y as usize)?.get(x as usize).copied()
    }

    fn add(&mut self, (y, x): (i32, i32), (dy, dx): (i32, i32), dist: i32) {
        if let Some(kind) = self.get(y + dy, x + dx) {
            let state = State {
                kind,
                pos: (y + dy, x + dx),
                dist: dist + 1,
            };

            if !self.visited.contains(&state) {
                self.visited.insert(state);
                self.queue.push_back(state);

                if state.dist > self.max {
                    self.max = state.dist;
                }
            }
        }
    }

    fn bfs(&mut self, (sy, sx): (i32, i32)) -> i32 {
        // up
        if let Some(kind) = self.get(sy - 1, sx) {
            if kind == Vert || kind == UpRight || kind == UpLeft {
                let state = State {
                    kind,
                    pos: (sy - 1, sx),
                    dist: 1,
                };
                self.queue.push_front(state);
                self.visited.insert(state);
            }
        }

        // right
        if let Some(kind) = self.get(sy, sx + 1) {
            if kind == Horiz || kind == DownLeft || kind == UpLeft {
                let state = State {
                    kind,
                    pos: (sy, sx + 1),
                    dist: 1,
                };
                self.queue.push_front(state);
                self.visited.insert(state);
            }
        }

        // down
        if let Some(kind) = self.get(sy + 1, sx) {
            if kind == Vert || kind == DownRight || kind == DownLeft {
                let state = State {
                    kind,
                    pos: (sy + 1, sx),
                    dist: 1,
                };
                self.queue.push_front(state);
                self.visited.insert(state);
            }
        }

        // left
        if let Some(kind) = self.get(sy, sx - 1) {
            if kind == Vert || kind == UpRight || kind == DownRight {
                let state = State {
                    kind,
                    pos: (sy, sx - 1),
                    dist: 1,
                };
                self.queue.push_front(state);
                self.visited.insert(state);
            }
        }

        while let Some(state) = self.queue.pop_front() {
            let (y, x) = state.pos;
            self._dot(y, x);
            // input!();
            for &(dy, dx) in &self.valid[&state.kind] {
                self.add((y, x), (dy, dx), state.dist);
            }
            match state.kind {
                Vert => {
                    for &(dy, dx) in &self.valid[&Vert] {
                        self.add((y, x), (dy, dx), state.dist);
                    }
                }
                Horiz => {
                    self.add((y, x), (dy, dx), state.dist);
                }
                DownRight => {
                    // |^   then: diry =  1
                    // | <  if:   dirx = -1
                    // +---
                    //  v   if:   diry = 1
                    //   >  then: dirx = 1
                    self.add((y, x), (-state.dir.1, state.dir.0), state.dist);
                }
                DownLeft => {
                    //   ^|  then: diry = -1
                    //  > |  if:   dirx =  1
                    // ---+
                    //  v   if: diry =  1
                    // <  then: dirx = -1
                    self.add((y, x), (-state.dir.1, -state.dir.0), state.dist);
                }
                UpRight => {
                    // ---+
                    // | >   then: dirx =  1
                    // |^    if:   diry = -1
                    //  <   if:   dirx = -1
                    // v  then:   diry =  1
                    self.add((y, x), (-state.dir.1, -state.dir.0), state.dist);
                }
                UpLeft => {
                    // ---+
                    //  < |  then: dirx = -1
                    //   ^|  if:   diry = -1
                    // >   if:   dirx =  1
                    //  v  then: diry =  1
                    self.add((y, x), (state.dir.1, state.dir.0), state.dist);
                }
                Start => {} // no.
                _ => unreachable!("skill issue lmao {:?}", state.kind),
            }
        }

        self.max
    }
}
