use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{read, DIRS};

const STEPS: i64 = 64;
const STEPS2: i64 = 26_501_365;

pub fn run() {
    let file = read!();

    let mut start = (0, 0);

    let map = file
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (y as i64, x as i64);
                        '.'
                    } else {
                        c
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let part1 = bfs(&map, start, STEPS, 0);
    println!("Part1: {part1}");

    /*
    n=2
    # of 1 = 4 (2n)
    # of 2 = 4 (2n)
    # of 3 = 8 (4n)
    odd = 1
    even = 4
    .313.
    32E23
    1EOE1
    32E23
    .313.

    ..313..
    .32O23.
    32OEO23
    1OEOEO1
    32OEO23
    .32O23.
    ..313..
    n=3
    # of 1 = 4
    # of 2 = 8  (n - 1) * 4
    # of 3 = 12 n * 4
    odd=9
    even=4

    ...313
    ..32E23
    .32EOE23
    32EOEOE23
    1EOEOEOE1
    32EOEOE23
    .32EOE23
    ..32E23
    ...313
    n=4

    odd=9
    even=16

    1=4 1 * corner
    2=12 n - 1 * all corner
    3=16 n * all corner
    */

    let half = map.len() as i64 / 2;
    let n = (STEPS2 - half) / map.len() as i64;

    let end = map.len() as i64 - 1; // inclusive end

    let size = map.len() as i64;

    let odd = bfs(&map, start, size, 1);
    let even = bfs(&map, start, size, 0);

    // 2
    let corner_tl = bfs(&map, (0, 0), size + half, 1);
    let corner_tr = bfs(&map, (0, end), size + half, 1);
    let corner_bl = bfs(&map, (end, 0), size + half, 1);
    let corner_br = bfs(&map, (end, end), size + half, 1);

    // 1
    let tiptop = bfs(&map, (end, half), size, 0);
    let tipright = bfs(&map, (half, 0), size, 0);
    let tipbottom = bfs(&map, (0, half), size, 0);
    let tipleft = bfs(&map, (half, end), size, 0);

    // 3
    let smol_tl = bfs(&map, (0, 0), half, 0);
    let smol_tr = bfs(&map, (0, end), half, 0);
    let smol_bl = bfs(&map, (end, 0), half, 0);
    let smol_br = bfs(&map, (end, end), half, 0);

    let num_odds = (n - 1).pow(2);
    let num_evens = n.pow(2);

    let count = (tiptop + tipright + tipbottom + tipleft)
        + (n - 1) * (corner_tl + corner_tr + corner_br + corner_bl)
        + n * (smol_tl + smol_tr + smol_bl + smol_br)
        + num_odds * odd
        + num_evens * even;
    println!("Part2: {}", count);
}

fn bfs(map: &Vec<Vec<char>>, (sy, sx): (i64, i64), max_steps: i64, parity: i64) -> i64 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_front(((sy, sx), 0));

    let mut ans = HashSet::new();

    while let Some(((y, x), steps)) = queue.pop_front() {
        if visited.contains(&(y, x)) {
            continue;
        }

        visited.insert((y, x));

        if steps % 2 == parity && steps <= max_steps {
            ans.insert((y, x));
        }

        for (dy, dx) in DIRS {
            let npos @ (ny, nx) = (y + dy as i64, x + dx as i64);
            if (ny >= 0 && ny < map.len() as i64 && nx >= 0 && nx < map[0].len() as i64)
                && map[ny as usize][nx as usize] != '#'
            {
                // go to all the ones that have steps before going on to steps + 1
                queue.push_back((npos, steps + 1));
            }
        }
    }

    ans.len() as i64
}
