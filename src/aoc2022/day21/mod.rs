use std::collections::{HashMap, VecDeque};

use crate::read;

#[derive(Debug, Clone, Copy)]
enum OpKind {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, Clone, Copy)]
enum Monkey<'a> {
    Num(i64),
    Op(&'a str, OpKind, &'a str),
}

use Monkey::*;
use OpKind::*;

pub fn run() {
    let file = read!();

    let graph: HashMap<String, Monkey> = file
        .map(|line| {
            let (name, right) = line.split_once(": ").unwrap();

            (
                name.into(),
                if let Ok(val) = right.parse() {
                    Num(val)
                } else {
                    let mut ops = right.split(" ");
                    let left = ops.next().unwrap().into();
                    let op = ops.next().unwrap();
                    let right = ops.next().unwrap().into();

                    match op {
                        "+" => Op(left, Plus, right),
                        "-" => Op(left, Minus, right),
                        "*" => Op(left, Times, right),
                        "/" => Op(left, Divide, right),
                        _ => unreachable!("{op}"),
                    }
                },
            )
        })
        .collect();

    println!("Part1: {}", val("root", &graph));

    // the right seems to NEVER use humn
    let Op(left, _, right) = graph["root"] else { unreachable!("should be +") };
    let right_val = val(right, &graph);

    println!("Part2: {}", bfs_solver(left, right_val, &graph));
}

fn val(name: &str, graph: &HashMap<String, Monkey>) -> i64 {
    match graph[name] {
        Num(n) => n,
        Op(a, kind, b) => {
            let left = val(&a, graph);
            let right = val(&b, graph);
            match kind {
                Plus => left + right,
                Minus => left - right,
                Times => left * right,
                Divide => left / right,
            }
        }
    }
}

struct State<'a> {
    /// left side of the equation (as a name)
    name: &'a str,
    /// right side of the equation
    expect: i64,
}

/// equation solver (apply these algorithms, and you'll unwrap to the solution):
/// ```txt
/// (...) * 5 = n, (...) = n / 5
/// (...) / 5 = n, (...) = n * 5
/// (...) + 5 = n, (...) = n - 5
/// (...) - 5 = n, (...) = n + 5
/// ```
///
/// NOTE: x only appears once, so a linear equation
/// NOTE: can use `val` because the base case is reaching `x`
/// so if the branch reaches x, it won't be used
///
/// Use BFS, end case = `humn` on some side
fn bfs_solver(left: &str, expect: i64, graph: &HashMap<String, Monkey>) -> i64 {
    let mut queue = VecDeque::new();

    // this stupid code is so stupid
    // this should be push_front but stupid troll me decides to do push_back
    queue.push_back(State { name: left, expect });

    while let Some(State { name, expect }) = queue.pop_front() {
        // base case
        if name == "humn" {
            return expect;
        }

        match graph[name] {
            Num(_) => {} // not sure what to do here
            Op(left, op, rname) => {
                let lval = val(left, &graph);
                let rval = val(rname, &graph);

                // left [op] right = expect
                // left = expect ?? [right]
                queue.push_front(State {
                    name: left,
                    expect: match op {
                        Plus => expect - rval,
                        Minus => expect + rval,
                        Times => expect / rval,
                        Divide => expect * rval,
                    },
                });

                // left [op] right = expect
                // right = expect ?? [left]
                queue.push_front(State {
                    name: rname,
                    expect: match op {
                        Plus => expect - lval,
                        Minus => -expect + lval,
                        Times => expect / lval,
                        Divide => lval / expect,
                    },
                });
            }
        }
    }

    unreachable!("skill issue lmao")
}

fn _print_equation(name: &str, graph: &HashMap<String, Monkey>) -> String {
    if name == "humn" {
        return "x".into();
    }
    match graph[name] {
        Num(n) => n.to_string(),
        Op(a, kind, b) => format!(
            "({}) {} ({})",
            _print_equation(&a, graph),
            match kind {
                Plus => "+",
                Minus => "-",
                Times => "*",
                Divide => "/",
            },
            _print_equation(&b, graph)
        ),
    }
}
