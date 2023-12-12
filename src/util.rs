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
            .lines()
            .map(String::from)
            .collect::<Vec<String>>()
            .into_iter()
    }};

    (str) => {{
        use std::{fs, path::Path};

        let file_path = file!();
        let path = Path::new(file_path).parent().unwrap().join("data.txt");

        fs::read_to_string(&path).unwrap_or_else(|_| panic!("Please create a data.txt!"))
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

fn factorial<T>(n: T) -> T
where
    T: std::ops::Mul<Output = T>
        + std::iter::Product
        + From<u8>
        + std::cmp::PartialOrd
        + std::cmp::PartialEq
        + std::ops::AddAssign
        + Copy,
{
    if n < T::from(0_u8) {
        panic!("n can't be less than 0")
    } else if n == T::from(0_u8) || n == T::from(1_u8) {
        T::from(1_u8)
    } else {
        let mut result = T::from(1_u8);
        let mut i = T::from(2_u8);

        while i <= n {
            result = result * i;
            i += T::from(1_u8);
        }

        result
    }
}
