use crate::read;

pub fn run() {
    let file = read!();
    let mut lines = file.peekable();

    // cool equation: 2 + x + 3(x - 1) = length
    let buckets = (lines.peek().unwrap().len() + 1) / 4;
    let mut buckets = vec![Vec::new(); buckets];

    // " 1" will start the labels which we DON'T need.
    for line in lines.by_ref().take_while(|&line| !line.starts_with(" 1")) {
        // [A] [B] [C]
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                buckets[i].insert(0, c);
            }
        }
    }

    let lines = lines.skip(1); // ""

    let mut part1_bucket = buckets.clone();
    let mut part2_bucket = buckets;

    // move %d from %d to %d
    for instr in lines.map(|line| {
        let iter = line
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<usize>().unwrap());
        Instr::from(iter)
    }) {
        instr.part1(&mut part1_bucket);
        instr.part2(&mut part2_bucket);
    }

    let part1: String = last(part1_bucket);
    let part2: String = last(part2_bucket);

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

fn last(vec: Vec<Vec<char>>) -> String {
    vec.iter().map(|b| b.last().unwrap()).collect()
}

#[derive(Debug)]
struct Instr {
    count: usize,
    from: usize,
    to: usize,
}

impl Instr {
    fn from(mut iter: impl Iterator<Item = usize>) -> Self {
        Self {
            count: iter.next().unwrap(),
            // label is 1 2 3 4 ...
            from: iter.next().unwrap() - 1,
            to: iter.next().unwrap() - 1,
        }
    }

    fn part1(&self, buckets: &mut Vec<Vec<char>>) {
        let from_idx = &mut buckets[self.from];
        let mut tail = from_idx.split_off(from_idx.len() - self.count);
        tail.reverse();

        let to_idx = &mut buckets[self.to];
        to_idx.append(&mut tail);
    }

    fn part2(&self, buckets: &mut Vec<Vec<char>>) {
        let from_idx = &mut buckets[self.from];
        let mut tail = from_idx.split_off(from_idx.len() - self.count);

        let to_idx = &mut buckets[self.to];
        to_idx.append(&mut tail);
    }
}
