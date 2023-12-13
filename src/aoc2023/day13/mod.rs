use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!(str).split("\n\n");
    let grids = file
        .map(|grid| {
            grid.split("\n")
                .map(|x| x.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    // warning: index by 1
    println!(
        "{:?}",
        grids.iter().map(|grid| get_sym(grid).into()).sum::<usize>()
    );
    // println!("{:?}", split_col(2, &grids[0]));
}

/// yessir
fn get_col(x: usize, grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut out = Vec::new();
    for c in grid {
        out.push(c[x]);
    }

    out
}

#[derive(Debug)]
enum Symmetry {
    Vert(usize),
    Horiz(usize),
}

impl Symmetry {
    fn into(self) -> usize {
        match self {
            Vert(n) => n,
            Horiz(n) => n * 100,
        }
    }
}

use Symmetry::*;

fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = Vec::new();

    for i in 0..grid[0].len() {
        new_grid.push(get_col(i, grid));
    }

    new_grid
}

/// get from x up to len, and x back to 0
/// 0 0 0 x 0 l=5
fn split(i: usize, grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    (
        grid[0..i].into_iter().cloned().rev().collect_vec(),
        grid[i..].to_vec(),
    )
}

fn get_sym(grid: &Vec<Vec<char>>) -> Symmetry {
    // horizontal
    for y in 1..grid.len() {
        let (top, bottom) = split(y, grid);

        // println!(
        //     "y={y}\n{}\n----\n{}\n\n",
        //     top.iter().map(|x| String::from_iter(x)).join("\n"),
        //     bottom.iter().map(|x| String::from_iter(x)).join("\n")
        // );

        let mut found = true;
        let offset = top.len().min(bottom.len());
        for i in 0..offset {
            if top[i] != bottom[i] {
                found = false;
                break;
            }
        }

        if found {
            // include the y
            return Horiz(y);
        }
    }

    // vert
    let x_grid = &rotate(grid);
    for x in 1..x_grid.len() {
        let (left, right) = split(x, x_grid);

        // println!(
        //     "x={x}\n{}\n----\n{}\n\n",
        //     left.iter().map(|x| String::from_iter(x)).join("\n"),
        //     right.iter().map(|x| String::from_iter(x)).join("\n")
        // );

        let mut found = true;
        let offset = left.len().min(right.len());
        for i in 0..offset {
            if left[i] != right[i] {
                found = false;
                break;
            }
        }

        if found {
            // include the y
            return Vert(x);
        }
    }

    _dot(grid);
    _dot(x_grid);
    unreachable!("skill issue lmao")
}

fn _dot(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!();
    }
    println!();
}
