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

    let mut seed_ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push((seeds[i], seeds[i] + seeds[i + 1]));
    }

    for mappers in maps {
        let mut next_ranges = Vec::new();
        while let Some((start, end)) = seed_ranges.pop() {
            let mut mapped = false;
            for mapper in &mappers {
                let (o_start, o_end) = mapper.overlap((start, end));

                // mapped range is nonempty, so must have some mapping
                /*
                don't check other ranges because they may duplicate
                the split ranges:

                LEGEND: = mapper, - range, ~ split range
                |------------|
                  |==|  |==|
                |~|  |~~~~~~~| mapped 1
                |~~~~~~~|  |~| mapped 2
                */
                if o_start < o_end {
                    /*
                    three cases:
                    mapped?  no      yes       no
                          |------|----------|------|
                                 |==mapper==|
                    */
                    next_ranges.push((mapper.imap(o_start), mapper.imap(o_end)));

                    /*
                        |--|  overlap
                    |~~~----- actual
                     ^^^ must have some before it (which could be remapped)
                    */
                    if o_start > start {
                        seed_ranges.push((start, o_start));
                    }

                    /*
                      |--|     overlap
                    ------~~~| actual
                          ^^^
                    */
                    if o_end < end {
                        seed_ranges.push((o_end, end));
                    }

                    mapped = true;
                    break;
                }
            }

            // there is a chance that none of the mappers
            // will map the range
            if !mapped {
                next_ranges.push((start, end));
            }
        }

        // newly mapped
        seed_ranges = next_ranges;
    }

    seed_ranges.sort_by(|(start1, _), (start2, _)| start1.cmp(start2));

    println!("Part2: {}", seed_ranges[0].0);
}
