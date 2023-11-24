use std::{collections::HashSet, iter::Cycle, vec::IntoIter};

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

    for i in 0..1_000_000_000_000i64 {
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
    }

    println!("Part2: {}", highest_y(&stones) + 1);
}

fn _show_rock(rock: &Vec<(i64, i32)>, stones: &HashSet<(i64, i32)>) {
    let ceil = highest_y(rock);
    let floor = lowest_y(ceil, stones);
    for y in (floor..=ceil).rev() {
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
    let floor = lowest_y(ceil, stones);
    for y in (floor..=ceil).rev() {
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

fn optimize(stones: &mut HashSet<(i64, i32)>) {
    let ceil = highest_y(&*stones);
    let lowest_y = lowest_y(ceil, &*stones);

    let val = stones
        .iter()
        .filter(|(y, _)| *y >= lowest_y)
        .copied()
        .collect();
    *stones = val;
}

fn highest_y<'a, T>(rocks: T) -> i64
where
    T: IntoIterator<Item = &'a (i64, i32)>,
{
    rocks.into_iter().map(|(y, _)| *y).max().unwrap()
}

fn lowest_y(ceil: i64, stones: &HashSet<(i64, i32)>) -> i64 {
    let mut lowest_y = i64::MAX;
    for x in 0..7 {
        let mut y = ceil;
        while y > 0 {
            if stones.contains(&(y, x)) {
                lowest_y = lowest_y.min(y);
                break;
            }

            y -= 1;
        }

        if y == 0 {
            return 0;
        }
    }

    lowest_y
}
