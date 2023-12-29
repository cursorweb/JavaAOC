use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::{range_intersects, read};

/// inclusive
#[derive(Debug, Clone, Copy)]
struct Cube {
    start: (i32, i32, i32),
    end: (i32, i32, i32),
}

impl Cube {
    /// rect intersection
    fn intersect(&self, other: &Cube) -> bool {
        range_intersects((self.start.0, self.end.0), (other.start.0, other.end.0))
            && range_intersects((self.start.1, self.end.1), (other.start.1, other.end.1))
    }
}

pub fn run() {
    let file = read!();
    let mut cubes: Vec<Cube> = file
        .map(|line| {
            let (s, e) = line.split_once("~").unwrap();
            let start = s
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let end = e
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Cube { start, end }
        })
        .collect();

    cubes.sort_by(|a, b| a.start.2.cmp(&b.start.2));

    settle_down(&mut cubes);

    let (cube_supported_by, cube_supports) = get_supporters(&cubes);

    let mut count = 0;

    let mut critical_cubes = Vec::new();

    for i in 0..cubes.len() {
        // all the cubes the cube supports have 2+ supporters
        if let Some(supports) = cube_supports.get(&i) {
            if supports
                .iter()
                .all(|cube| cube_supported_by[cube].len() >= 2)
            {
                count += 1;
            } else {
                critical_cubes.push(i);
            }
        } else {
            // doesn't support anyone, can be removed
            count += 1;
        }
    }

    println!("Part1: {count}");

    let mut count = 0;
    for i in 0..cubes.len() {
        count += count_supports(i, &cube_supported_by, &cube_supports);
    }

    println!("Part2: {count}");
}

fn count_supports(
    i: usize,
    cube_supported_by: &HashMap<usize, HashSet<usize>>,
    cube_supports: &HashMap<usize, HashSet<usize>>,
) -> usize {
    let mut queue = VecDeque::new();
    let mut fallers = HashSet::new();

    // get all the cubes that will fall (supported only by i)
    if let Some(supports) = cube_supports.get(&i) {
        for &j in supports {
            if cube_supported_by[&j].len() == 1 {
                queue.push_front(j);
                fallers.insert(j);
            }
        }
    }

    // this is falling, because
    // we check if supports are things that fall
    // so this technically falls too
    fallers.insert(i);

    while let Some(j) = queue.pop_front() {
        if let Some(supports) = cube_supports.get(&j) {
            // all the unique cubes that don't currently fall
            for k in supports - &fallers {
                // if their supports are all things that fall
                if cube_supported_by[&k].is_subset(&fallers) {
                    queue.push_front(k);
                    fallers.insert(k);
                }
            }
        }
    }

    // don't include the original i
    fallers.len() - 1
}

fn settle_down(cubes: &mut Vec<Cube>) {
    // make cubes settle
    for i in 0..cubes.len() {
        let mut max_z = 1;
        let cube = cubes[i];
        for other in &cubes[0..i] {
            if other.intersect(&cube) {
                max_z = max_z.max(other.end.2 + 1);
            }
        }

        // (old_start - max_z) = fall dist
        cubes[i].end.2 -= cubes[i].start.2 - max_z;
        // new_start = old_start - (old_start - max_z)
        cubes[i].start.2 = max_z;
    }
}

/// `([cube]: <supported by>, [cube]: <cubes supported>)`
/// Cubes support if z_diff = 1
fn get_supporters(
    cubes: &Vec<Cube>,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let mut cube_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut cube_supports: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (u, &upper_cube) in cubes.iter().enumerate() {
        for (l, &lower_cube) in cubes[0..u].iter().enumerate() {
            if upper_cube.intersect(&lower_cube) && upper_cube.start.2 - lower_cube.end.2 == 1 {
                // lower supports upper
                cube_supports.entry(l).or_default().insert(u);
                cube_supported_by.entry(u).or_default().insert(l);
            }
        }
    }

    (cube_supported_by, cube_supports)
}
