pub mod aoc2022;

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
pub const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn name_to_dirs(name: &str) -> (i32, i32) {
    match name {
        "U" => (0, 1),
        "R" => (1, 0),
        "D" => (0, -1),
        "L" => (-1, 0),
        _ => unreachable!(),
    }
}
