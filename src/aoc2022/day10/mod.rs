use crate::read;

#[derive(Debug)]
enum Op {
    /// 1 cycle
    Noop,

    /// 2 cycle
    Addx(i32),
}

use Op::*;

pub fn run() {
    let file = read!();

    let ops = file.map(|line| {
        if line == "noop" {
            Noop
        } else {
            // "addx" " " <num>
            let num = line.split(" ").skip(1).next().unwrap().parse().unwrap();
            Addx(num)
        }
    });

    let mut target_cycle = 20; // + 40
    let mut cycle = 0;

    // register x
    let mut rx = 1;
    let mut signal_strengths = 0;
    let mut display = String::new();

    let mut incr_cycle = |rx| {
        cycle += 1;

        let cycle_pos = (cycle - 1) % 40; // screen coordinates
        let sprite_pos = (rx - 1)..=(rx + 1);

        if sprite_pos.contains(&cycle_pos) {
            display += "#";
        } else {
            display += ".";
        }

        if cycle % 40 == 0 {
            display += "\n";
        }

        if cycle == target_cycle {
            signal_strengths += cycle * rx;
            target_cycle += 40;
        }
    };

    for op in ops {
        match op {
            Noop => incr_cycle(rx),
            Addx(n) => {
                incr_cycle(rx);
                incr_cycle(rx);
                rx += n;
            }
        }
    }

    println!("Part1: {signal_strengths}");
    println!("Part2:\n{display}");
}
