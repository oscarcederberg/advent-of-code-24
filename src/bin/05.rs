use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

const TEST: &'static str = include_str!("../../input/05/test.txt");
const INPUT: &'static str = include_str!("../../input/05/input.txt");

fn part_1(input: &str) -> i32 {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let ordering = ordering
        .lines()
        .map(|o| {
            let tuple = o.split_once('|').unwrap();
            (
                tuple.0.parse::<i32>().unwrap(),
                tuple.1.parse::<i32>().unwrap(),
            )
        })
        .fold(
            HashMap::<i32, HashSet<i32>>::with_capacity(ordering.len()),
            |mut map, (left, right)| {
                map.entry(left).or_insert(HashSet::new()).insert(right);
                map
            },
        );
    updates
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|update| {
            let mut queue = VecDeque::from(update.clone());
            for _ in 0..update.len() {
                let current = queue.pop_front().unwrap();
                if !queue.iter().all(|after| {
                    ordering
                        .get(&current)
                        .or(Some(&HashSet::new()))
                        .unwrap()
                        .contains(after)
                }) {
                    return false;
                }
            }
            true
        })
        .fold(0, |count, update| {
            count + update.get(update.len() / 2).unwrap()
        })
}

fn part_2(input: &str) -> i32 {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let ordering = ordering
        .lines()
        .map(|o| {
            let tuple = o.split_once('|').unwrap();
            (
                tuple.0.parse::<i32>().unwrap(),
                tuple.1.parse::<i32>().unwrap(),
            )
        })
        .fold(
            HashMap::<i32, HashSet<i32>>::with_capacity(ordering.len()),
            |mut map, (left, right)| {
                map.entry(left).or_insert(HashSet::new()).insert(right);
                map
            },
        );
    updates
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|update| {
            let mut queue = VecDeque::from(update.clone());
            for _ in 0..update.len() {
                let current = queue.pop_front().unwrap();
                if !queue.iter().all(|after| {
                    ordering
                        .get(&current)
                        .or(Some(&HashSet::new()))
                        .unwrap()
                        .contains(after)
                }) {
                    return true;
                }
            }
            false
        })
        .map(|mut update| {
            update.sort_unstable_by(|a, b| {
                if ordering
                    .get(&a)
                    .or(Some(&HashSet::new()))
                    .unwrap()
                    .contains(b)
                {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            update
        })
        .fold(0, |count, update| {
            count + update.get(update.len() / 2).unwrap()
        })
}

fn main() {
    println!("day 5");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
