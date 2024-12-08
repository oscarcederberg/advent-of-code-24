use std::collections::HashSet;

use crate::{Direction::*, Space::*};

const TEST: &'static str = include_str!("../../input/06/test.txt");
const INPUT: &'static str = include_str!("../../input/06/input.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Unvisited,
    Visited,
    Wall,
}

fn part_1(input: &str) -> i32 {
    let (mut visited, mut direction, mut x, mut y): (_, _, i32, i32) = (1, Up, 0, 0);
    let mut map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| match symbol {
                    '.' => Unvisited,
                    '#' => Wall,
                    '^' => {
                        (x, y) = (col as i32, row as i32);
                        Visited
                    }
                    _ => unreachable!("Unknown character found in input: {:?}", symbol),
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<Vec<Space>>>();
    let (height, width) = (map.len() as i32, map[0].len() as i32);

    loop {
        let (dx, dy) = match direction {
            Left => (-1, 0),
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
        };

        if x + dx < 0 || x + dx >= width || y + dy < 0 || y + dy >= height {
            break;
        }

        match map[(y + dy) as usize][(x + dx) as usize] {
            Unvisited => {
                visited += 1;
                (x, y) = (x + dx, y + dy);
                map[y as usize][x as usize] = Visited;
            }
            Visited => (x, y) = (x + dx, y + dy),
            Wall => {
                direction = match direction {
                    Left => Up,
                    Up => Right,
                    Right => Down,
                    Down => Left,
                }
            }
        }
    }

    visited
}

fn simulate(mut map: Vec<Vec<Space>>, mut direction: Direction, mut x: i32, mut y: i32) -> bool {
    let (height, width) = (map.len() as i32, map[0].len() as i32);
    let mut visited: Vec<Vec<HashSet<Direction>>> = (0..height)
        .map(|_| (0..width).map(|_| HashSet::new()).collect())
        .collect();

    loop {
        visited[(y) as usize][(x) as usize].insert(direction.clone());
        map[(y) as usize][(x) as usize] = Visited;

        let (dx, dy) = match direction {
            Left => (-1, 0),
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
        };

        if x + dx < 0 || x + dx >= width || y + dy < 0 || y + dy >= height {
            return false;
        }

        match map[(y + dy) as usize][(x + dx) as usize] {
            Unvisited => {
                (x, y) = (x + dx, y + dy);
            }
            Visited => {
                (x, y) = (x + dx, y + dy);
                if visited[y as usize][x as usize].contains(&direction) {
                    return true;
                }
            }
            Wall => {
                direction = match direction {
                    Left => Up,
                    Up => Right,
                    Right => Down,
                    Down => Left,
                }
            }
        }
    }
}

fn part_2(input: &str) -> i32 {
    let (mut direction, mut x, mut y): (_, i32, i32) = (Up, 0, 0);
    let original = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| match symbol {
                    '.' => Unvisited,
                    '#' => Wall,
                    '^' => {
                        (x, y) = (col as i32, row as i32);
                        Visited
                    }
                    _ => unreachable!("Unknown character found in input: {:?}", symbol),
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<Vec<Space>>>();
    let (height, width) = (original.len() as i32, original[0].len() as i32);
    let (start_x, start_y) = (x, y);

    let mut map = original.clone();
    loop {
        let (dx, dy) = match direction {
            Left => (-1, 0),
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
        };

        if x + dx < 0 || x + dx >= width || y + dy < 0 || y + dy >= height {
            break;
        }

        match map[(y + dy) as usize][(x + dx) as usize] {
            Unvisited => {
                (x, y) = (x + dx, y + dy);
                map[y as usize][x as usize] = Visited;
            }
            Visited => (x, y) = (x + dx, y + dy),
            Wall => {
                direction = match direction {
                    Left => Up,
                    Up => Right,
                    Right => Down,
                    Down => Left,
                }
            }
        }
    }

    map.iter()
        .enumerate()
        .map(|(wall_y, row)| {
            row.iter()
                .enumerate()
                .filter(|(wall_x, space)| {
                    if **space != Visited
                        || (*wall_x == start_x as usize && wall_y == start_y as usize)
                    {
                        false
                    } else {
                        let mut copy = original.clone();
                        copy[wall_y as usize][*wall_x as usize] = Wall;
                        simulate(copy, Up, start_x, start_y)
                    }
                })
                .count()
        })
        .fold(0, |count, value| count + value as i32)
}

fn main() {
    println!("day 6");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
