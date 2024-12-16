use std::{cell::RefCell, usize};

const TEST: &'static str = include_str!("../../input/16/test.txt");
const INPUT: &'static str = include_str!("../../input/16/input.txt");

#[derive(PartialEq, Eq)]
enum Symbol {
    Empty,
    Wall,
    Goal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    West,
    North,
    East,
    South,
}

fn dfs(
    position: (isize, isize),
    direction: Direction,
    has_rotated: bool,
    map: &Vec<Vec<Symbol>>,
    goal: (isize, isize),
    current_score: usize,
    lowest_score: RefCell<usize>,
) {
    if current_score > *lowest_score.borrow() {
        return;
    }

    if position == goal {
        *lowest_score.borrow_mut() = current_score;
        return;
    }

    for (dir, anti, dx, dy) in [
        (Direction::West, Direction::East, -1, 0),
        (Direction::North, Direction::South, 0, -1),
        (Direction::East, Direction::West, 1, 0),
        (Direction::South, Direction::North, 0, 1),
    ] {
        if dir != direction && has_rotated {
            continue;
        } else if dir == direction {
            if map[(position.1 + dy) as usize][(position.0 + dx) as usize] == Symbol::Wall {
                continue;
            }
            dfs(
                (position.0 + dx, position.1 + dy),
                direction,
                false,
                map,
                goal,
                current_score + 1,
                lowest_score.clone(),
            );
        } else if anti != direction {
            if map[(position.1 + dy) as usize][(position.0 + dx) as usize] == Symbol::Wall {
                continue;
            }
            dfs(
                (position.0 + dx, position.1 + dy),
                dir,
                true,
                map,
                goal,
                current_score + 1001,
                lowest_score.clone(),
            );
        }
    }
}

fn part_1(input: &str) -> usize {
    let (mut reindeer_x, mut reindeer_y) = (0, 0);
    let (mut goal_x, mut goal_y) = (0, 0);
    let lowest_score = RefCell::new(usize::MAX);
    let map: Vec<Vec<Symbol>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| match symbol {
                    '.' => Symbol::Empty,
                    '#' => Symbol::Wall,
                    'E' => {
                        (goal_x, goal_y) = (col as isize, row as isize);
                        Symbol::Goal
                    }
                    'S' => {
                        (reindeer_x, reindeer_y) = (col as isize, row as isize);
                        Symbol::Empty
                    }
                    _ => unreachable!("found non-handled symbol: {}", symbol),
                })
                .collect()
        })
        .collect();

    dfs(
        (reindeer_x, reindeer_y),
        Direction::East,
        false,
        &map,
        (goal_x, goal_y),
        0,
        lowest_score.clone(),
    );
    0
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    println!("day 16");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
