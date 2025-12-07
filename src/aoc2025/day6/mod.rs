use itertools::Itertools;

use crate::read;

#[derive(Clone, Copy, Debug)]
enum Op {
    Plus,
    Times,
}

use Op::*;

pub fn run() {
    let file = read!().collect_vec();

    let numbers = file[..file.len() - 1]
        .iter()
        .map(|x| {
            x.split(" ")
                .filter_map(|x| x.parse::<i64>().ok())
                .collect_vec()
        })
        .collect_vec();

    let numbers = transpose(numbers);

    let operators = file[file.len() - 1]
        .split(' ')
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(match x {
                    "+" => Plus,
                    "*" => Times,
                    _ => unreachable!(),
                })
            }
        })
        .collect_vec();

    let mut part1 = 0;
    for (problem, &op) in numbers.iter().zip(&operators) {
        let mut start = problem[0];
        for num in &problem[1..] {
            match op {
                Plus => start += num,
                Times => start *= num,
            }
        }

        part1 += start;
    }

    println!("Part1: {part1}");

    let file2 = transpose(
        file.iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec(),
    );

    let mut problems: Vec<Vec<i64>> = vec![];
    let mut ops = vec![];
    for line in file2 {
        if let Some(&x) = line.last() {
            if x != ' ' {
                match x {
                    '+' => ops.push(Plus),
                    '*' => ops.push(Times),
                    _ => (),
                }

                problems.push(vec![]);
            }
        }

        if let Ok(x) = String::from_iter(&line[..line.len() - 1]).trim().parse() {
            problems.last_mut().unwrap().push(x);
        }
    }

    let mut part2 = 0;
    for (problem, &op) in problems.iter().zip(&operators) {
        let mut start = problem[0];
        for num in &problem[1..] {
            match op {
                Plus => start += num,
                Times => start *= num,
            }
        }

        part2 += start;
    }

    println!("Part2: {part2}");
}

fn transpose<T: Copy>(x: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut out = vec![];
    for col in 0..x[0].len() {
        let mut new_col = vec![];
        for row in 0..x.len() {
            new_col.push(x[row][col]);
        }
        out.push(new_col);
    }

    out
}
