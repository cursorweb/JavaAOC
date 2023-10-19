use crate::{read, DIRS};

pub fn run() {
    let file = read!();

    let trees: Vec<Vec<i32>> = file
        .map(|row| row.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect();

    let height = trees.len();
    let width = trees[0].len();

    let mut count = 0;
    let mut highest_score = -1;

    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let tree_height = trees[row][col];
            let mut is_visible = false;
            let mut scenic_score = 1;

            for (dy, dx) in DIRS {
                let mut k = 1;
                loop {
                    let offset = |p, d| (p as i32 + d * k) as usize;

                    // direction is BLOCKED
                    if trees[offset(row, dy)][offset(col, dx)] >= tree_height {
                        break;
                    }

                    // we reached a border
                    // it's visible!
                    if offset(row, dy) == height - 1
                        || offset(col, dx) == width - 1
                        || offset(row, dy) == 0
                        || offset(col, dx) == 0
                    {
                        if !is_visible {
                            is_visible = true;
                            count += 1;
                        }

                        break;
                    }

                    k += 1;
                }

                scenic_score *= k;
            }

            if is_visible {
                if scenic_score > highest_score {
                    highest_score = scenic_score;
                }
            }
        }
    }

    count += 2 * width + 2 * height - 4; // perimeter

    println!("Part1: {count}");
    println!("Part2: {highest_score}");
}
