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

- in our case, x1 and x2 are ITEMS
- modding N will mean they still satisfy x == 0 mod (n1)
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

    // part1 monkeys
    for _ in 0..20 {
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

                item /= 3;

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

    for _ in 0..10_000 {
        for i in 0..monkeys2.len() {
            loop {
                let monkey = &mut monkeys2[i];
                let Some(mut item) = monkey.items.pop_front() else { break; };
                monkey.items_inspected += 1;

                match monkey.operation {
                    Plus(n) => item += n,
                    Times(n) => item *= n,
                    Square => item *= item,
                }

                item %= big_n;

                // make monkey immutable again
                // so we can mutate other monkeys
                let monkey = &monkeys2[i];
                if item % monkey.test.div_by == 0 {
                    let if_true = monkey.test.if_true;
                    monkeys2[if_true].items.push_back(item);
                } else {
                    let if_false = monkey.test.if_false;
                    monkeys2[if_false].items.push_back(item);
                }
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.items_inspected.cmp(&m1.items_inspected));
    println!(
        "Part1: {}",
        monkeys
            .iter()
            .take(2)
            .map(|m| m.items_inspected)
            .product::<i64>()
    );

    monkeys2.sort_by(|m1, m2| m2.items_inspected.cmp(&m1.items_inspected));
    println!(
        "Part2: {}",
        monkeys2
            .iter()
            .take(2)
            .map(|m| m.items_inspected)
            .product::<i64>()
    );
}
