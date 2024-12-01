use std::{collections::HashMap, iter::zip};

const TEST: &'static str = include_str!("../../input/01/test.txt");
const INPUT: &'static str = include_str!("../../input/01/input.txt");

fn part_1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|x| x.split("   ").collect::<Vec<&str>>())
        .map(|x| (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap()))
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    zip(left, right).map(|(a, b)| a.abs_diff(b)).sum()
}

fn part_2(input: &str) -> i32 {
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|x| x.split("   ").collect::<Vec<&str>>())
        .map(|x| (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap()))
        .unzip();
    let counter: HashMap<i32, i32> = right.into_iter().fold(HashMap::new(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });
    left.into_iter().map(|x| x * (*counter.get(&x).unwrap_or(&0))).sum()
}

fn main() {
    println!("day 1");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}