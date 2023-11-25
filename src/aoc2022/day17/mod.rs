/*
1984 1247 1247
     |||| ---- is the same as
     ----     this
How are they the same? The last slice = this slice
Find that, and then, the max height will be: the repetition
and then a little more
*/
use std::{
    collections::{HashMap, HashSet},
    iter::Cycle,
    vec::IntoIter,
};

/// how much to store in our pattern checker
const LOOK_BEHIND: i64 = 40i64;
const TOTAL_STACK: i64 = 1_000_000_000_000;

use crate::read;

// (0, 0) is the top most leftmost block
// (y, x), y-- = down
fn create_map() -> Vec<Vec<(i64, i32)>> {
    /*
    [
        [
            .#.
            ###
            .#.
        ],
        ...
    ]
    */
    let text: Vec<Vec<Vec<char>>> = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"
    .split("\n\n")
    .map(|block| {
        block
            .split("\n")
            .map(|line| line.chars().collect())
            .collect()
    })
    .collect();

    let cycle: Vec<Vec<(i64, i32)>> = text
        .into_iter()
        .map(|block| {
            let mut out = Vec::new();

            // since y goes down, bottommost one should be at y=0
            // in the array, the bottommost one is last, so w/o rev
            // it would be at y=len (in other words the whole thing is upside down)
            for (y, row) in block.into_iter().rev().enumerate() {
                for (x, c) in row.into_iter().enumerate() {
                    if c == '#' {
                        // blocks start 2 from the edge (y=0)
                        // 3 above the floor
                        out.push((y as i64, x as i32 + 2));
                    }
                }
            }

            out
        })
        .collect();

    cycle
}

pub fn run() {
    let file = read!(str).chars();

    // [1, -1, ...]
    let push: Vec<i32> = file
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => unreachable!("{c}"),
        })
        .collect();
    let mut push_cycle = push.into_iter().cycle();

    let mut rock_cycle = create_map().into_iter().cycle();
    let mut stones = HashSet::new();
    let mut floor = 0;

    let mut rock = create_rock(&mut rock_cycle, floor);

    // last N : (iter number, height)
    let mut cache: HashMap<String, (i64, i64)> = HashMap::new();

    let mut remaining = 0;
    // as after the prelim pattern checking loop
    // you need to subtract all that height
    // to get the true 'remainder' delta
    let mut curr_height = 0;
    let mut rest_height = 0;

    for i in 0..LOOK_BEHIND * 100 {
        // just hope 2022 was in it LMAO
        if i == 2022 {
            // floor starts at 0, but we want the height
            println!("Part1: {}", highest_y(&stones) + 1);
        }

        let mut should_fall = false;
        loop {
            if !should_fall {
                let dir = push_cycle.next().unwrap();
                rock = push_by(rock, &stones, dir);
            } else {
                match fall(rock, floor, &stones) {
                    Ok(r) => rock = r,
                    Err(r) => {
                        // the highest y is the highest of the stone
                        // so add 1 = the floor
                        floor = floor.max(highest_y(&r) + 1);
                        stones.extend(r);
                        rock = create_rock(&mut rock_cycle, floor);
                        break;
                    }
                };
            }
            should_fall = !should_fall;
        }

        if cache.contains_key(&dotstring(&stones)) {
            // last N : (iter number, height)
            let val = cache[&dotstring(&stones)];
            let height = highest_y(&stones) + 1;

            // how many more iterations to go
            let rest = TOTAL_STACK - i;
            // how long the iteration lasts
            let skip = i - val.0;
            // how long the pattern repeats
            let products = rest / skip;
            // how much the height increases
            let amt = height - val.1;

            remaining = rest % skip;
            curr_height = height;
            rest_height = height + products * amt;

            break;
        }

        if i > LOOK_BEHIND {
            cache.insert(dotstring(&stones), (i, highest_y(&stones) + 1));
        }
    }

    for _ in 0..remaining {
        let mut should_fall = false;
        loop {
            if !should_fall {
                let dir = push_cycle.next().unwrap();
                rock = push_by(rock, &stones, dir);
            } else {
                match fall(rock, floor, &stones) {
                    Ok(r) => rock = r,
                    Err(r) => {
                        // the highest y is the highest of the stone
                        // so add 1 = the floor
                        floor = floor.max(highest_y(&r) + 1);
                        stones.extend(r);
                        rock = create_rock(&mut rock_cycle, floor);
                        break;
                    }
                };
            }
            should_fall = !should_fall;
        }
    }

    // (max_height + 1) - (max + 1) + (max + 1) - 1
    println!("Part2: {}", highest_y(&stones) - curr_height + rest_height);
}

fn _show_rock(rock: &Vec<(i64, i32)>, stones: &HashSet<(i64, i32)>) {
    let ceil = highest_y(rock);
    for y in (0..=ceil).rev() {
        for x in 0..7 {
            if stones.contains(&(y, x)) {
                print!("#")
            } else if rock.contains(&(y, x)) {
                print!("@")
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}

fn _dot(stones: &HashSet<(i64, i32)>) {
    let ceil = highest_y(stones);
    for y in (0..=ceil).rev() {
        for x in 0..7 {
            if stones.contains(&(y, x)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

/// dot actually becomes not debugging?! :O
fn dotstring(stones: &HashSet<(i64, i32)>) -> String {
    let mut out = String::new();
    let ceil = highest_y(stones);
    for y in (ceil - LOOK_BEHIND..=ceil).rev() {
        for x in 0..7 {
            if stones.contains(&(y, x)) {
                out += "#";
            } else {
                out += ".";
            }
        }
        out += "\n";
    }
    out
}

fn create_rock(rock: &mut Cycle<IntoIter<Vec<(i64, i32)>>>, floor: i64) -> Vec<(i64, i32)> {
    rock.next()
        .unwrap()
        .into_iter()
        .map(|(y, x)| (y + 3 + floor, x))
        .collect()
}

fn push_by(rock: Vec<(i64, i32)>, stones: &HashSet<(i64, i32)>, dir: i32) -> Vec<(i64, i32)> {
    for &(y, x) in &rock {
        if x + dir >= 7 || x + dir < 0 || stones.contains(&(y, x + dir)) {
            return rock;
        }
    }

    rock.into_iter().map(|(y, x)| (y, x + dir)).collect()
}

/// goofy ahh type system
fn fall(
    rock: Vec<(i64, i32)>,
    floor: i64,
    stones: &HashSet<(i64, i32)>,
) -> Result<Vec<(i64, i32)>, Vec<(i64, i32)>> {
    for &(y, x) in &rock {
        if stones.contains(&(y - 1, x)) || (y - 1 < floor && floor == 0) {
            return Err(rock);
        }
    }

    Ok(rock.into_iter().map(|(y, x)| (y - 1, x)).collect())
}

fn highest_y<'a, T>(rocks: T) -> i64
where
    T: IntoIterator<Item = &'a (i64, i32)>,
{
    rocks.into_iter().map(|(y, _)| *y).max().unwrap()
}
