#![feature(iter_array_chunks)]

use ndarray::prelude::*;
use ndarray_linalg::Solve;
use regex::Regex;

const TEST: &'static str = include_str!("../../input/13/test.txt");
const INPUT: &'static str = include_str!("../../input/13/input.txt");

fn part_1(input: &str) -> usize {
    let button_regex = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let pos_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    input
        .lines()
        .array_chunks::<4>()
        .map(|[a, b, pos, _]| {
            let (_, [a_x, a_y]) = button_regex.captures(a).unwrap().extract();
            let (_, [b_x, b_y]) = button_regex.captures(b).unwrap().extract();
            let (_, [p_x, p_y]) = pos_regex.captures(pos).unwrap().extract();
            [a_x, a_y, b_x, b_y, p_x, p_y].map(|value| value.parse().unwrap())
        })
        .map(|[a_x, a_y, b_x, b_y, p_x, p_y]| {
            let a: Array2<f64> = array![[a_x, a_y, 0.], [b_x, b_y, 0.], [p_x, p_y, -1.]];
            let b: Array1<f64> = array![3., 1., 1.];
            match a.solve_into(b) {
                Ok(array) => *array.get(2).unwrap() as usize,
                _ => 0,
            }
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    println!("day 13");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
