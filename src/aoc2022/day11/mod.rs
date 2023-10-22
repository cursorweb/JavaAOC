use std::collections::VecDeque;

use crate::read;

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus(i64),
    Times(i64),

    /// old * old
    Square,
}

use Op::*;

#[derive(Debug, Clone, Copy)]
struct Test {
    div_by: i64,

    /// monkey to give to
    if_true: usize,
    /// monkey to give to
    if_false: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Op,
    test: Test,
    items_inspected: i64,
}

/*
Chinese Remainder Theorem
n1 ... nk
N = product n1 ... nk

For system
x == 0 mod (n1)
...
x == 0 mod (nk)

has solutions x1 and x2, x1 == x2 (mod N)

- in our case, x1 is big number, x2 is small number
- x1 % N == x2
*/

pub fn run() {
    let file = read!(str)
        .split("\n\n")
        .map(|monkey| monkey.split("\n").skip(1));

    let mut big_n = 1;

    let mut monkeys: Vec<Monkey> = file
        .map(|mut line| {
            let items: VecDeque<_> = line
                .next()
                .unwrap() // items: 1, 2
                .split_once(": ")
                .unwrap()
                .1 // 1, 2
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();

            let mut operation_line = line
                .next()
                .unwrap()
                .split_once("old ")
                .unwrap()
                .1 // * 19
                .split(" ");

            let operation = match operation_line.next().unwrap() {
                // interestingly only old * old pattern exist, old + old never exists
                // (foreshadowing to part 2...)
                "*" => match operation_line.next().unwrap().parse() {
                    Ok(n) => Times(n),
                    Err(_) => Square,
                },
                "+" => Plus(operation_line.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            };

            // divisible by ..
            let div_by = line
                .next()
                .unwrap()
                .split_once("by ")
                .unwrap()
                .1
                .parse()
                .unwrap();
            let if_true = line
                .next()
                .unwrap()
                .split_once("monkey ")
                .unwrap()
                .1
                .parse()
                .unwrap();
            let if_false = line
                .next()
                .unwrap()
                .split_once("monkey ")
                .unwrap()
                .1
                .parse()
                .unwrap();

            big_n *= div_by;

            Monkey {
                items,
                operation,
                test: Test {
                    div_by,
                    if_true,
                    if_false,
                },
                items_inspected: 0,
            }
        })
        .collect();

    let mut monkeys2: Vec<Monkey> = monkeys.clone();

    for _ in 0..20 {
        round(&mut monkeys, |num| num / 3);
    }

    for _ in 0..10_000 {
        round(&mut monkeys2, |num| num % big_n);
    }

    println!("Part1: {}", top_banana(monkeys));
    println!("Part2: {}", top_banana(monkeys2));
}

fn round(monkeys: &mut Vec<Monkey>, op: impl Fn(i64) -> i64) {
    for i in 0..monkeys.len() {
        loop {
            let monkey = &mut monkeys[i];
            let Some(mut item) = monkey.items.pop_front() else { break; };
            monkey.items_inspected += 1;

            match monkey.operation {
                Plus(n) => item += n,
                Times(n) => item *= n,
                Square => item *= item,
            }

            item = op(item);

            // make monkey immutable again
            // so we can mutate other monkeys
            let monkey = &monkeys[i];
            if item % monkey.test.div_by == 0 {
                let if_true = monkey.test.if_true;
                monkeys[if_true].items.push_back(item);
            } else {
                let if_false = monkey.test.if_false;
                monkeys[if_false].items.push_back(item);
            }
        }
    }
}

/// 😎🐒🍌
fn top_banana(mut monkeys: Vec<Monkey>) -> i64 {
    monkeys.sort_by(|m1, m2| m2.items_inspected.cmp(&m1.items_inspected));
    monkeys
        .iter()
        .take(2)
        .map(|m| m.items_inspected)
        .product::<i64>()
}
