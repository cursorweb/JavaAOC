use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{dot, read};

#[derive(Debug, Clone, Copy)]
/// All coords are (x, y)
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

const WIDTH: i32 = 101; //11
const HEIGHT: i32 = 103; //7;

// superposition: x and y act independently so you just add and loop lmao
pub fn run() {
    let file = read!();
    let robots = file
        .map(|line| {
            let (pos, vel) = line.split_once(" ").unwrap();

            // p=0,4 -> 0,4
            let pos = pos.split_once("=").unwrap().1;
            let (px, py) = pos.split_once(",").unwrap();

            let vel = vel.split_once("=").unwrap().1;
            let (vx, vy) = vel.split_once(",").unwrap();

            Robot {
                pos: (px.parse().unwrap(), py.parse().unwrap()),
                vel: (vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect_vec();

    let mut poses: HashMap<(i32, i32), i32> = HashMap::new();
    for robot in &robots {
        *poses.entry(simulate(robot)).or_default() += 1;
    }

    let mut quadrants = vec![0; 4];

    for (&(x, y), &sum) in &poses {
        if x == WIDTH / 2 || y == HEIGHT / 2 {
            continue;
        }

        // XY -> 00 01 10 11
        // 0 1 2 3
        let x = ((x < WIDTH / 2) as usize) << 1;
        let y = (y < HEIGHT / 2) as usize;

        quadrants[x | y] += sum;
    }

    let mut robots = robots;
    'l: for i in 1.. {
        // they all need to be in unique position
        let mut poses = HashSet::new();
        for robot in &mut robots {
            let r = simulate_once(robot);
            poses.insert(r);
        }

        if poses.len() != robots.len() {
            continue 'l;
        }

        dot!(
            vec![vec!['.'; WIDTH as usize]; HEIGHT as usize],
            |x, y, c| {
                if poses.contains(&(y, x)) {
                    '#'
                } else {
                    c
                }
            }
        );

        println!("Part1: {}", quadrants.iter().product::<i32>());
        println!("Part2: {i}");
        break;
    }
}

fn simulate_once(robot: &mut Robot) -> (i32, i32) {
    let (px, py) = &mut robot.pos;
    let (vx, vy) = robot.vel;

    *px = (*px + vx).rem_euclid(WIDTH);
    *py = (*py + vy).rem_euclid(HEIGHT);

    (*px, *py)
}

fn simulate(robot: &Robot) -> (i32, i32) {
    let times = 100;
    let (mut px, mut py) = robot.pos;
    let (vx, vy) = robot.vel;

    px = (px + vx * times).rem_euclid(WIDTH);
    py = (py + vy * times).rem_euclid(HEIGHT);

    (px, py)
}
