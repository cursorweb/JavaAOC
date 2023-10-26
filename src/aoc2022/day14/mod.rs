use std::collections::HashSet;

use crate::read;

pub fn run() {
    let file = read!();
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
                    blocks.push(block);
                }
            }

            blocks
        })
        .collect();

    dot(blocks);
}

fn dot(blocks: HashSet<(i32, i32)>) {
    // 9x10
    for y in 0..=9 {
        for x in 494..=503 {
            if x == 500 && y == 0 {
                print!("+");
            } else if blocks.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
