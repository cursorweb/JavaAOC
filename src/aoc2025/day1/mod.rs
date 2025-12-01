use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let instrs: Vec<(char, i32)> = file
        .map(|x| {
            let chars: Vec<_> = x.chars().collect();
            (
                chars[0],
                (chars[1..].iter().collect::<String>()).parse().unwrap(),
            )
        })
        .collect_vec();

    let mut password2 = 0;
    let mut password1 = 0;

    let mut num = 50;
    for (a, b) in instrs {
        if a == 'R' {
            password2 += (num + b) / 100;
        } else {
            password2 += (100 - num + b) / 100 - (100 - num) / 100;
        }

        num = (if a == 'R' { num + b } else { num - b }).rem_euclid(100);

        if num == 0 {
            password1 += 1;
        }
    }

    println!("Part1: {password1}");
    println!("Part2: {password2}");
}
