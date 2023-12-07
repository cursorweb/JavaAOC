use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::read;

/*
Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
*/

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Kind {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

use Kind::*;

pub fn run() {
    let file = read!();
    let score1: HashMap<char, i32> = "A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2"
        .split(", ")
        .collect_vec()
        .into_iter()
        .rev() // wtf
        .enumerate()
        .map(|(i, x)| (x.chars().next().unwrap(), i as i32))
        .collect();

    let score2: HashMap<char, i32> = "A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J"
        .split(", ")
        .collect_vec()
        .into_iter()
        .rev() // wtf
        .enumerate()
        .map(|(i, x)| (x.chars().next().unwrap(), i as i32))
        .collect();

    let games = file
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let bid: i32 = bid.parse().unwrap();

            (hand, bid)
        })
        .collect_vec();

    let mut games1 = games
        .clone()
        .into_iter()
        .map(|(hand, bid)| {
            let hand = hand.chars().map(|x| score1[&x]).collect_vec();
            let hand_type = get_kind(&hand);

            (hand, hand_type, bid)
        })
        .collect_vec();

    games1.sort_by(|(hand1, hand_type1, _), (hand2, hand_type2, _)| {
        match hand_type2.cmp(&hand_type1) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                assert_eq!(5, hand1.len());
                let mut out = Ordering::Equal;
                for i in 0..5 {
                    let val1 = hand1[i];
                    let val2 = hand2[i];

                    if val1 != val2 {
                        // want a number...? idk
                        out = val1.cmp(&val2);
                        break;
                    }
                }

                assert_ne!(out, Ordering::Equal);
                out
            }
            Ordering::Greater => Ordering::Greater,
        }
    });

    let winnings: i32 = games1
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) as i32 * bid)
        .sum();

    println!("Part1: {winnings}");

    let mut games2 = games
        .clone()
        .into_iter()
        .map(|(hand, bid)| {
            let hand = hand.chars().map(|x| score2[&x]).collect_vec();
            let hand_type = j_kind(&hand);

            (hand, hand_type, bid)
        })
        .collect_vec();

    games2.sort_by(|(hand1, hand_type1, _), (hand2, hand_type2, _)| {
        match hand_type2.cmp(&hand_type1) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                assert_eq!(5, hand1.len());
                let mut out = Ordering::Equal;
                for i in 0..5 {
                    let val1 = hand1[i];
                    let val2 = hand2[i];

                    if val1 != val2 {
                        // want a number...? idk
                        out = val1.cmp(&val2);
                        break;
                    }
                }

                assert_ne!(out, Ordering::Equal);
                out
            }
            Ordering::Greater => Ordering::Greater,
        }
    });

    let winnings: i32 = games2
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) as i32 * bid)
        .sum();

    // 35150181
    println!("Part2: {winnings}");
}

fn five_kind(x: &Vec<i32>) -> bool {
    x.iter().filter(|&&n| n == x[0]).count() == 5
}

fn four_kind(x: &Vec<i32>) -> bool {
    let counts = get_counts(x);

    counts.values().contains(&4)
}

fn full_house(x: &Vec<i32>) -> bool {
    let counts = get_counts(x);
    let vals = counts.values().copied().collect_vec();

    vals.contains(&3) && vals.contains(&2)
}

fn three_kind(x: &Vec<i32>) -> bool {
    let counts = get_counts(x);
    let vals = counts.values().copied().collect_vec();

    vals.contains(&3) && !vals.contains(&2)
}

fn two_pair(x: &Vec<i32>) -> bool {
    let counts = get_counts(x);
    let mut pairs_two = 0;

    for &n in counts.values() {
        if n == 2 {
            pairs_two += 1;
        }
    }

    pairs_two == 2
}

fn one_pair(x: &Vec<i32>) -> bool {
    let counts = get_counts(x);
    let mut pairs_two = 0;

    for &n in counts.values() {
        if n == 2 {
            pairs_two += 1;
        }
    }

    pairs_two == 1
}

fn get_counts(x: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for &n in x {
        let entry = counts.entry(n).or_insert(0);
        *entry += 1;
    }

    counts
}

fn has_joker(x: &Vec<i32>) -> bool {
    x.contains(&0)
}

fn j_kind(x: &Vec<i32>) -> Kind {
    // it's all jokers
    if five_kind(x) {
        return Five;
    }

    if !has_joker(x) {
        get_kind(x)
    } else {
        let mut counts = get_counts(x);
        let jokers = counts.remove(&0).unwrap();
        let no_joke = x.clone().iter().copied().filter(|&x| x != 0).collect_vec();
        let mut max = High;
        // in ord, High is the biggest
        for &val in counts.keys() {
            let mut new_vec = no_joke.clone();
            new_vec.extend(std::iter::repeat(val).take(jokers as usize));
            if new_vec.len() == 5 {
                let val = get_kind(&new_vec);
                if val < max {
                    max = val;
                }
            }
        }

        assert_ne!(max, High);
        max
    }
}

fn get_kind(hand: &Vec<i32>) -> Kind {
    if five_kind(&hand) {
        Five
    } else if four_kind(&hand) {
        Four
    } else if full_house(&hand) {
        Full
    } else if three_kind(&hand) {
        Three
    } else if two_pair(&hand) {
        Two
    } else if one_pair(&hand) {
        One
    } else {
        High
    }
}

/// JXXXX
/// JJXXX
/// JJJXX
/// JJJJX
fn _j_five_kind(x: &Vec<i32>) -> bool {
    if five_kind(x) {
        true
    } else if has_joker(x) {
        let counts = get_counts(x);
        counts.values().len() == 2
    } else {
        false
    }
}

/// YJJJX
/// YJJXX
/// YJXXX
///
/// but not
/// YJXXY
fn _j_four_kind(x: &Vec<i32>) -> bool {
    if four_kind(x) {
        true
    } else if has_joker(x) {
        let mut counts = get_counts(x);
        let jokers = counts.remove(&0).unwrap();
        counts.values().find(|&&x| x + jokers == 4).is_some()
    } else {
        false
    }
}

/// YXJJY
/// YXXJY
/// JXXXY
fn _j_full_house(x: &Vec<i32>) -> bool {
    if full_house(x) {
        true
    } else if has_joker(x) {
        let mut counts = get_counts(x);
        let jokers = counts.remove(&0).unwrap();
        counts
            .values()
            .find(|&&x| x + jokers == 2 || x + jokers == 3)
            .is_some()
    } else {
        false
    }
}

/// YXXJZ
/// YXJJZ
fn _j_three_kind(x: &Vec<i32>) -> bool {
    if three_kind(x) {
        true
    } else if has_joker(x) {
        let mut counts = get_counts(x);
        let jokers = counts.remove(&0).unwrap();
        counts.values().len() == 3 && counts.values().find(|&&x| x + jokers == 3).is_some()
    } else {
        false
    }
}

/// XJYYZ
/// XXYJZ
fn _j_two_pair(x: &Vec<i32>) -> bool {
    if two_pair(x) {
        true
    } else if has_joker(x) {
        let mut counts = get_counts(x);
        let jokers = counts.remove(&0).unwrap();
        counts.values().len() == 3 && jokers == 1
    } else {
        false
    }
}

// XYZWJ
fn _j_one_pair(x: &Vec<i32>) -> bool {
    if one_pair(x) {
        true
    } else if has_joker(x) {
        true
    } else {
        false
    }
}
