use crate::read;

#[derive(Debug)]
enum Ops {
    Insert(usize, (String, i32)),
    Remove(usize, String),
}

use itertools::Itertools;
use Ops::*;

pub fn run() {
    let file = read!(str).trim();
    let part1: i32 = file
        .split(",")
        .map(|code| {
            let mut start = 0;
            for char in code.chars() {
                start += char as i32;
                start *= 17;
                start %= 256;
            }
            start
        })
        .sum();

    println!("Part1: {part1}");

    let mut hashmap = vec![];
    for _ in 0..256 {
        hashmap.push(Vec::<(String, i32)>::new());
    }

    let instrs: Vec<Ops> = file
        .split(",")
        .map(|code| {
            let mut box_num = 0;
            let mut name = String::new();
            let mut num = String::new();
            let mut is_remove = true;

            for char in code.chars() {
                if char == '-' {
                    break;
                } else if char == '=' {
                    is_remove = false;
                    continue;
                }

                if is_remove {
                    // is_remove also guarantees label
                    box_num += char as i32;
                    box_num *= 17;
                    box_num %= 256;
                }

                if !is_remove {
                    num.push(char);
                } else {
                    name.push(char);
                }
            }

            if code.contains('-') {
                Remove(box_num as usize, name.to_string())
            } else {
                Insert(box_num as usize, (name.to_string(), num.parse().unwrap()))
            }
        })
        .collect_vec();

    for instr in instrs {
        match instr {
            Insert(idx, (name, num)) => {
                if let Some(index) = hashmap[idx].iter().position(|(x, _)| x == &name) {
                    hashmap[idx][index] = (name, num);
                } else {
                    hashmap[idx].push((name, num));
                }
            }
            Remove(idx, name) => {
                if let Some(index) = hashmap[idx].iter().position(|(x, _)| x == &name) {
                    hashmap[idx].remove(index);
                }
            }
        }
    }

    let mut sum = 0;
    for (box_i, vec) in hashmap.into_iter().enumerate() {
        let box_i = box_i as i32 + 1;
        sum += vec
            .into_iter()
            .enumerate()
            .map(|(j, (_, val))| box_i * (j as i32 + 1) * val)
            .sum::<i32>();
    }
    println!("Part2: {sum}");
}
