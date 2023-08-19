use std::collections::HashSet;

use crate::read;

pub fn run() {
    let file = read!(str);
    let slice = file.as_bytes();
    for (i, window) in slice.windows(4).enumerate() {
        if hashset(window).len() == 4 {
            println!("Part1: {}", i + 4);
            break;
        }
    }

    for (i, window) in slice.windows(14).enumerate() {
        if hashset(window).len() == 14 {
            println!("Part1: {}", i + 14);
            break;
        }
    }
}

fn hashset(itm: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(itm.iter().cloned())
}
