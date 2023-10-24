use std::{cmp::Ordering, fmt::Debug};

use crate::read;

#[derive(PartialEq, Eq, Clone)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::List(arg0) => f.write_fmt(format_args!("{:?}", arg0)),
        }
    }
}

use Item::*;

pub fn run() {
    let file = read!(str).split("\n\n");
    let packets: Vec<(Vec<Item>, Vec<Item>)> = file
        .map(|line| {
            let (left, right) = line.split_once("\n").unwrap();
            let (left, right) = (parse_array(left), parse_array(right));

            (left, right)
        })
        .collect();

    let sum_indices: usize = packets
        .iter()
        .map(|(left, right)| in_order(left, right, 1))
        .enumerate()
        .filter_map(|(i, o)| {
            if o != Ordering::Less {
                None
            } else {
                Some(i + 1)
            }
        })
        .sum();

    println!("Part1: {sum_indices}");

    let mut packets: Vec<Vec<Item>> = packets
        .into_iter()
        .flat_map(|(left, right)| vec![left, right])
        .collect();

    let divider2 = vec![List(vec![Int(2)])];
    let divider6 = vec![List(vec![Int(6)])];
    packets.push(divider2.clone());
    packets.push(divider6.clone());

    packets.sort_by(|left, right| in_order(left, right, 0));

    let mut idx = 1;
    let mut divider2_idx = 0;
    let mut divider6_idx = 0;
    for itm in packets {
        if itm == divider2 {
            divider2_idx = idx;
        }
        if itm == divider6 {
            divider6_idx = idx;
        }

        idx += 1;
    }

    println!("Part2: {}", divider2_idx * divider6_idx);
}

/// ordering should be less than
fn in_order(left: &Vec<Item>, right: &Vec<Item>, idt: usize) -> Ordering {
    let left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    for left in left_iter {
        let right = match right_iter.next() {
            Some(right) => right,
            // right should not run out of items
            None => {
                // println!("{}Right ran out of items", "\t".repeat(idt));
                return Ordering::Greater;
            }
        };
        match (left, right) {
            (Int(left), Int(right)) => {
                // println!("{}Compare {left} vs {right}", "\t".repeat(idt));
                if left > right {
                    // println!("{}Left greater", "\t".repeat(idt + 1));
                    return Ordering::Greater;
                } else if left < right {
                    // println!("{}Left smaller (right order)", "\t".repeat(idt + 1));
                    return Ordering::Less;
                }
            }
            (List(left), List(right)) => {
                // println!("{}Comparing {left:?} vs {right:?}", "\t".repeat(idt));
                let val = in_order(left, right, idt + 1);
                if val != Ordering::Equal {
                    return val;
                }
            }
            (&Int(left), List(right)) => {
                // println!("{}Comparing [ {left:?} ] vs {right:?}", "\t".repeat(idt));
                let val = in_order(&vec![Int(left)], right, idt + 1);
                if val != Ordering::Equal {
                    return val;
                }
            }
            (List(left), &Int(right)) => {
                // println!("{}Comparing {left:?} vs [ {right} ]", "\t".repeat(idt));
                let val = in_order(left, &vec![Int(right)], idt + 1);
                if val != Ordering::Equal {
                    return val;
                }
            }
        }
    }

    if right_iter.next().is_none() {
        // right same length as left
        // println!("{}Right same length as left", "\t".repeat(idt));
        Ordering::Equal
    } else {
        // left ran out of items
        // println!("{}Left ran out of items", "\t".repeat(idt));
        Ordering::Less
    }
}

fn parse_array(text: &str) -> Vec<Item> {
    // This would have come from '[]' -> ''
    if text.is_empty() {
        return Vec::new();
    }
    // remove the '[' and ']'
    let items = text[1..text.len() - 1].chars();

    let mut out = Vec::new();

    // what is currently being read (either to be parsed or ...)
    let mut curr = String::new();
    let mut nest = 0;

    let mut parse_curr = |curr: &mut String| match curr.parse() {
        Ok(num) => {
            out.push(Int(num));
        }
        Err(_) => {
            out.push(List(parse_array(&curr)));
        }
    };

    for item in items {
        if item == ',' && nest == 0 {
            parse_curr(&mut curr);
            curr.clear();
            continue;
        }

        if item == '[' {
            nest += 1;
        }

        if item == ']' {
            nest -= 1;
        }

        curr.push(item);
    }

    parse_curr(&mut curr);

    out
}
