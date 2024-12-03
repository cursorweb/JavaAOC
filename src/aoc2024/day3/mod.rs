use crate::read;

pub fn run() {
    let file = read!(str);

    let mut sum1 = 0;
    let mut sum2 = 0;

    let mut s = String::new();
    let mut mul_mode = false;
    let mut left = None;

    let mut should_do = true;

    // let mut right;
    for c in file.chars() {
        if s == "do()" {
            should_do = true;
            s.clear();
        }

        if s == "don't()" {
            should_do = false;
            s.clear();
        }

        if s == "mul" {
            if c == '(' {
                mul_mode = true;
                s.clear();
                continue;
            } else {
                s.clear();
            }
        }

        if mul_mode && c == ',' {
            left = Some(s.parse::<i32>().unwrap());
            s.clear();
            continue;
        }

        if mul_mode && c == ')' {
            let right = s.parse::<i32>().unwrap();
            let product = left.unwrap() * right;

            sum1 += product;

            if should_do {
                sum2 += product;
            }

            s.clear();
            mul_mode = false;
            continue;
        }

        s.push(c);

        if !mul_mode
            && !s.starts_with(&"mul("[..s.len().clamp(0, 4)])
            && !s.starts_with(&"do()"[..s.len().clamp(0, 4)])
            && !s.starts_with(&"don't()"[..s.len()])
        {
            s.clear();
        }

        if mul_mode && !c.is_ascii_digit() {
            s.clear();
            mul_mode = false;
        }
    }

    println!("Part1: {sum1}");
    println!("Part2: {sum2}");
}
