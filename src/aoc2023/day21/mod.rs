use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::{read, DIRS};

const STEPS: i32 = 64;
const STEPS2: i64 = 10; //26_501_365;
                        // n = 2

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

    let (part1, part2) = bfs(&map, start);
    println!("Part1: {part1}");
    println!("Part2: {part2}");
}

fn bfs(map: &Vec<Vec<char>>, (sy, sx): (i64, i64)) -> (i64, i64) {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_front(((sy, sx), 0));

    let mut ans = HashSet::new();

    while let Some(((y, x), steps)) = queue.pop_front() {
        if visited.contains_key(&(y, x)) {
            continue;
        }

        visited.insert((y, x), steps as i64);

        if steps % 2 == 0 && steps <= STEPS {
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

    let half = map.len() as i64 / 2;

    let n = (STEPS2 - half) / map.len() as i64;

    let size = map.len() as i64;

    let end = (map.len() - 1) as i64;

    println!("1:");
    crate::dot!(map, |y, x, c| {
        if visited.contains_key(&(x, y))
            && visited[&(x, y)] % 2 == 1
            && dist(x, y, half, 0) <= end
        {
            'O'
        } else {
            c
        }
    });

    println!("2:");
    crate::dot!(map, |y, x, c| {
        if visited.contains_key(&(x, y))
            && visited[&(x, y)] % 2 == 1
            && dist(x, y, end, 0) <= size + half
        {
            'O'
        } else {
            c
        }
    });

    println!("3:");
    crate::dot!(map, |y, x, c| {
        if visited.contains_key(&(x, y)) && visited[&(x, y)] % 2 == 0 && dist(x, y, end, 0) <= half
        {
            'O'
        } else {
            c
        }
    });

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

    let odd = visited.values().filter(|&&steps| steps % 2 == 1).count() as i64;
    let even = visited.values().filter(|&&steps| steps % 2 == 0).count() as i64;

    // 1
    let tip = visited
        .iter()
        .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, size, half) <= end) // <
        .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, half, size) <= end) // ^
            .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, 0, half) <= end) // >
            .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, half, 0) <= end) // v
            .count();

    // 2
    let edge_corner = visited
        .iter()
        .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, end, end) <= size + half) // / top
        .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, 0, end) <= size + half) // top \
            .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, 0, 0) <= size + half) // bottom /
            .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 1 && dist(x, y, end, 0) <= size + half) // \ bottom
            .count();

    // 3
    let smol_edge = visited
        .iter()
        .filter(|&(&(y, x), &steps)| steps % 2 == 0 && dist(x, y, end, end) <= half) // / top
        .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 0 && dist(x, y, 0, end) <= half) // top \
            .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 0 && dist(x, y, 0, 0) <= half) // bottom /
            .count()
        + visited
            .iter()
            .filter(|&(&(y, x), &steps)| steps % 2 == 0 && dist(x, y, end, 0) <= half) // \ bottom
            .count();

    let (tip, edge_corner, smol_edge) = (tip as i64, edge_corner as i64, smol_edge as i64);

    let odd_grid_count = (n - 1).pow(2);
    let even_grid_count = (n).pow(2);

    println!("n={n} even: {even}, odd: {odd} half={half}");
    println!("# odd ={odd_grid_count} # even = {even_grid_count}");

    (
        ans.len() as i64,
        odd_grid_count * odd + even_grid_count * even + tip + (n - 1) * edge_corner + n * smol_edge,
    )
}

fn dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
