pub mod aoc2022;

#[macro_export]
macro_rules! read {
    () => {
        include_str!("data.txt").lines()
    };

    (str) => {
        include_str!("data.txt")
    };
}
