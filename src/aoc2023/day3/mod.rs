use std::collections::{HashMap, HashSet};

use crate::read;

#[derive(Debug, Clone, Copy)]
struct Number {
    len: i32,
    num: i32,
}

pub fn run() {
    let file = read!();

    // (y, x)
    let mut symbols = HashSet::new();
    let mut gears = HashSet::new();
    let mut numbers = HashMap::new();

    for (y, line) in file.enumerate() {
        let mut num = String::new();
        let mut nx = -1;
        let chars = line.chars();

        for (x, char) in chars.enumerate() {
            if char.is_numeric() {
                if nx == -1 {
                    nx = x as i32;
                }
                num.push(char);
            } else {
                if !num.is_empty() {
                    numbers.insert(
                        (y as i32, nx),
                        Number {
                            len: num.len() as i32,
                            num: num.parse().unwrap(),
                        },
                    );
                    nx = -1;
                    num = String::new();
                }

                if char == '*' {
                    gears.insert((y as i32, x as i32));
                }

                if char != '.' {
                    symbols.insert((y as i32, x as i32));
                }
            }
        }

        // at the end
        if !num.is_empty() {
            numbers.insert(
                (y as i32, nx),
                Number {
                    len: num.len() as i32,
                    num: num.parse().unwrap(),
                },
            );
        }
    }

    let mut touch = Vec::new();

    // coord, vec
    let mut gear_pairs = HashMap::new();

    for (&(y, x), &number) in numbers.iter() {
        // the x in range
        // the y diff is <= 1
        //  x___
        // .1234
        // ******
        // index = 1 (len 4) (5)
        // symbol at 0
        // symbol at 5

        for &(sy, sx) in &symbols {
            if (y - sy).abs() <= 1 && (sx <= x + number.len && sx >= x - 1) {
                if gears.contains(&(sy, sx)) {
                    let entry: &mut HashSet<i32> =
                        gear_pairs.entry((sy, sx)).or_insert(HashSet::new());
                    entry.insert(number.num);
                }
                touch.push(number.num);
                break;
            }
        }
    }

    println!("Part1: {}", touch.iter().sum::<i32>());

    println!(
        "Part2: {}",
        gear_pairs
            .values()
            .filter_map(|set| if set.len() == 2 {
                Some(set.iter().product::<i32>())
            } else {
                None
            })
            .sum::<i32>()
    );
}

fn _dot(symbols: &HashSet<(i32, i32)>, numbers: &HashMap<(i32, i32), Number>) {
    for y in 0..10 {
        let mut x = 0;
        while x < 10 {
            if symbols.contains(&(y, x)) {
                print!("$");
            } else if numbers.contains_key(&(y, x)) {
                print!("{}", numbers[&(y, x)].num);
                x += numbers[&(y, x)].len;
                continue;
            } else {
                print!(".");
            }
            x += 1;
        }
        println!();
    }
}
