use std::collections::HashSet;

const TEST: &'static str = include_str!("../../input/02/test.txt");
const INPUT: &'static str = include_str!("../../input/02/input.txt");

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
                .fold(
                    (HashSet::from([1, 2, 3]), HashSet::from([-1, -2, -3])),
                    |(mut inc_set, mut dec_set), values| {
                        let diff = values[1] - values[0];
                        inc_set.insert(diff);
                        dec_set.insert(diff);
                        (inc_set, dec_set)
                    },
                )
        })
        .fold(0, |count, (inc_set, dec_set)| {
            if inc_set.len() == 3 || dec_set.len() == 3 {
                count + 1
            } else {
                count
            }
        })
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|vec| {
            let mut combinations: Vec<Vec<i32>> = Vec::new();
            for i in 0..vec.len() {
                let mut copy = vec.clone();
                copy.remove(i);
                combinations.push(copy);
            }
            combinations
                .into_iter()
                .map(|vec| {
                    vec.windows(2).fold(
                        (HashSet::from([1, 2, 3]), HashSet::from([-1, -2, -3])),
                        |(mut inc_set, mut dec_set), values| {
                            let diff = values[1] - values[0];
                            inc_set.insert(diff);
                            dec_set.insert(diff);
                            (inc_set, dec_set)
                        },
                    )
                })
                .any(|(inc_set, dec_set)| inc_set.len() == 3 || dec_set.len() == 3)
        })
        .fold(0, |count, safe| if safe { count + 1 } else { count })
}

fn main() {
    println!("day 2");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
