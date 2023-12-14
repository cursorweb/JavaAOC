use std::collections::HashMap;

use itertools::{Either, Itertools};

use crate::read;

const COUNT: usize = 1_000_000_000;

pub fn run() {
    let file = read!();
    let mut map = file.map(|line| line.chars().collect_vec()).collect_vec();

    let part2_map = map.clone();

    // secret hack where the first index can't be moved
    for y in 1..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                map[y][x] = '.';

                let (ny, nx) = shift_north(&mut map, y, x);

                assert_ne!(map[ny][nx], 'O');
                map[ny][nx] = 'O';
            }
        }
    }

    println!("Part1: {}", get_a_load_of_this(map));

    // value, (index, weight)
    let mut cache: HashMap<String, usize> = HashMap::new();

    // find how much remaining after the prelim checking
    let mut remaining = 0;

    let mut map = part2_map;
    for i in 0..200 {
        cycle(&mut map);
        let dot = dotstring(&map);
        if let Some(prev_i) = cache.get(&dot) {
            let rest = COUNT - i;
            let skip = i - prev_i;
            remaining = rest % skip - 1;
            // 0 1 2 3 4 5 6 7 8 9 10 11 12
            // 0 1 2 3 4 5 6 7     ^
            //         ^     ^ skip = 3
            // rest = 12 - 7 = 5
            // remaining = 5 % 3 == 2
            // you need to subtract one because we start at 0
            break;
        } else {
            cache.insert(dot, i);
        }
    }

    assert_ne!(remaining, 0);

    for _ in 0..remaining {
        cycle(&mut map);
    }

    println!("Part2: {}", get_a_load_of_this(map));
}

fn cycle(map: &mut Vec<Vec<char>>) {
    for i in 0..4 {
        // south needs to be reversed
        let y_range = if i == 2 {
            Either::Right((0..map.len()).rev())
        } else {
            Either::Left(0..map.len())
        };
        for y in y_range {
            let x_range = if i == 3 {
                Either::Right((0..map[0].len()).rev())
            } else {
                Either::Left(0..map[0].len())
            };
            for x in x_range {
                if map[y][x] == 'O' {
                    map[y][x] = '.';

                    // north ^ west > south v east <
                    let (ny, nx) = match i {
                        0 => shift_north(map, y, x),
                        1 => shift_west(map, y, x),
                        2 => shift_south(map, y, x),
                        3 => shift_east(map, y, x),
                        _ => unreachable!("{i}"),
                    };

                    assert_ne!(map[ny][nx], 'O');
                    map[ny][nx] = 'O';
                }
            }
        }
    }
}

fn get_a_load_of_this(map: Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    for y in 0..map.len() {
        let times = map.len() - y;
        count += (times * map[y].iter().filter(|&&x| x == 'O').count()) as i64;
    }

    count
}

/// (y, x)
fn shift_north(map: &mut Vec<Vec<char>>, y: usize, x: usize) -> (usize, usize) {
    let mut ny = y as i32;

    while ny >= 0 {
        if map[ny as usize][x] == '#' || map[ny as usize][x] == 'O' {
            break;
        }

        ny -= 1;
    }

    let ny = (ny + 1) as usize;

    (ny, x)
}

fn shift_south(map: &mut Vec<Vec<char>>, y: usize, x: usize) -> (usize, usize) {
    let mut ny = y as i32;

    map[y][x] = '.';

    while ny < map.len() as i32 {
        if map[ny as usize][x] == '#' || map[ny as usize][x] == 'O' {
            break;
        }

        ny += 1;
    }

    let ny = (ny - 1) as usize;

    (ny, x)
}

fn shift_west(map: &mut Vec<Vec<char>>, y: usize, x: usize) -> (usize, usize) {
    let mut nx = x as i32;

    map[y][x] = '.';

    while nx >= 0 {
        if map[y][nx as usize] == '#' || map[y][nx as usize] == 'O' {
            break;
        }

        nx -= 1;
    }

    let nx = (nx + 1) as usize;

    (y, nx)
}

fn shift_east(map: &mut Vec<Vec<char>>, y: usize, x: usize) -> (usize, usize) {
    let mut nx = x as i32;

    map[y][x] = '.';

    while nx < map.len() as i32 {
        if map[y][nx as usize] == '#' || map[y][nx as usize] == 'O' {
            break;
        }

        nx += 1;
    }

    let nx = (nx - 1) as usize;

    (y, nx)
}

fn _dot(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}

fn dotstring(map: &Vec<Vec<char>>) -> String {
    let mut out = String::new();
    for row in map {
        for &c in row {
            out.push(c);
        }
        out += "\n";
    }
    out
}
