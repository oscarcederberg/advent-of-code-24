use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

const TEST: &'static str = include_str!("../../input/16/test.txt");
const INPUT: &'static str = include_str!("../../input/16/input.txt");

#[derive(PartialEq, Eq)]
enum Symbol {
    Empty,
    Wall,
    Goal,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    West,
    North,
    East,
    South,
}

#[allow(dead_code)]
fn print_iteration(position: (isize, isize, Direction), map: &Vec<Vec<Symbol>>) {
    println!("checking {:?}", position);
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if (position.0, position.1) == (col as isize, row as isize) {
                print!(
                    "{}",
                    match position.2 {
                        Direction::West => "<",
                        Direction::North => "^",
                        Direction::East => ">",
                        Direction::South => "v",
                    }
                )
            } else {
                print!(
                    "{}",
                    match map[row][col] {
                        Symbol::Empty => ".",
                        Symbol::Wall => "#",
                        Symbol::Goal => "E",
                    }
                )
            }
        }
        println!("");
    }
    let mut string = String::new();
    let _ = std::io::stdin()
        .read_line(&mut string)
        .expect("failed to read stdin");
}

fn part_1(input: &str) -> usize {
    let (mut reindeer_x, mut reindeer_y) = (0, 0);
    let (mut goal_x, mut goal_y) = (0, 0);
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

    let mut lowest_score = usize::MAX;
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(((reindeer_x, reindeer_y, Direction::East), 0));
    while let Some((position, score)) = queue.pop_front() {
        // print_iteration(position, &map);

        if score >= *visited.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        visited.insert(position, score);

        let (x, y, direction) = position;
        if (x, y) == (goal_x, goal_y) {
            if score < lowest_score {
                lowest_score = score;
            }
            continue;
        }

        for (dir, opposite, dx, dy) in [
            (Direction::West, Direction::East, -1, 0),
            (Direction::North, Direction::South, 0, -1),
            (Direction::East, Direction::West, 1, 0),
            (Direction::South, Direction::North, 0, 1),
        ] {
            if direction == opposite {
                continue;
            } else if map[(y + dy) as usize][(x + dx) as usize] == Symbol::Wall {
                continue;
            }

            if direction == dir {
                queue.push_back(((x + dx, y + dy, direction), score + 1));
            } else {
                queue.push_back(((x + dx, y + dy, dir), score + 1001));
            }
        }
    }

    lowest_score
}

#[allow(dead_code)]
fn print_paths(tiles: &HashSet<(isize, isize)>, map: &Vec<Vec<Symbol>>) {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if tiles.contains(&(col as isize, row as isize)) {
                print!("O")
            } else {
                print!(
                    "{}",
                    match map[row][col] {
                        Symbol::Empty => ".",
                        Symbol::Wall => "#",
                        Symbol::Goal => "E",
                    }
                )
            }
        }
        println!("");
    }
}

fn part_2(input: &str) -> usize {
    let (mut reindeer_x, mut reindeer_y) = (0, 0);
    let (mut goal_x, mut goal_y) = (0, 0);
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

    let mut lowest_score = usize::MAX;
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    let mut best_tiles: HashSet<(isize, isize)> = HashSet::new();

    queue.push_back(((reindeer_x, reindeer_y, Direction::East), 0, HashSet::new()));
    while let Some((position, score, mut tiles)) = queue.pop_front() {
        if score > *visited.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        visited.insert(position, score);

        let (x, y, direction) = position;

        tiles.insert((position.0, position.1));
        if (x, y) == (goal_x, goal_y) {
            if score < lowest_score {
                best_tiles.clear();
                lowest_score = score;
            }

            if score <= lowest_score {
                best_tiles.extend(&tiles);
            }
            continue;
        }

        for (dir, opposite, dx, dy) in [
            (Direction::West, Direction::East, -1, 0),
            (Direction::North, Direction::South, 0, -1),
            (Direction::East, Direction::West, 1, 0),
            (Direction::South, Direction::North, 0, 1),
        ] {
            if direction == opposite {
                continue;
            } else if map[(y + dy) as usize][(x + dx) as usize] == Symbol::Wall {
                continue;
            }

            if direction == dir {
                queue.push_back(((x + dx, y + dy, direction), score + 1, tiles.clone()));
            } else {
                queue.push_back(((x + dx, y + dy, dir), score + 1001, tiles.clone()));
            }
        }
    }

    print_paths(&best_tiles, &map);

    best_tiles.len()
}

fn main() {
    println!("day 16");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
