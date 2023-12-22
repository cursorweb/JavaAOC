/// Read the `data.txt` relative to your current directory.
///
/// You can optionally include `str` if you don't want it to split by lines.
///
/// # Examples
/// ```
/// read!(); // Lines<'_>
/// read!(str); // &str
/// ```
#[macro_export]
macro_rules! read {
    () => {{
        use std::{fs, path::Path};

        let file_path = file!();
        let path = Path::new(file_path).parent().unwrap().join("data.txt");

        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Please create a data.txt!"))
            .leak()
            .lines()
    }};

    (str) => {{
        use std::{fs, path::Path};

        let file_path = file!();
        let path = Path::new(file_path).parent().unwrap().join("data.txt");

        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Please create a data.txt!"))
            .leak()
    }};
}

/// The four directions (up, down, left, right)
/// It is in the form (y, x) with negative being up and left
pub const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// in the form (y, x) with negative being up and left
pub fn name_to_dirs(name: &str) -> (i32, i32) {
    match name {
        "U" => (-1, 0),
        "R" => (0, 1),
        "D" => (1, 0),
        "L" => (0, -1),
        _ => unreachable!(),
    }
}

/// debug version for easy printing in the form (x, y)
pub fn name_to_dirs_dbg(name: &str) -> (i32, i32) {
    match name {
        "U" => (0, -1),
        "R" => (1, 0),
        "D" => (0, 1),
        "L" => (-1, 0),
        _ => unreachable!(),
    }
}

/// checks if two ranges intersect
pub fn range_intersects<T: PartialOrd>(r1: (T, T), r2: (T, T)) -> bool {
    return (r1.0 <= r2.1) && (r1.1 >= r2.0);
}

/// enter as step debugging
#[macro_export]
macro_rules! input {
    () => {
        use std::io::{stdin, stdout, Write};

        print!(">>> {}:{}:{} : ", file!(), line!(), column!());
        stdout().flush().unwrap();
        stdin().read_line(&mut String::new()).unwrap();
    };
}

/// print grid of array
/// (y, x) where y++ and x++ are down and right
///
/// Usage:
/// ```
/// dot!(my_vec);
/// dot!(my_vec, true);
/// dot!(map, |y, x, c| if x == 0 && y == 0 { '#' } else { c });
/// dot!(map, |y, x, c| if x == 0 && y == 0 { '#' } else { c }, true);
/// ```
#[macro_export]
macro_rules! dot {
    ($map:expr) => {
        for row in $map {
            for c in row {
                print!("{c}");
            }
            println!();
        }
        println!();
    };

    ($map:expr, true) => {
        use crate::input;
        for row in $map {
            for c in row {
                print!("{c}");
            }
            println!();
        }
        println!();
        input!();
    };

    ($map:expr, $fn:expr) => {
        for (y, row) in $map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                print!("{}", $fn(y as i32, x as i32, *c));
            }
            println!();
        }
        println!();
    };

    ($map:expr, $fn:expr, true) => {
        use crate::input;
        for (y, row) in $map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                print!("{}", $fn(y as i32, x as i32, *c));
            }
            println!();
        }
        println!();
        input!();
    };
}

pub fn iter_lcm(iter: impl Iterator<Item = i64>) -> i64 {
    iter.fold(1, |p, c| lcm(p, c))
}

pub fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

pub fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
