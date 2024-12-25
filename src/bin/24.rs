use crate::Gate::*;

use std::collections::{HashMap, HashSet};

const TEST: &'static str = include_str!("../../input/24/test.txt");
const INPUT: &'static str = include_str!("../../input/24/input.txt");

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Gate {
    OFF,
    ON,
    AND(String, String),
    OR(String, String),
    XOR(String, String),
}

fn solve(gates: &HashMap<String, Gate>, gate: Gate) -> bool {
    match gate {
        OFF => false,
        ON => true,
        AND(a, b) => solve(gates, gates[&a].clone()) && solve(gates, gates[&b].clone()),
        OR(a, b) => solve(gates, gates[&a].clone()) || solve(gates, gates[&b].clone()),
        XOR(a, b) => solve(gates, gates[&a].clone()) != solve(gates, gates[&b].clone()),
    }
}

fn part_1(input: &str) -> usize {
    let mut outputs = Vec::new();
    let mut gates = HashMap::new();

    let (initials, connections) = input.split_once("\n\n").unwrap();

    initials.lines().for_each(|line| {
        let line = line.trim().split(": ").collect::<Vec<&str>>();
        let (wire, value) = (line[0].to_string(), if line[1] == "0" { OFF } else { ON });
        gates.insert(wire, value);
    });

    connections.lines().for_each(|line| {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let (a, b, out) = (
            line[0].to_string(),
            line[2].to_string(),
            line[4].to_string(),
        );
        let gate = match line[1] {
            "AND" => AND(a, b),
            "OR" => OR(a, b),
            "XOR" => XOR(a, b),
            _ => unreachable!("got unhandled token: {}", line[1]),
        };

        if out.starts_with("z") {
            outputs.push(out.clone());
        }

        gates.insert(out, gate);
    });

    outputs.sort_unstable();
    outputs
        .iter()
        .map(|output| solve(&gates, gates[output].clone()))
        .enumerate()
        .fold(0, |acc, (i, b)| acc | (b as usize) << i)
}

fn part_2(input: &str) -> usize {
    let mut outputs = Vec::new();
    let mut gates = HashMap::new();
    let mut wires = HashSet::new();

    let (initials, connections) = input.split_once("\n\n").unwrap();

    initials.lines().for_each(|line| {
        let line = line.trim().split(": ").collect::<Vec<&str>>();
        let (wire, value) = (line[0].to_string(), if line[1] == "0" { OFF } else { ON });
        gates.insert(wire, value);
    });

    connections.lines().for_each(|line| {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let (a, b, out) = (
            line[0].to_string(),
            line[2].to_string(),
            line[4].to_string(),
        );
        let gate = match line[1] {
            "AND" => AND(a, b),
            "OR" => OR(a, b),
            "XOR" => XOR(a, b),
            _ => unreachable!("got unhandled token: {}", line[1]),
        };

        if out.starts_with("z") {
            outputs.push(out.clone());
        }

        gates.insert(out, gate);
    });

    gates.iter().for_each(|(wire, gate)| match gate {
        AND(a, b) => {
            if wire.starts_with("z") {
                wires.insert(wire.clone());
            } else if (a != "x00" && a != "y00") && (b != "x00" && b != "y00") {
                for (_, gate) in gates.clone() {
                    match gate {
                        AND(a, b) | XOR(a, b) => {
                            if a == *wire || b == *wire {
                                wires.insert(wire.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        OR(_, _) => {
            if wire.starts_with("z") && !wire.ends_with("45") {
                wires.insert(wire.clone());
            }
        }
        XOR(a, b) => {
            if !(a.starts_with("x") || a.starts_with("y"))
                && !(b.starts_with("x") || b.starts_with("y"))
                && !wire.starts_with("z")
            {
                wires.insert(wire.clone());
            } else {
                for (_, gate) in gates.clone() {
                    match gate {
                        OR(a, b) => {
                            if a == *wire || b == *wire {
                                wires.insert(wire.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    });

    let mut wires: Vec<_> = wires.into_iter().collect();
    wires.sort_unstable();
    println!("{}", wires.join(","));
    wires.len()
}

fn main() {
    println!("day 24");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
