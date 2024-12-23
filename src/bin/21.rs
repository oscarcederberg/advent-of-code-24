use std::collections::HashMap;

use memoize::memoize;

const TEST: &'static str = include_str!("../../input/21/test.txt");
const INPUT: &'static str = include_str!("../../input/21/input.txt");

#[memoize]
fn next_keypad_to_sequence(state: char, next: char) -> Vec<Vec<char>> {
    let mapping = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);
    let mut sequences = Vec::new();
    let (x_1, y_1) = *mapping.get(&state).unwrap();
    let (x_2, y_2) = *mapping.get(&next).unwrap();
    let (dx, dy) = (x_2 - x_1, y_2 - y_1);

    let mut segfault = false;
    let mut sequence = Vec::new();
    let (mut x, mut y) = (x_1, y_1);
    if dx > 0 {
        for _ in 0..dx {
            x += 1;
            sequence.push('>');
            if x == 0 && y == 3 {
                segfault = true;
                break;
            }
        }
    } else if dx < 0 {
        for _ in 0..(-dx) {
            x -= 1;
            sequence.push('<');
            if x == 0 && y == 3 {
                segfault = true;
                break;
            }
        }
    }

    if segfault {
        sequence.clear();
        segfault = false;
    } else {
        if dy > 0 {
            for _ in 0..dy {
                y += 1;
                sequence.push('v');
                if x == 0 && y == 3 {
                    segfault = true;
                    break;
                }
            }
        } else if dy < 0 {
            for _ in 0..(-dy) {
                y -= 1;
                sequence.push('^');
                if x == 0 && y == 3 {
                    segfault = true;
                    break;
                }
            }
        }
        if !segfault {
            sequence.push('A');
            sequences.push(sequence);
            segfault = false;
        }
    }

    let mut sequence = Vec::new();
    let (mut x, mut y) = (x_1, y_1);
    if dy > 0 {
        for _ in 0..dy {
            y += 1;
            sequence.push('v');
            if x == 0 && y == 3 {
                segfault = true;
                break;
            }
        }
    } else if dy < 0 {
        for _ in 0..(-dy) {
            y -= 1;
            sequence.push('^');
            if x == 0 && y == 3 {
                segfault = true;
                break;
            }
        }
    }

    if segfault {
        sequence.clear();
    } else {
        if dx > 0 {
            for _ in 0..dx {
                x += 1;
                sequence.push('>');
                if x == 0 && y == 3 {
                    segfault = true;
                    break;
                }
            }
        } else if dx < 0 {
            for _ in 0..(-dx) {
                x -= 1;
                sequence.push('<');
                if x == 0 && y == 3 {
                    segfault = true;
                    break;
                }
            }
        }
        if !segfault {
            sequence.push('A');
            if sequences.len() == 0 || sequence != sequences[0] {
                sequences.push(sequence);
            }
        }
    }

    sequences
}

#[memoize]
fn next_sequence_to_sequence(state: char, next: char) -> Vec<Vec<char>> {
    let mapping = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut sequences = Vec::new();
    let (x_1, y_1) = *mapping.get(&state).unwrap();
    let (x_2, y_2) = *mapping.get(&next).unwrap();
    let (dx, dy) = (x_2 - x_1, y_2 - y_1);

    let mut segfault = false;
    let mut sequence = Vec::new();
    let (mut x, mut y) = (x_1, y_1);
    if dx > 0 {
        for _ in 0..dx {
            x += 1;
            sequence.push('>');
            if x == 0 && y == 0 {
                segfault = true;
                break;
            }
        }
    } else if dx < 0 {
        for _ in 0..(-dx) {
            x -= 1;
            sequence.push('<');
            if x == 0 && y == 0 {
                segfault = true;
                break;
            }
        }
    }

    if segfault {
        sequence.clear();
        segfault = false;
    } else {
        if dy > 0 {
            for _ in 0..dy {
                y += 1;
                sequence.push('v');
                if x == 0 && y == 0 {
                    segfault = true;
                    break;
                }
            }
        } else if dy < 0 {
            for _ in 0..(-dy) {
                y -= 1;
                sequence.push('^');
                if x == 0 && y == 0 {
                    segfault = true;
                    break;
                }
            }
        }
        if !segfault {
            sequence.push('A');
            sequences.push(sequence);
            segfault = false;
        }
    }

    let mut sequence = Vec::new();
    let (mut x, mut y) = (x_1, y_1);
    if dy > 0 {
        for _ in 0..dy {
            y += 1;
            sequence.push('v');
            if x == 0 && y == 0 {
                segfault = true;
                break;
            }
        }
    } else if dy < 0 {
        for _ in 0..(-dy) {
            y -= 1;
            sequence.push('^');
            if x == 0 && y == 0 {
                segfault = true;
                break;
            }
        }
    }

    if segfault {
        sequence.clear();
    } else {
        if dx > 0 {
            for _ in 0..dx {
                x += 1;
                sequence.push('>');
                if x == 0 && y == 0 {
                    segfault = true;
                    break;
                }
            }
        } else if dx < 0 {
            for _ in 0..(-dx) {
                x -= 1;
                sequence.push('<');
                if x == 0 && y == 0 {
                    segfault = true;
                    break;
                }
            }
        }

        if !segfault {
            sequence.push('A');
            if sequences.len() == 0 || sequence != sequences[0] {
                sequences.push(sequence);
            }
        }
    }

    sequences
}

#[memoize]
fn solve(current: char, next: char, bots: usize, max: usize) -> usize {
    let sequences = if bots == max {
        next_keypad_to_sequence(current, next)
    } else {
        next_sequence_to_sequence(current, next)
    };

    if bots == 1 {
        return sequences[0].len();
    }

    sequences
        .iter()
        .map(|sequence| {
            sequence
                .iter()
                .fold(('A', 0), |(current, len), next| {
                    (*next, len + solve(current, *next, bots - 1, max))
                })
                .1
        })
        .min()
        .unwrap()
}

fn part_1(input: &str) -> usize {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    codes
        .iter()
        .map(|code| {
            let len = code
                .iter()
                .fold(('A', 0), |(current, len), next| {
                    (*next, len + solve(current, *next, 3, 3))
                })
                .1;
            len * code[0..3]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let codes: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    codes
        .iter()
        .map(|code| {
            let len = code
                .iter()
                .fold(('A', 0), |(current, len), next| {
                    (*next, len + solve(current, *next, 26, 26))
                })
                .1;
            len * code[0..3]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn main() {
    println!("day 21");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
