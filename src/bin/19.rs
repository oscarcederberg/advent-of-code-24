use memoize::memoize;

const TEST: &'static str = include_str!("../../input/19/test.txt");
const INPUT: &'static str = include_str!("../../input/19/input.txt");

#[memoize]
fn is_possible(pattern: String, towels: Vec<String>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for towel in &towels {
        if let Some(pattern) = pattern.strip_prefix(towel) {
            if is_possible(pattern.to_owned(), towels.clone()) {
                return true;
            }
        }
    }

    false
}

fn part_1(input: &str) -> usize {
    let input = input.split_once("\n\n").unwrap();
    let (towels, patterns): (Vec<String>, Vec<String>) = (
        input.0.split(", ").map(|towel| towel.to_owned()).collect(),
        input.1.lines().map(|pattern| pattern.to_owned()).collect(),
    );

    patterns
        .iter()
        .filter(|pattern| is_possible((*pattern).clone(), towels.clone()))
        .count()
}

#[memoize]
fn count_possible(pattern: String, towels: Vec<String>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    towels
        .iter()
        .map(|towel| {
            if let Some(pattern) = pattern.strip_prefix(towel) {
                count_possible(pattern.to_owned(), towels.clone())
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let input = input.split_once("\n\n").unwrap();
    let (towels, patterns): (Vec<String>, Vec<String>) = (
        input.0.split(", ").map(|towel| towel.to_owned()).collect(),
        input.1.lines().map(|pattern| pattern.to_owned()).collect(),
    );

    patterns
        .iter()
        .map(|pattern| count_possible((*pattern).clone(), towels.clone()))
        .sum()
}

fn main() {
    println!("day 19");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
