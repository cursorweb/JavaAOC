use itertools::Itertools;

use crate::read;

/// Map Slice
/// ```txt
/// localx = x - offset
/// realx = localx + offset
/// next localx = realx - offset
/// ```
#[derive(Debug)]
struct MSlice {
    offset: i32,
    slice: Vec<char>,
}

pub fn run() {
    let file = read!(str);
    let (map, path) = file.split_once("\n\n").unwrap();

    let map = map
        .split("\n")
        .map(|line| {
            let line_len = line.len();
            let path = line.trim_start();

            let offset = (line_len - path.len()) as i32;

            let slice = path.chars().collect_vec();

            MSlice { offset, slice }
        })
        .collect_vec();
}
