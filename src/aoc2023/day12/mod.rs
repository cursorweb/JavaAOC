use std::collections::HashMap;

use itertools::Itertools;

use crate::read;

pub fn run() {
    let file = read!();
    let records: Vec<(Vec<char>, Vec<i32>)> = file
        .map(|line| {
            let (left, right) = line.split_once(" ").unwrap();
            (
                left.chars().collect(),
                right.split(",").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect_vec();

    let sum: i64 = records
        .iter()
        .map(|(chars, group)| count_solve(&chars, &group))
        .sum();

    println!("Part1: {sum:?}");

    let records = records
        .into_iter()
        .map(|(chars, nums)| {
            let chars = String::from_iter(chars);
            let string = vec![&chars].repeat(5);
            let nums = nums.repeat(5);

            (string.into_iter().join("?").chars().collect_vec(), nums)
        })
        .collect_vec();

    let sum: i64 = records
        .iter()
        .map(|(chars, group)| count_solve(&chars, &group))
        .sum();

    println!("Part2: {sum:?}");
}

fn count_solve(chars: &[char], group: &[i32]) -> i64 {
    let mut cache = HashMap::new();
    let out = _count_solve(chars, group, &mut cache);
    out
}

/// go through remaining char, and if can solve group, solve group
fn _count_solve(chars: &[char], group: &[i32], cache: &mut HashMap<(String, i32), i64>) -> i64 {
    if let Some(&val) = cache.get(&hash_it(chars, group)) {
        return val;
    }

    // base cases
    if chars.len() == 0 {
        let val = if group.len() > 0 {
            // There are still groups unsolved, so not solved
            0
        } else {
            // Success!! Solved all groups!
            1
        };

        return val;
    }

    if group.len() == 0 {
        let val = if chars.contains(&'#') {
            // if no more groups, but there's a #
            // then that's bad
            0
        } else {
            1
        };

        return val;
    }

    if chars[0] == '.' {
        let out = _count_solve(&chars[1..], group, cache);
        cache.insert(hash_it(chars, group), out);
        return out;
    }

    // check if it solved a group
    if chars[0] == '#' {
        let group_len = group[0] as usize;
        if group_len > chars.len() {
            cache.insert(hash_it(chars, group), 0);
            return 0;
        }

        // must be all #
        let solved = !chars[0..group_len].contains(&'.');

        /*
        must be discrete
        ### group=1
         ^^ -- this is actually a 3 block
        */
        let is_discrete = if group_len == chars.len() {
            true
        } else {
            chars[group_len] != '#'
        };

        if solved && is_discrete {
            // got the first group down
            // now check if can go next
            // skip the '.' at group_len (or if it is a '?' convert it into a '.')
            let out = _count_solve(
                if group_len + 1 < chars.len() {
                    &chars[group_len + 1..]
                } else {
                    &[]
                },
                &group[1..],
                cache,
            );
            cache.insert(hash_it(chars, group), out);
            return out;
        } else {
            // didn't work, and we can't skip it either
            // because this # MUST be accounted for! (which it hasn't)
            cache.insert(hash_it(chars, group), 0);
            return 0;
        }
    }

    // try and solve (either # or .)
    if chars[0] == '?' {
        // fun rust feature : [1, 2, 3][3..] works! (Returns [])
        let rest = &chars[1..];

        let mut with_hash = vec!['#'];
        with_hash.extend(rest);

        // with a # or as a dot (so skip)
        let out = _count_solve(&with_hash, group, cache) + _count_solve(rest, group, cache);
        cache.insert(hash_it(chars, group), out);
        return out;
    }

    unreachable!("skill issue lmao")
}

fn hash_it(chars: &[char], group: &[i32]) -> (String, i32) {
    (String::from_iter(chars), group.len() as i32)
}
