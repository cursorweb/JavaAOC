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

    let sum: i32 = records
        .iter()
        .map(|(chars, group)| count_solve(&chars, &group))
        .sum();

    // println!(">> {}", count_solve(&records[i].0, &records[i].1));
    println!("Part1: {sum:?}");
}

/// go through remaining char, and if can solve group, solve group
fn count_solve(chars: &[char], group: &[i32]) -> i32 {
    if chars.len() == 0 {
        return if group.len() > 0 {
            // There are still groups unsolved, so not solved
            0
        } else {
            // Success!! Solved all groups!
            1
        };
    }

    if group.len() == 0 {
        return if chars.contains(&'#') {
            // if no more groups, but there's a #
            // then that's bad
            0
        } else {
            1
        };
    }

    if chars[0] == '.' {
        return count_solve(&chars[1..], group);
    }

    // check if it solved a group
    if chars[0] == '#' {
        let group_len = group[0] as usize;
        if group_len > chars.len() {
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

            return count_solve(
                if group_len + 1 < chars.len() {
                    &chars[group_len + 1..]
                } else {
                    &[]
                },
                &group[1..],
            );
        } else {
            // didn't work, and we can't skip it either
            // because this # MUST be accounted for! (which it hasn't)
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
        return count_solve(&with_hash, group) + count_solve(rest, group);
    }

    unreachable!("skill issue lmao")
}
