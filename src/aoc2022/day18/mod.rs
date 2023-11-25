use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::read;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32, i32);

impl From<(i32, i32, i32)> for Point {
    fn from((a, b, c): (i32, i32, i32)) -> Self {
        Self(a, b, c)
    }
}

impl Point {
    fn offset(&self, x: i32, y: i32, z: i32) -> Vec<Point> {
        vec![
            Point(self.0 + x, self.1 + y, self.2 + z),
            Point(self.0 - x, self.1 - y, self.2 - z),
        ]
    }
}

pub fn run() {
    let file = read!();

    // (x, y, z) or whatever -- order doesn't matter lol
    // lhr anyone?
    let cubes: HashSet<Point> = file
        .map(|line| {
            Point::from(
                line.split(",")
                    .map(|x| x.parse().unwrap())
                    .collect_tuple::<(_, _, _)>()
                    .unwrap(),
            )
        })
        .collect();

    // potential pockets
    let mut potential = HashSet::new();
    let mut faces = 0;

    for &cube in &cubes {
        for adj in adjacent(cube) {
            if !cubes.contains(&adj) {
                potential.insert(adj);
                faces += 1;
            }
        }
    }

    println!("Part1: {faces}");
    /*
    go to a potential-pocket
    "expand"
        if you can't go anymore, you have a pocket
    if you reach a border
        that is not an air pocket
    */

    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;

    for &Point(x, y, z) in &cubes {
        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }

        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }

        if z > max_z {
            max_z = z;
        }
        if z < min_z {
            min_z = z;
        }
    }

    let mut pocket_points = HashSet::new();
    let mut state = ExpanderState {
        cache: HashMap::new(),
        cubes: cubes.clone(),
        max_x,
        max_y,
        max_z,
        min_x,
        min_y,
        min_z,
    };

    for coord in potential {
        if let Some(path) = state.expand(coord) {
            pocket_points.extend(path);
        }
    }

    for point in pocket_points {
        for adj in adjacent(point) {
            if cubes.contains(&adj) {
                faces -= 1;
            }
        }
    }

    println!("Part2: {faces}");
}

struct ExpanderState {
    /// every point that has been seen before
    /// true = is a pocket, false = not a pocket
    cache: HashMap<Point, bool>,
    cubes: HashSet<Point>,

    max_x: i32,
    max_y: i32,
    max_z: i32,

    min_x: i32,
    min_y: i32,
    min_z: i32,
}

impl ExpanderState {
    fn expand(&mut self, point: Point) -> Option<HashSet<Point>> {
        let mut path = HashSet::new();
        let is_pocket = self._expand(point, &mut path);

        // add all the previously visited
        // into the cache
        self.cache.extend(path.iter().map(|&x| (x, is_pocket)));

        if is_pocket {
            Some(path)
        } else {
            None
        }
    }

    fn _expand(&mut self, point: Point, path: &mut HashSet<Point>) -> bool {
        if self.cache.contains_key(&point) {
            // it is either
            // - out of bounds
            // - another point in an existing (found) air pocket
            return self.cache[&point];
        }

        if point.0 < self.min_x
            || point.1 < self.min_y
            || point.2 < self.min_z
            || point.0 > self.max_x
            || point.1 > self.max_y
            || point.2 > self.max_z
        {
            // out of bounds
            return false;
        }

        path.insert(point);

        for adj in adjacent(point) {
            // can't go into a cube, and can't go back
            if !path.contains(&adj) && !self.cubes.contains(&adj) {
                if !self._expand(adj, path) {
                    return false;
                }
            }
        }

        true
    }
}

fn adjacent(point: Point) -> Vec<Point> {
    vec![
        point.offset(1, 0, 0),
        point.offset(0, 1, 0),
        point.offset(0, 0, 1),
    ]
    .into_iter()
    .flatten()
    .collect()
}
