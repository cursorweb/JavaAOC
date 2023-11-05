use std::collections::HashMap;

use crate::read;

/// closest beacon (y, x)
#[derive(Debug)]
struct Sensor(i32, i32);

#[derive(Debug)]
enum Pos {
    Sens(Sensor),
    Beac,
}

use Pos::*;

pub fn run() {
    let file = read!();

    // let objects: HashMap<(i32, i32), Pos> = HashMap::new();
    // let objects = HashMap::new();

    let objects: HashMap<(i32, i32), Pos> = file
        .flat_map(|line| {
            let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();

            let sensor = sensor.replace("Sensor at ", "");
            let (sx, sy) = sensor.split_once(", ").unwrap();
            let sx: i32 = sx.split_once("=").unwrap().1.parse().unwrap();
            let sy: i32 = sy.split_once("=").unwrap().1.parse().unwrap();

            let (bx, by) = beacon.split_once(", ").unwrap();
            let bx: i32 = bx.split_once("=").unwrap().1.parse().unwrap();
            let by: i32 = by.split_once("=").unwrap().1.parse().unwrap();

            vec![((sy, sx), Sens(Sensor(by, bx))), ((by, bx), Beac)]
        })
        .collect();

    dot(&objects);
}

fn dot(objects: &HashMap<(i32, i32), Pos>) {
    for y in -2..22 {
        for x in -2..25 {
            let res = objects.get(&(y, x));
            if let Some(Sens(_)) = res {
                print!("S");
            } else if let Some(Beac) = res {
                print!("B");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
