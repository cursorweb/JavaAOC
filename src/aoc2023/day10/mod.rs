use std::collections::{HashMap, HashSet, VecDeque};

use crate::{read, DIRS};

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

    /// `-`
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
        map: map.clone(),
        queue: VecDeque::new(),
        visited: HashSet::new(),
        max: 0,
        valid: HashMap::from([
            (Vert, vec![(-1, 0), (1, 0)]),
            (Horiz, vec![(0, -1), (0, 1)]),
            (UpLeft, vec![(1, 0), (0, -1)]),
            (UpRight, vec![(1, 0), (0, 1)]),
            (DownLeft, vec![(-1, 0), (0, -1)]),
            (DownRight, vec![(-1, 0), (0, 1)]),
        ]),
    };

    let (count, rloop) = solver.bfs(start);

    println!("Part1: {count}");

    let mut filler = MapFiller::new(map, rloop, start);

    println!("Part2: {}", filler.fill());
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

    fn bfs(&mut self, (sy, sx): (i32, i32)) -> (i32, HashSet<(i32, i32)>) {
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

            let Some(valid) = self.valid.get(&state.kind) else {
                assert_eq!(state.kind, Start);
                continue;
            };

            for (dy, dx) in valid.clone() {
                self.add((y, x), (dy, dx), state.dist);
            }
        }

        (
            self.max,
            self.visited.iter().map(|state| state.pos).collect(),
        )
    }
}

struct MapFiller {
    map: Vec<Vec<Kind>>,
    maxx: i32,
    maxy: i32,
    /// all the points that are outside the loop
    visited: HashSet<(i32, i32)>,
    loop_peri: i32,
}

impl MapFiller {
    fn _dot(map: &Vec<Vec<Kind>>, points: &HashSet<(i32, i32)>) {
        for (cy, row) in map.iter().enumerate() {
            for (cx, &col) in row.iter().enumerate() {
                if points.contains(&(cy as i32, cx as i32)) {
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

    /// remap everything outside the loop to .
    /// and upscale
    fn remap(
        mut map: Vec<Vec<Kind>>,
        rloop: HashSet<(i32, i32)>,
        (sy, sx): (i32, i32),
    ) -> Vec<Vec<Kind>> {
        let mut new_map = Vec::new();

        macro_rules! get {
            ($y:expr, $x:expr, $pattern:pat) => {
                match map.get(($y) as usize) {
                    Some(a) => match a.get(($x) as usize) {
                        Some(v) => matches!(v, $pattern),
                        _ => false,
                    },
                    _ => false,
                }
            };
        }

        map[sy as usize][sx as usize] = {
            // up
            let up = get!(sy - 1, sx, Vert | UpRight | UpLeft);

            // right
            let right = get!(sy, sx + 1, Horiz | DownLeft | UpLeft);

            // down
            let down = get!(sy + 1, sx, Vert | DownRight | DownLeft);

            // left
            let left = get!(sy, sx - 1, Vert | UpRight | DownRight);

            match (up, right, down, left) {
                (true, false, true, false) => Vert,
                (false, true, false, true) => Horiz,
                (true, true, false, false) => DownRight,
                (true, false, false, true) => DownLeft,
                (false, false, true, true) => UpLeft,
                (false, true, true, false) => UpRight,
                _ => unreachable!(
                    "state (up, right, down, left) = {:?}",
                    (up, right, down, left)
                ),
            }
        };

        // double the size
        // that way, gaps are easier
        // extend all that can be extended
        // . -> .#
        //      ##
        for (y, row) in map.iter().enumerate() {
            let mut row_up = Vec::new();
            let mut row_down = Vec::new();
            /*
            |.  --  L-  J.  7.  F-  ..
            |.  ..  ..  ..  |.  |.  ..
            */
            for (x, &kind) in row.iter().enumerate() {
                if rloop.contains(&(y as i32, x as i32)) {
                    match kind {
                        Vert => {
                            row_up.extend([Vert, Ground].iter());
                            row_down.extend([Vert, Ground].iter());
                        }
                        Horiz => {
                            row_up.extend([Horiz, Horiz].iter());
                            row_down.extend([Ground, Ground].iter());
                        }
                        DownRight => {
                            row_up.extend([DownRight, Horiz].iter());
                            row_down.extend([Ground, Ground].iter());
                        }
                        DownLeft => {
                            row_up.extend([DownLeft, Ground].iter());
                            row_down.extend([Ground, Ground].iter());
                        }
                        UpLeft => {
                            row_up.extend([UpLeft, Ground].iter());
                            row_down.extend([Vert, Ground].iter());
                        }
                        UpRight => {
                            row_up.extend([UpRight, Horiz].iter());
                            row_down.extend([Vert, Ground].iter());
                        }
                        Ground => {
                            row_up.extend([Ground, Ground].iter());
                            row_down.extend([Ground, Ground].iter());
                        }
                        _ => unreachable!("skill issue lmao (don't start)"),
                    }
                } else {
                    row_up.extend([Ground, Ground].iter());
                    row_down.extend([Ground, Ground].iter());
                }
            }

            new_map.push(row_up);
            new_map.push(row_down);
        }

        new_map
    }

    fn new(map: Vec<Vec<Kind>>, rloop: HashSet<(i32, i32)>, start: (i32, i32)) -> Self {
        // upscale, and then count only the top left corners
        let peri = rloop.len();
        let new_map = Self::remap(map, rloop, start);

        // inclusive
        let maxx = new_map[0].len() - 1;
        let maxy = new_map.len() - 1;

        Self {
            map: new_map,
            maxx: maxx as i32,
            maxy: maxy as i32,
            visited: HashSet::new(),
            loop_peri: peri as i32,
        }
    }

    fn fill(&mut self) -> i32 {
        for y in 0..=self.maxy {
            self._fill((y, 0));
            self._fill((y, self.maxx));
        }

        /*
        ..    ....
        .. -> .... (factor of 4)
              ....
              ....
        */
        let downscale_area = ((self.maxx + 1) * (self.maxy + 1)) / 4;
        let downsale_outside = self
            .visited
            .iter()
            .filter(|(y, x)| y % 2 == 0 && x % 2 == 0)
            .count() as i32;

        /*
        .....
        .+-+.
        .|*|.
        .+-+.
        .....

        area = 25
        perimeter = 8
        outside = 16
        */

        downscale_area - downsale_outside - self.loop_peri
    }

    fn _fill(&mut self, (y, x): (i32, i32)) {
        if self.map[y as usize][x as usize] != Ground || self.visited.contains(&(y, x)) {
            return;
        }

        let mut queue = VecDeque::new();

        self.visited.insert((y, x));
        queue.push_front((y, x));

        while let Some((y, x)) = queue.pop_front() {
            for (dy, dx) in DIRS {
                let (ny, nx) = (y + dy, x + dx);
                if !self.visited.contains(&(ny, nx))
                    && ny >= 0
                    && ny <= self.maxy
                    && ny >= 0
                    && nx <= self.maxx
                    && nx >= 0
                    && self.map[ny as usize][nx as usize] == Ground
                {
                    self.visited.insert((ny, nx));
                    queue.push_front((ny, nx));
                }
            }
        }
    }
}
