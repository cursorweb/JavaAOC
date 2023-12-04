use itertools::Itertools;

use crate::read;

/// Map Slice
/// ```txt
/// localx = realx - offset
/// ```
#[derive(Debug)]
struct MSlice {
    offset: i32,
    // last element
    end: i32,
    slice: Vec<char>,
}

impl std::ops::Index<i32> for MSlice {
    type Output = char;

    /// x
    fn index(&self, ix: i32) -> &Self::Output {
        let localx = (ix - self.offset) as usize;
        &self.slice[localx]
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Right,
    Left,
    Forward(i32),
}

use Instr::*;

pub fn run() {
    let file = read!(str);
    let (map, pathstr) = file.split_once("\n\n").unwrap();

    let map = map
        .split("\n")
        .map(|line| {
            let line_len = line.len();
            let path = line.trim_start();

            let offset = (line_len - path.len()) as i32;

            let slice = path.chars().collect_vec();

            MSlice {
                offset,
                end: offset + slice.len() as i32 - 1,
                slice,
            }
        })
        .collect_vec();

    let mut path = Vec::new();

    let mut num = String::new();
    for char in pathstr.chars() {
        if char.is_numeric() {
            num.push(char);
        } else {
            path.push(Forward(num.parse().unwrap()));

            path.push(match char {
                'R' => Right,
                'L' => Left,
                _ => unreachable!("{char}"),
            });

            num = String::new();
        }
    }

    // trailing number
    path.push(Forward(num.parse().unwrap()));

    // parsing done!! yay!!

    // (y, x) in absolute, assume it is open
    assert_eq!(map[0][map[0].offset], '.');
    let mut pos: (i32, i32) = (0, map[0].offset);

    // rotate right, i++
    let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut diri: i32 = 0;

    for &instr in &path {
        match instr {
            Right => diri = (diri + 1) % 4,
            Left => diri = (diri - 1).rem_euclid(4),
            Forward(n) => {
                let (dy, dx) = dirs[diri as usize];
                for _ in 0..n {
                    // x
                    let mut nx = pos.1 + dx;
                    let row = &map[pos.0 as usize];

                    // wrap around
                    if nx < row.offset {
                        nx = row.end;
                    } else if nx > row.end {
                        nx = row.offset;
                    }

                    if row[nx] == '#' {
                        break;
                    }

                    pos.1 = nx;

                    // y
                    let mut ny = pos.0 + dy;

                    // if out of bounds for map, then we need to wrap
                    if ny == map.len() as i32
                        || ny < 0
                        || pos.1 < map[ny as usize].offset
                        || pos.1 > map[ny as usize].end
                    {
                        /*
                        ......
                        ...# ^ dy =  1, di = -1
                        .... |
                        ...v |

                        ...^ | dy = -1, di =  1
                        .... |
                        ...# v
                        ......

                        go negative dy from pos.y
                        until the x position isn't inside the range (or reach end/beginning)
                        check i + dy if you can go there, then go there
                        */
                        let mut i = pos.0;

                        while i >= 0 && i < map.len() as i32 {
                            let row = &map[i as usize];
                            if pos.1 < row.offset || pos.1 > row.end {
                                break;
                            }
                            i -= dy;
                        }

                        i += dy;
                        ny = i;
                    }

                    if map[ny as usize][pos.1] == '#' {
                        break;
                    }

                    pos.0 = ny;
                }
            }
        }
    }

    // note: r, c starts at 1
    // 1000 * row + 4 * column + facing
    let password = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + diri;
    println!("Part1: {password}");

    for &instr in &path {
        match instr {
            Right => diri = (diri + 1) % 4,
            Left => diri = (diri - 1).rem_euclid(4),
            Forward(n) => {
                let (dy, dx) = dirs[diri as usize];
                for _ in 0..n {
                    // x
                    let mut nx = pos.1 + dx;
                    let row = &map[pos.0 as usize];

                    // wrap around
                    if nx < row.offset {
                        nx = row.end;
                    } else if nx > row.end {
                        nx = row.offset;
                    }

                    if row[nx] == '#' {
                        break;
                    }

                    pos.1 = nx;

                    // y
                    let mut ny = pos.0 + dy;

                    // if out of bounds for map, then we need to wrap
                    if ny == map.len() as i32
                        || ny < 0
                        || pos.1 < map[ny as usize].offset
                        || pos.1 > map[ny as usize].end
                    {
                        /*
                        ......
                        ...# ^ dy =  1, di = -1
                        .... |
                        ...v |

                        ...^ | dy = -1, di =  1
                        .... |
                        ...# v
                        ......

                        go negative dy from pos.y
                        until the x position isn't inside the range (or reach end/beginning)
                        check i + dy if you can go there, then go there
                        */
                        let mut i = pos.0;

                        while i >= 0 && i < map.len() as i32 {
                            let row = &map[i as usize];
                            if pos.1 < row.offset || pos.1 > row.end {
                                break;
                            }
                            i -= dy;
                        }

                        i += dy;
                        ny = i;
                    }

                    if map[ny as usize][pos.1] == '#' {
                        break;
                    }

                    pos.0 = ny;
                }
            }
        }
    }
}

fn _dot(map: &Vec<MSlice>, pos: (i32, i32), dir: (i32, i32)) {
    for (y, row) in map.iter().enumerate() {
        print!("{}", " ".repeat(row.offset as usize));
        assert_eq!(row.end - row.offset + 1, row.slice.len() as i32);
        for (x, char) in row.slice.iter().enumerate() {
            if (y as i32, x as i32 + row.offset) == pos {
                print!(
                    "{}",
                    match dir {
                        (0, 1) => ">",
                        (1, 0) => "v",
                        (0, -1) => "<",
                        (-1, 0) => "^",
                        _ => unreachable!(),
                    }
                );
            } else {
                print!("{char}");
            }
        }
        println!();
    }
    println!();
}
