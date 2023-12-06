use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let (times, records) = file
        .map(|line| {
            let right = line.split_once(":").unwrap().1.trim();
            right
                .split(" ")
                .filter_map(|x| x.parse::<i32>().ok())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    // is this a math test?
    // distance_traveled = (time - held) * held
    // dist = t * h - h ^ 2
    // 9 = 7 * h - h ^ 2
    // -h^2 + th - d = 0
    // -t +- sqrt(t^2 - 4 * d) / -2

    let mut prod = 1;

    for (i, &time) in times.iter().enumerate() {
        let min = records[i];
        let sqrt = f32::sqrt((time * time - 4 * min) as f32);

        // 1.69 -- 5.30
        // 2 -- 5 in integer (round up/round down)
        // 2.69 -> 2 -- 4.30 -> 5
        // this works because we want the numbers to be less than
        // you need to add 1 to top, and then round down
        // and sub 1 to bottom and round up

        let start = ((-time as f32 + sqrt) / -2f32 + 1f32).floor() as i32;
        let end = ((-time as f32 - sqrt) / -2f32 - 1f32).ceil() as i32;
        prod *= end - start + 1; // inclusive
    }

    println!("Part1: {prod}");

    let time: i64 = times
        .into_iter()
        .map(|x| x.to_string())
        .join("")
        .parse()
        .unwrap();

    let record: i64 = records
        .into_iter()
        .map(|x| x.to_string())
        .join("")
        .parse()
        .unwrap();

    let sqrt = f64::sqrt((time * time - 4 * record) as f64);
    let start = ((-time as f64 + sqrt) / -2f64 + 1f64).floor() as i64;
    let end = ((-time as f64 - sqrt) / -2f64 - 1f64).ceil() as i64;

    println!("Part2: {}", end - start + 1);
}
