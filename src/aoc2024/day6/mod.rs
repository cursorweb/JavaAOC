use std::collections::HashSet;

use crate::read;

struct Guard {
    /// (y,x)
    pos: (i32, i32),
    dir: Dir,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;

pub fn run() {
    let file = read!();

    let mut guard = None;
    let mut grid = vec![];

    for (y, r) in file.enumerate() {
        let mut row = vec![];
        for (x, c) in r.chars().enumerate() {
            if c == '^' {
                guard = Some(Guard {
                    pos: (y as i32, x as i32),
                    dir: Up,
                });

                row.push('.');
            } else {
                row.push(c);
            }
        }

        grid.push(row);
    }

    let mut guard = guard.unwrap();
    let start_pos = guard.pos;
    let mut visited = HashSet::new();

    visited.insert(guard.pos);
    loop {
        let Some(pos) = walk(&mut guard, &grid) else {
            break;
        };
        visited.insert(pos);
    }

    println!("Part1: {}", visited.len());

    let mut sum = 0;
    for obby in visited {
        guard.pos = start_pos;
        guard.dir = Up;
        if did_loop(&mut guard, &grid, obby) {
            sum += 1;
        }
    }

    println!("Part2: {sum}");
}

fn walk(guard: &mut Guard, grid: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    let mut npos = guard.pos;

    match guard.dir {
        Up => npos.0 -= 1,
        Down => npos.0 += 1,
        Left => npos.1 -= 1,
        Right => npos.1 += 1,
    }

    if npos.0 < 0 || npos.1 < 0 || npos.0 as usize >= grid.len() || npos.1 as usize >= grid[0].len()
    {
        return None;
    }

    if grid[npos.0 as usize][npos.1 as usize] == '#' {
        guard.dir = match guard.dir {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
        Some(guard.pos)
    } else {
        guard.pos = npos;
        Some(npos)
    }
}

fn did_loop(guard: &mut Guard, grid: &Vec<Vec<char>>, obby: (i32, i32)) -> bool {
    let mut visited = HashSet::new();

    visited.insert((guard.pos, Up));
    loop {
        let Some(pos) = walk2(guard, &grid, obby) else {
            break;
        };

        if visited.contains(&(pos, guard.dir)) {
            return true;
        }

        visited.insert((pos, guard.dir));
    }

    false
}

fn walk2(guard: &mut Guard, grid: &Vec<Vec<char>>, obby: (i32, i32)) -> Option<(i32, i32)> {
    let mut npos = guard.pos;

    match guard.dir {
        Up => npos.0 -= 1,
        Down => npos.0 += 1,
        Left => npos.1 -= 1,
        Right => npos.1 += 1,
    }

    if npos.0 < 0 || npos.1 < 0 || npos.0 as usize >= grid.len() || npos.1 as usize >= grid[0].len()
    {
        return None;
    }

    if grid[npos.0 as usize][npos.1 as usize] == '#' || npos == obby {
        guard.dir = match guard.dir {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
        Some(guard.pos)
    } else {
        guard.pos = npos;
        Some(npos)
    }
}
