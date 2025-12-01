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
    for (d, delta) in instrs {
        if d == 'R' {
            password2 += (num + delta) / 100;
        } else {
            // num - delta is like -18 or something, so when you add 100, you get 118
            // next, you subtract the original, 100 - num amt
            password2 += (100 - (num - delta)) / 100 - (100 - num) / 100;
        }

        num = (if d == 'R' { num + delta } else { num - delta }).rem_euclid(100);

        if num == 0 {
            password1 += 1;
        }
    }

    println!("Part1: {password1}");
    println!("Part2: {password2}");
}
