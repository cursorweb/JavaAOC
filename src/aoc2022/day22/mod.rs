use itertools::Itertools;

use crate::read;

/// Map Slice
struct MSlice {
    offset: i32,
    array: Vec<char>,
}

pub fn run() {
    let file = read!(str);
    let (map, path) = file.split_once("\n\n").unwrap();

    map.split("\n").map(|line| {
        let chars = line.chars().collect_vec();
        let mut offset = 0;
    });
}
