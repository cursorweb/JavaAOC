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

    // i hate this so much
    // i wanted to be cool

    // is this a math test?
    // distance_traveled = (time - held) * held
    // dist = t * h - h ^ 2
    // 9 = 7 * h - h ^ 2
    // -h^2 + th - d = 0
    // t - sqrt(t^2 - 4d) / 2
    // increase that number until you lose, then multiply by 2

    let mut prod = 1;

    for (i, &time) in times.iter().enumerate() {
        let min = records[i];
        let mut count = 0;
        for held in 0..time {
            let dist = time * held - held * held;
            if dist > min {
                count += 1;
            }
        }

        prod *= count;
    }

    println!("{prod}");

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

    let mut count = 0;

    for held in 0..time {
        let dist = time * held - held * held;
        if dist > record {
            count += 1;
        } else if held > time / 2 {
            break;
        }
    }

    println!("{count}");
}
