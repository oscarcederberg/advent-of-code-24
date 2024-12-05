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

    for diagonal in 0..(rows + cols - 2) {
        for col in 0..=diagonal {
            let row = diagonal - col;
            if row < rows && col < cols {
                if push_char(&mut queue, chars[row][col]) {
                    count += 1;
                }
            }
        }

        queue.clear();

        for col in 0..=diagonal {
            let row = diagonal - col;
            if row < rows && col < cols {
                if push_char(&mut queue, chars[rows - row - 1][col]) {
                    count += 1;
                }
            }
        }

        queue.clear();
    }

    count
}

fn part_2(input: &str) -> i32 {
    let mut count = 0;
    let chars: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    for row in 0..(chars.len() - 2) {
        for col in 0..(chars[0].len() - 2) {
            match [(0, 0), (1, 1), (2, 2), (0, 2), (2, 0)]
                .iter()
                .map(|(r, c)| chars[row + r][col + c])
                .collect::<Vec<_>>()[..]
            {
                ['M', 'A', 'S', 'M', 'S'] => count += 1,
                ['M', 'A', 'S', 'S', 'M'] => count += 1,
                ['S', 'A', 'M', 'M', 'S'] => count += 1,
                ['S', 'A', 'M', 'S', 'M'] => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn main() {
    println!("day 4");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
