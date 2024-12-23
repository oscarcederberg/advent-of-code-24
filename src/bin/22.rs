use std::collections::{HashMap, HashSet, VecDeque};

const TEST_1: &'static str = include_str!("../../input/22/test_1.txt");
const TEST_2: &'static str = include_str!("../../input/22/test_2.txt");
const INPUT: &'static str = include_str!("../../input/22/input.txt");

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = (secret ^ (secret * 64)).rem_euclid(16777216);
                secret = (secret ^ (secret / 32)).rem_euclid(16777216);
                secret = (secret ^ (secret * 2048)).rem_euclid(16777216);
            }
            secret
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut sum_per_delta: HashMap<[i32; 4], usize> = HashMap::new();
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .for_each(|mut secret| {
            let mut last_price = secret.rem_euclid(10) as i32;
            let mut delta_sequence: VecDeque<i32> = VecDeque::new();
            let mut delta_sequences: HashSet<[i32; 4]> = HashSet::new();

            for _ in 0..2000 {
                secret = (secret ^ (secret * 64)).rem_euclid(16777216);
                secret = (secret ^ (secret / 32)).rem_euclid(16777216);
                secret = (secret ^ (secret * 2048)).rem_euclid(16777216);

                let price = secret.rem_euclid(10) as i32;
                let delta = price - last_price;

                last_price = price;
                delta_sequence.push_back(delta);
                if delta_sequence.len() == 4 {
                    let sequence = [
                        delta_sequence[0],
                        delta_sequence[1],
                        delta_sequence[2],
                        delta_sequence[3],
                    ];
                    delta_sequence.pop_front();
                    if delta_sequences.insert(sequence) {
                        *sum_per_delta.entry(sequence).or_insert(0) += price as usize;
                    }
                }
            }
        });

    *sum_per_delta.values().max().unwrap()
}

fn main() {
    println!("day 22");
    println!("part 1 (test): {}", part_1(TEST_1));
    println!("part 2 (test): {}", part_2(TEST_2));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
