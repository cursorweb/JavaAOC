use itertools::Itertools;

use crate::read;

#[derive(Debug, Clone, Copy)]
struct Mapper {
    dest_start: i64,
    source_start: i64,
    len: i64,
}

impl Mapper {
    /// maps source to dest
    fn map(&self, num: i64) -> Option<i64> {
        if self.source_start <= num && num < self.source_start + self.len {
            Some(self.dest_start + num - self.source_start)
        } else {
            None
        }
    }

    fn maps_down(&self) -> bool {
        self.dest_start < self.source_start
    }

    /// (start, len)
    fn map_range(&self, (start, len): (i64, i64)) -> Option<(i64, i64)> {
        let start = start.clamp(self.source_start, self.source_start + self.len);
        let end = (start + len).clamp(self.source_start, self.source_start + self.len);
        let len = end - start;

        if len == 0 {
            None
        } else {
            let diff = self.source_start - self.dest_start;
            Some((start - diff, len))
        }
    }
}

pub fn run() {
    let mut file = read!(str).split("\n\n");
    let seeds: Vec<i64> = file
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect_vec();

    let maps: Vec<Vec<Mapper>> = file
        .map(|line| {
            let mut lines = line.split("\n");
            lines.next(); // x-to-x

            /*
            seed-to-soil map:
            50 98 2 <-- dest range start, source range start, length
            52 50 48 <-- dest range start, source range start, length
            */
            let mappers = lines
                .map(|x| {
                    let mut nums = x.split(" ").map(|n| n.parse().unwrap());
                    let dest_start = nums.next().unwrap();
                    let source_start = nums.next().unwrap();
                    let len = nums.next().unwrap();

                    Mapper {
                        dest_start,
                        source_start,
                        len,
                    }
                })
                .collect_vec();
            mappers
        })
        .collect_vec();

    let mut lowest = i64::MAX;
    for &seed in &seeds {
        let mut loc_out = seed;

        // map it through the whole thing
        for mappers in &maps {
            let mut map_out = loc_out;
            for mapper in mappers {
                if let Some(val) = mapper.map(map_out) {
                    map_out = val;
                    break;
                }
            }

            loc_out = map_out;
        }

        if loc_out < lowest {
            lowest = loc_out;
        }
    }

    println!("Part1: {lowest}");

    /*
    optimization:
    - only go through the range that actually will optimize lower
        - ignore if the range will optimize higher
    - map the entire range, and take the lowest
    */
    let mut seed_ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push((seeds[i], seeds[i + 1]));
    }

    let mut lowest = i64::MAX;
    for range in seed_ranges {
        for seed in range.0..range.0 + range.1 {
            let mut loc_out = seed;

            // map it through the whole thing
            for mappers in &maps {
                let mut map_out = loc_out;
                for mapper in mappers {
                    if let Some(val) = mapper.map(map_out) {
                        map_out = val;
                        break;
                    }
                }

                loc_out = map_out;
            }

            if loc_out < lowest {
                lowest = loc_out;
            }
        }
    }
    println!("Part2: {lowest}");
}
