pub mod aoc2022;
pub mod aoc2023;

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
    () => {
        include_str!("data.txt").lines()
    };

    (str) => {
        include_str!("data.txt")
    };
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
