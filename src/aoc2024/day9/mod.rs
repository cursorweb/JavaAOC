use crate::read;

pub fn run() {
    let file = read!(str).chars();

    let mut disk = vec![];

    for (i, char) in file.enumerate() {
        let num = char as i32 - '0' as i32;
        if i % 2 == 1 {
            // 1 == empty
            disk.extend(vec![-1; num as usize]);
        } else {
            // 0 == file
            let id = i as i64 / 2;
            disk.extend(vec![id; num as usize]);
        }
    }

    let disk2 = disk.clone();

    let disk = fragment_compress(disk);
    let part1 = get_checksum(&disk);
    println!("Part1: {part1}");

    let disk2 = space_compress(disk2);
    let part2 = get_checksum(&disk2);
    println!("Part2: {part2}");
}

fn fragment_compress(mut disk: Vec<i64>) -> Vec<i64> {
    let mut front = 0usize;
    let mut back = disk.len() - 1;
    loop {
        // find a space
        while disk[front] != -1 {
            front += 1;
        }

        // find an id
        while disk[back] == -1 {
            back -= 1;
        }

        if front >= back {
            break;
        }

        disk.swap(back, front);
    }

    disk
}

fn space_compress(mut disk: Vec<i64>) -> Vec<i64> {
    let mut back = disk.len() - 1;

    // for each back, go through front and see if you can find a space up to the index
    // then swap
    while back > 0 {
        let mut front = 0usize;
        let mut curr_id = -1;

        let mut front_idxs = vec![];
        let mut back_idxs = vec![];

        while back > 0 {
            let item = disk[back];

            if item == -1 && curr_id == -1 {
                // look for nonempty
                back -= 1;
            } else if item != -1 && (curr_id == -1 || curr_id == item) {
                // add the block
                curr_id = item;
                back_idxs.push(back);
                back -= 1;
            } else {
                // found either an other id or an empty
                break;
            }
        }

        while front <= back {
            // find a space
            if disk[front] == -1 {
                front_idxs.push(front);

                // found empty spot
                if front_idxs.len() == back_idxs.len() {
                    for (&b, f) in back_idxs.iter().zip(front_idxs) {
                        disk.swap(b, f);
                    }

                    break;
                }
            } else {
                // no longer empty
                front_idxs.clear();
            }

            front += 1;
        }
    }

    disk
}

fn get_checksum(disk: &[i64]) -> i64 {
    disk.iter().enumerate().fold(0, |acc, (i, &curr)| {
        if curr != -1 {
            i as i64 * curr + acc
        } else {
            acc
        }
    })
}
