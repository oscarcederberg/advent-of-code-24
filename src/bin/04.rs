use std::{char, collections::VecDeque};

const TEST: &'static str = include_str!("../../input/04/test.txt");
const INPUT: &'static str = include_str!("../../input/04/input.txt");

fn push_char(word: &mut VecDeque<char>, character: char) -> bool {
    word.push_back(character);

    if word.len() < 4 {
        return false;
    }

    let result = match word.iter().collect::<Vec<_>>()[..] {
        ['X', 'M', 'A', 'S'] => true,
        ['S', 'A', 'M', 'X'] => true,
        _ => false,
    };

    word.pop_front();

    result
}

fn part_1(input: &str) -> i32 {
    let mut count = 0;
    let chars: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut queue = VecDeque::with_capacity(4);
    let rows = chars.len();
    let cols = chars[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if push_char(&mut queue, chars[row][col]) {
                count += 1;
            }
        }

        queue.clear()
    }

    for col in 0..cols {
        for row in 0..rows {
            if push_char(&mut queue, chars[row][col]) {
                count += 1;
            }
        }

        queue.clear()
    }

    for diagonal in 3..std::cmp::min(rows, cols) {
        for row in 0..diagonal {
            if push_char(&mut queue, chars[row][diagonal - row]) {
                count += 1;
            }
        }

        queue.clear();

        for row in 0..diagonal {
            if push_char(
                &mut queue,
                chars[rows - row - 1][cols - (diagonal - row) - 1],
            ) {
                count += 1;
            }
        }
        queue.clear();
    }

    count
}

fn part_2(input: &str) -> i32 {
    0
}

fn main() {
    println!("day 4");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
