use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::read;

/// part: vec[x, m, a, s]
#[derive(Debug)]
struct WorkFlow {
    conds: Vec<Cond>,
    /// fall through case
    last: String,
}

/// a<2006:qkq
#[derive(Debug)]
struct Cond {
    /// x, m, a, s
    cat: usize,
    /// true = lt, false = gt
    lt: bool,
    num: i32,
    to: String,
}

pub fn run() {
    let file = read!(str);
    let (left, right) = file.split_once("\n\n").unwrap();
    let workflows: HashMap<String, WorkFlow> = left
        .split("\n")
        .map(|line| {
            let (name, rest) = line.split_once("{").unwrap();
            // remove }
            let rest = rest[..rest.len() - 1].split(",");

            let mut conds = Vec::new();
            let mut last = String::new();

            for cond in rest {
                if 2 >= cond.len() {
                    last = cond.into();
                    break;
                }

                if let Some((num, to)) = cond[2..].split_once(":") {
                    let cat = match &cond[0..1] {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => unreachable!("{}", &cond[0..1]),
                    };
                    let lt = &cond[1..2] == "<";
                    let num = num.parse().unwrap();
                    conds.push(Cond {
                        cat,
                        lt,
                        num,
                        to: to.into(),
                    });
                } else {
                    last = cond.into();
                }
            }

            (name.into(), WorkFlow { conds, last })
        })
        .collect();

    let parts: Vec<Vec<i32>> = right
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split(",")
                .map(|num| {
                    // remove {}
                    num.split_once("=").unwrap().1.parse().unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let sum: i32 = parts
        .iter()
        .filter_map(|part| {
            if is_accepted(&part, &workflows, "in") {
                Some(part.iter().copied().sum::<i32>())
            } else {
                None
            }
        })
        .sum();

    println!("Part1: {sum}");
    println!("Part2: {}", bfs_range(&workflows));
}

/// begin at in
fn is_accepted(part: &[i32], map: &HashMap<String, WorkFlow>, name: &str) -> bool {
    let workflow = &map[name];

    for cond in &workflow.conds {
        let condition = if cond.lt {
            part[cond.cat] < cond.num
        } else {
            part[cond.cat] > cond.num
        };

        if condition {
            if cond.to == "A" {
                return true;
            } else if cond.to == "R" {
                return false;
            }

            return is_accepted(part, map, &cond.to);
        }
    }

    if workflow.last == "A" {
        return true;
    } else if workflow.last == "R" {
        return false;
    }

    is_accepted(part, map, &workflow.last)
}

#[derive(Debug)]
struct State<'a> {
    xmas: [(i32, i32); 4],
    workflow: &'a str,
    /// index of the one to consider
    index: usize,
}

/// equation solver:
/// go backwards from in, and add a bound from min (1) to max (4000) based on conditions
/// ```txt
/// [5, 7] => 3 (7 - 5 + 1)
/// ```
fn bfs_range(workflows: &HashMap<String, WorkFlow>) -> i64 {
    let mut queue = VecDeque::new();

    queue.push_front(State {
        workflow: "in",
        // inclusive
        xmas: [(1, 4000); 4],
        index: 0,
    });

    let mut sum = 0;

    while let Some(State {
        workflow: workflow_name,
        xmas,
        index,
    }) = queue.pop_front()
    {
        // sometimes could get to impossible state I suppose
        let mut valid = true;
        for x in xmas {
            if x.0 >= x.1 {
                valid = false;
                break;
            }
        }
        if !valid {
            continue;
        }

        // A means done
        if workflow_name == "A" {
            sum += xmas_spirit(xmas);
            continue;
        }

        if workflow_name == "R" {
            continue;
        }

        let workflow = &workflows[workflow_name];

        // last
        if index >= workflow.conds.len() {
            queue.push_front(State {
                xmas,
                workflow: &workflow.last,
                index: 0,
            });
            continue;
        }

        let cond = &workflow.conds[index];

        // we need to "recursively" make sure the next rule range
        // includes the inverse of the previous rule
        let mut xmas = xmas;
        let mut inverse_xmas = xmas;
        if cond.lt {
            // < so max should be < n
            xmas[cond.cat].1 = cond.num - 1;

            // >= so min should be = n
            inverse_xmas[cond.cat].0 = cond.num;
        } else {
            // > so min should be > n
            xmas[cond.cat].0 = cond.num + 1;

            // <= so max should be = n
            inverse_xmas[cond.cat].1 = cond.num;
        }

        // this assumes condition was met
        queue.push_front(State {
            xmas,
            workflow: &cond.to,
            index: 0,
        });

        // this assumes we go to next condition (condition NOT met)
        queue.push_front(State {
            xmas: inverse_xmas,
            workflow: workflow_name,
            index: index + 1,
        });
    }

    sum
}

fn xmas_spirit(xmas: [(i32, i32); 4]) -> i64 {
    xmas.iter()
        .map(|(min, max)| (max - min + 1) as i64)
        .product()
}
