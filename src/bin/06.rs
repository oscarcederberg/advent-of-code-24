use crate::{Direction::*, Space::*};

const TEST: &'static str = include_str!("../../input/06/test.txt");
const INPUT: &'static str = include_str!("../../input/06/input.txt");

#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
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

fn part_2(input: &str) -> i32 {
    0
}

fn main() {
    println!("day 6");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
