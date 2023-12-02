use itertools::Itertools;

use crate::read;

#[derive(Debug, Clone, Copy)]
enum Color {
    Red(i32),
    Green(i32),
    Blue(i32),
}

use Color::*;

pub fn run() {
    let file = read!();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let games = file
        .map(|line| {
            let shown = line.split_once(": ").unwrap().1;
            shown
                .split("; ")
                .map(|shown| {
                    shown
                        .split(", ")
                        .map(|cube| {
                            let (number, color) = cube.split_once(" ").unwrap();
                            let number = number.parse().unwrap();
                            match color {
                                "red" => Red(number),
                                "blue" => Blue(number),
                                "green" => Green(number),
                                _ => unreachable!("{color}"),
                            }
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let valid_games: usize = games
        .iter()
        .enumerate()
        .filter_map(|(i, shown)| {
            for sets in shown {
                for &cube in sets {
                    match cube {
                        Red(n) => {
                            if n > max_red {
                                return None;
                            }
                        }
                        Green(n) => {
                            if n > max_green {
                                return None;
                            }
                        }
                        Blue(n) => {
                            if n > max_blue {
                                return None;
                            }
                        }
                    }
                }
            }

            return Some(i + 1);
        })
        .sum();

    let min_games: i32 = games
        .iter()
        .map(|shown| {
            let mut max_blue = 0;
            let mut max_green = 0;
            let mut max_red = 0;

            for set in shown {
                for &cube in set {
                    match cube {
                        Red(n) => max_red = max_red.max(n),
                        Green(n) => max_green = max_green.max(n),
                        Blue(n) => max_blue = max_blue.max(n),
                    }
                }
            }

            max_blue * max_green * max_red
        })
        .sum();

    println!("Part1: {}", valid_games);
    println!("Part2: {}", min_games);
}
