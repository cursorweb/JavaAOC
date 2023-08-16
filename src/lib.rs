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
