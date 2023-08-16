use crate::read;

pub fn run() {
    let file = read!();

    let (part1, part2) = file
        .map(|s| {
            let mut chars = s.chars();
            let l = chars.next().unwrap();
            let r = chars.skip(1).next().unwrap();
            (part1_score(l, r), part2_score(l, r))
        })
        .fold((0, 0), |(pa, pb), (a, b)| (pa + a, pb + b));

    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

// shape: 1=rock 2=paper 3=scissor
// outcome: 0=loss 3=L 6=W
// X Y Z
fn part1_score(abc: char, xyz: char) -> i32 {
    let shape_score = xyz as u8 - 'X' as u8 + 1;

    let xyz = xyz as u8 - 'X' as u8;
    let abc = abc as u8 - 'A' as u8;

    let win_score = if abc == xyz {
        3 // tie
    } else if (abc + 1) % 3 == xyz {
        6 // W
    } else {
        0
    };

    (shape_score + win_score) as i32
}

// shape: 1=rock 2=paper 3=scissor
// outcome: 0=loss 3=L 6=W
fn part2_score(abc: char, res: char) -> i32 {
    let win_score = match res {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => unreachable!(),
    };

    let abc = abc as u8 - 'A' as u8;

    let shape_score = match res {
        'X' => (abc + (3 - 1)) % 3,
        'Y' => abc,
        'Z' => (abc + 1) % 3,
        _ => unreachable!(),
    } + 1;

    (shape_score + win_score) as i32
}
