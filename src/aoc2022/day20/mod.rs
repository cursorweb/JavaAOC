use crate::read;

const KEY: i64 = 811589153;

pub fn run() {
    let file = read!();
    let list: Vec<(usize, i64)> = file
        .enumerate()
        .map(|(i, x)| (i, x.parse().unwrap()))
        .collect();

    let len = list.len() as i32;

    let mut list1 = list.clone();
    swap(&mut list1, 0);
    let idx_0 = list1.iter().position(|(_, x)| *x == 0).unwrap() as i32;
    println!(
        "Part1: {}",
        [1000, 2000, 3000]
            .into_iter()
            .map(|x| { list1[((idx_0 + x) % len) as usize].1 })
            .sum::<i64>()
    );

    let mut list2 = list.into_iter().map(|(i, k)| (i, k * KEY)).collect();
    let mut prev_i = 0;
    for _ in 0..10 {
        prev_i = swap(&mut list2, prev_i);
    }

    let idx_0 = list2.iter().position(|(_, x)| *x == 0).unwrap() as i32;

    println!(
        "Part2: {}",
        [1000, 2000, 3000]
            .into_iter()
            .map(|x| { list2[((idx_0 + x) % len) as usize].1 })
            .sum::<i64>()
    );
}

/// `prev_i` is where the previous one is, then look left and right from there
fn swap(list: &mut Vec<(usize, i64)>, mut prev_i: usize) -> usize {
    let len = list.len() as i32;

    for id in 0..list.len() {
        let mut offset = 0;
        let mut negate = false;
        let mut val = list[prev_i];
        let mut i = prev_i as i32;

        while val.0 != id {
            i = prev_i as i32 + offset * if negate { -1 } else { 1 };

            i = i.rem_euclid(len);

            val = list[i as usize];

            // every other
            if !negate {
                offset += 1;
            }

            negate = !negate;
        }

        prev_i = i as usize; // where it was

        list.remove(i as usize);

        // where to swap
        // remember it's len - 1 because the moving element
        // doesn't count
        let swap_index = (i as i64 + val.1).rem_euclid((len - 1) as i64);

        list.insert(swap_index as usize, val);
    }

    prev_i
}

fn _print_list(list: &Vec<(usize, i32)>) {
    println!("{:?}", list.iter().map(|(_, x)| x).collect::<Vec<_>>());
}
