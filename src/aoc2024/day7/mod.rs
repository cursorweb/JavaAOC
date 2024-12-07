use crate::read;

pub fn run() {
    let file = read!();
    let lines: Vec<(i64, Vec<i64>)> = file
        .map(|x| {
            let (left, right) = x.split_once(": ").unwrap();
            (
                left.parse().unwrap(),
                right.split(" ").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect();

    let mut total = 0;
    let mut total2 = 0;
    for (sum, list) in &lines {
        if part1_works(*sum, list) {
            total += sum;
        }

        if part2_works(*sum, list) {
            total2 += sum;
        }
    }

    println!("Part1: {total}");
    println!("Part2: {total2}")
}

// heuristics: if over the number, stop it
fn part1_works(sum: i64, list: &Vec<i64>) -> bool {
    return add_p1(sum, list[0], &list[1..]);
}

fn part2_works(sum: i64, list: &Vec<i64>) -> bool {
    return add_p2(sum, list[0], &list[1..]);
}

fn add_p1(sum: i64, curr: i64, rest: &[i64]) -> bool {
    if curr > sum {
        return false;
    }

    if rest.len() == 0 {
        return curr == sum;
    }

    let next = curr + rest[0];
    let next2 = curr * rest[0];

    add_p1(sum, next, &rest[1..]) || add_p1(sum, next2, &rest[1..])
}

fn add_p2(sum: i64, curr: i64, rest: &[i64]) -> bool {
    if curr > sum {
        return false;
    }

    if rest.len() == 0 {
        return curr == sum;
    }

    let next = curr + rest[0];
    let next2 = curr * rest[0];
    let next3 = curr * 10i64.pow(rest[0].ilog10() + 1) + rest[0];

    add_p2(sum, next, &rest[1..])
        || add_p2(sum, next2, &rest[1..])
        || add_p2(sum, next3, &rest[1..])
}
