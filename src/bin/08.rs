use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const TEST: &'static str = include_str!("../../input/08/test.txt");
const INPUT: &'static str = include_str!("../../input/08/input.txt");

fn part_1(input: &str) -> usize {
    let mut locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (height, width) = (map.len() as i32, map[0].len() as i32);

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, symbol)| match symbol {
            '.' => (),
            symbol => locations
                .entry(*symbol)
                .or_insert_with(Vec::new)
                .push((x as i32, y as i32)),
        })
    });

    locations
        .iter()
        .fold(HashSet::new(), |antinodes, (_frequency, locations)| {
            locations
                .iter()
                .permutations(2)
                .fold(antinodes, |mut antinodes, positions| {
                    let (x_1, y_1) = positions[0];
                    let (x_2, y_2) = positions[1];
                    let (dx, dy) = (x_2 - x_1, y_2 - y_1);
                    antinodes.insert((x_1 - dx, y_1 - dy));
                    antinodes.insert((x_2 + dx, y_2 + dy));
                    antinodes
                })
        })
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
        .count()
}

fn part_2(input: &str) -> usize {
    let mut locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (height, width) = (map.len() as i32, map[0].len() as i32);

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, symbol)| match symbol {
            '.' => (),
            symbol => locations
                .entry(*symbol)
                .or_insert_with(Vec::new)
                .push((x as i32, y as i32)),
        })
    });

    locations
        .iter()
        .fold(HashSet::new(), |antinodes, (_frequency, locations)| {
            locations
                .iter()
                .permutations(2)
                .fold(antinodes, |mut antinodes, positions| {
                    let (x_1, y_1) = positions[0];
                    let (x_2, y_2) = positions[1];

                    antinodes.insert((*x_1, *y_1));
                    antinodes.insert((*x_2, *y_2));

                    let (dx, dy) = (*x_2 - *x_1, *y_2 - *y_1);

                    let (mut x, mut y) = (*x_1 - dx, *y_1 - dy);
                    while x >= 0 && x < width && y >= 0 && y < height {
                        antinodes.insert((x, y));
                        (x, y) = (x - dx, y - dy);
                    }

                    (x, y) = (*x_2 + dx, *y_2 + dy);
                    while x >= 0 && x < width && y >= 0 && y < height {
                        antinodes.insert((x, y));
                        (x, y) = (x + dx, y + dy);
                    }

                    antinodes
                })
        })
        .len()
}

fn main() {
    println!("day 8");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
