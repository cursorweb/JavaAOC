#![allow(unused)]
use std::collections::HashMap;

use crate::read;

#[derive(Debug, Clone, Copy)]
enum Monkey<'a> {
    Num(i64),
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Times(&'a str, &'a str),
    Divide(&'a str, &'a str),
}

use Monkey::*;

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
                        "+" => Plus(left, right),
                        "-" => Minus(left, right),
                        "*" => Times(left, right),
                        "/" => Divide(left, right),
                        _ => unreachable!("{op}"),
                    }
                },
            )
        })
        .collect();

    // println!("Part1: {}", val("root", &graph));

    let mut graph = graph;

    // the right seems to NEVER use humn
    let Plus(left, right) = graph["root"] else { unreachable!("should be +") };
    let right_val = val(right, &graph);

    for i in 0..100 {
        graph.insert("humn".into(), Num(num1));
        let y1 = val(left, &graph);
    }

    /*
    let num1 = 0;
    let num2 = 100;

    graph.insert("humn".into(), Num(num1));
    let y1 = val(left, &graph);

    graph.insert("humn".into(), Num(num2));
    let y2 = val(left, &graph);

    let slope = (y2 - y1) / (num2 - num1);

    // this is our equation
    // y1 + slope * x
    println!("{y1} {slope}x");

    // what is our intercept?
    let x = (right_val - y1) / slope;
    */
}

fn val(name: &str, graph: &HashMap<String, Monkey>) -> i64 {
    match graph[name] {
        Num(n) => n,
        Plus(a, b) => val(&a, graph) + val(&b, graph),
        Minus(a, b) => val(&a, graph) - val(&b, graph),
        Times(a, b) => val(&a, graph) * val(&b, graph),
        Divide(a, b) => val(&a, graph) / val(&b, graph),
    }
}

fn _print_equation(name: &str, graph: &HashMap<String, Monkey>) -> String {
    if name == "humn" {
        return "x".into();
    }
    match graph[name] {
        Num(n) => n.to_string(),
        Plus(a, b) => format!(
            "({}) + ({})",
            _print_equation(&a, graph),
            _print_equation(&b, graph)
        ),
        Minus(a, b) => format!(
            "({}) - ({})",
            _print_equation(&a, graph),
            _print_equation(&b, graph)
        ),
        Times(a, b) => format!(
            "({}) * ({})",
            _print_equation(&a, graph),
            _print_equation(&b, graph)
        ),
        Divide(a, b) => format!(
            "({}) / ({})",
            _print_equation(&a, graph),
            _print_equation(&b, graph)
        ),
    }
}
