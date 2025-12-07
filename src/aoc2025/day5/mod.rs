use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::read;

pub fn run() {
    let (ranges, items) = read!(str).split_once("\n\n").unwrap();

    let mut ranges = ranges
        .split("\n")
        .map(|line| {
            let (a, b) = line
                .split("-")
                .map(|x| x.parse().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap();

            a..=b
        })
        .collect_vec();

    let items: Vec<i64> = items.split("\n").map(|x| x.parse().unwrap()).collect_vec();

    let part1 = items
        .iter()
        .filter(|&number| {
            for range in &ranges {
                if range.contains(number) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("Part1: {part1}");

    loop {
        let mut changed = false;

        // check two ranges, if they intersect, merge them
        // repeat until no more ranges intersect
        'end: for i in (0..ranges.len()).rev() {
            for j in (0..i).rev() {
                let r1 = &ranges[i];
                let r2 = &ranges[j];
                if range_intersects(r1, r2) {
                    ranges.push(merge_range(r1, r2));
                    ranges.remove(i);
                    ranges.remove(j);
                    changed = true;
                    break 'end;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let part2: usize = ranges.into_iter().map(|x| x.count()).sum();

    println!("Part2: {part2}");
}

fn merge_range(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    let start = *r1.start().min(r2.start());
    let end = *r1.end().max(r2.end());

    start..=end
}

fn range_intersects(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> bool {
    crate::range_intersects((r1.start(), r1.end()), (r2.start(), r2.end()))
}
