use memoize::memoize;

const TEST: &'static str = include_str!("../../input/11/test.txt");
const INPUT: &'static str = include_str!("../../input/11/input.txt");

fn digits(value: &u64) -> usize {
    format!("{}", *value).len()
}

fn split(value: &u64) -> (u64, u64) {
    let mut string = format!("{}", *value);
    let len = string.len() / 2;
    let split = string.split_at_mut(len);
    (split.0.parse().unwrap(), split.1.parse().unwrap())
}

#[memoize]
fn match_stone(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        value if digits(&value) % 2 == 0 => {
            let split = split(&value);
            vec![split.0, split.1]
        }
        value => {
            vec![2024 * value]
        },
    }
}

fn part_1(input: &str) -> usize {
    let mut stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|word| word.parse().unwrap())
        .collect();
    for _ in 0..25 {
        stones = stones
            .iter_mut()
            .flat_map(|stone|{
                match_stone(*stone)
            }).collect();
    }

    stones.len()
}

fn dfs(stone: u64, iter: usize, max_iter: usize, count: &mut usize) {
    if iter == max_iter {
        *count += 1;
        return
    }

    match_stone(stone).iter().for_each(|stone| dfs(*stone, iter + 1, max_iter, count));
}

fn part_2(input: &str) -> usize {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|word| word.parse().unwrap())
        .collect();
    let mut count = 0;

    stones.iter().for_each(|stone| dfs(*stone, 0, 75, &mut count));

    count
}

fn main() {
    println!("day 11");
    //println!("part 1 (test): {}", part_1(TEST));
    //println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
