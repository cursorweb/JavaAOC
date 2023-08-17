use crate::read;

pub fn run() {
    let file = read!();
    let (part1, part2) = file.fold((0, 0), |(acc1, acc2), curr| {
        let mut pair_iter = curr.split(",").map(|p| Pair::from(p));
        let left = pair_iter.next().unwrap();
        let right = pair_iter.next().unwrap();

        (
            acc1 + if left.contains(&right) { 1 } else { 0 },
            acc2 + if left.intersects(&right) { 1 } else { 0 },
        )
    });

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

struct Pair {
    start: i32,
    end: i32,
}

impl Pair {
    fn from(pair: &str) -> Self {
        let mut pair_iter = pair.split("-").map(|n| n.parse().unwrap());
        Self {
            start: pair_iter.next().unwrap(),
            end: pair_iter.next().unwrap(),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (self.start >= other.start && self.end <= other.end)
    }

    fn intersects(&self, other: &Self) -> bool {
        (self.start <= other.end) && (self.end >= other.start)
    }
}
