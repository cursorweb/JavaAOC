use std::collections::HashSet;

use crate::read;

pub fn run() {
    let file = read!();

    let mut highest_y = i32::MIN;
    let start = (0, 500);

    let blocks: HashSet<(i32, i32)> = file
        .flat_map(|line| {
            let pairs = line.split(" -> ");
            let pairs: Vec<(i32, i32)> = pairs
                .map(|pair| {
                    let mut pair = pair.split(",");
                    (
                        pair.next().unwrap().parse().unwrap(),
                        pair.next().unwrap().parse().unwrap(),
                    )
                })
                .collect();

            let mut blocks = Vec::new();

            // &[(x, y)]
            for pair in pairs.windows(2) {
                let dx = pair[1].0 - pair[0].0;
                let dy = pair[1].1 - pair[0].1;

                // x+ is right
                let sdx = dx.signum();
                // y+ is down
                let sdy = dy.signum();

                let delta = dx.abs().max(dy.abs());

                // convert to (y, x)
                let mut block = (pair[0].1, pair[0].0);

                blocks.push(block); // add the block
                for _ in 0..delta {
                    // move it delta times
                    block.0 += sdy;
                    block.1 += sdx;

                    if block.0 > highest_y {
                        highest_y = block.0;
                    }

                    blocks.push(block);
                }
            }

            blocks
        })
        .collect();

    let floor = highest_y + 2;

    let get_next_pos = |sand: (i32, i32), sands: &mut HashSet<_>| {
        let mut next_pos = (sand.0 + 1, sand.1);

        if sands.contains(&next_pos) || blocks.contains(&next_pos) {
            // left
            let pos = (sand.0 + 1, sand.1 - 1);
            if !sands.contains(&pos) && !blocks.contains(&pos) {
                next_pos = pos;
            } else {
                // right
                let pos = (sand.0 + 1, sand.1 + 1);
                if !sands.contains(&pos) && !blocks.contains(&pos) {
                    next_pos = pos;
                } else {
                    // you must be on top of a sand with no where to go
                    sands.insert(sand);
                    return None;
                }
            }
        }

        // you reached the bottom!
        if next_pos.0 == floor {
            sands.insert(sand);
            return None;
        }

        return Some(next_pos);
    };

    let mut sands = HashSet::new();

    loop {
        // (y, x)
        let mut sand = start;
        while sand.0 <= highest_y {
            let Some(next_pos) = get_next_pos(sand, &mut sands) else { break; };
            sand = next_pos;
        }

        // went into the 'abyss'
        if sand.0 > highest_y {
            break;
        }
    }

    println!("Part1: {}", sands.len());

    let mut sands = HashSet::new();

    loop {
        let mut sand: (i32, i32) = start;
        loop {
            let Some(next_pos) = get_next_pos(sand, &mut sands) else { break; };
            sand = next_pos;
        }

        // blocked off!
        if sand == start {
            break;
        }
    }

    println!("Part2: {}", sands.len());
}

fn _dot(blocks: &HashSet<(i32, i32)>, sands: &HashSet<(i32, i32)>) {
    // 9x10
    for y in 0..=11 {
        for x in 494 - 6..=503 + 6 {
            if x == 500 && y == 0 {
                print!("+");
            } else if sands.contains(&(y, x)) {
                print!("o");
            } else if blocks.contains(&(y, x)) {
                print!("#");
            } else if y == 11 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
