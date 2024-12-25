use std::collections::{HashMap, VecDeque};

const TEST: &'static str = include_str!("../../input/20/test.txt");
const INPUT: &'static str = include_str!("../../input/20/input.txt");

fn part_1(input: &str, save_at_least: usize) -> usize {
    let (mut start_x, mut start_y): (isize, isize) = (0, 0);
    let (mut goal_x, mut goal_y): (isize, isize) = (0, 0);

    let map: Vec<Vec<bool>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map({
                    |(col, symbol)| match symbol {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            (start_x, start_y) = (col as isize, row as isize);
                            true
                        }
                        'E' => {
                            (goal_x, goal_y) = (col as isize, row as isize);
                            true
                        }
                        _ => unreachable!("found unhandled symbol '{}'", symbol),
                    }
                })
                .collect()
        })
        .collect();

    let mut lowest_picoseconds = usize::MAX;
    let mut scores_per_tile = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(((start_x, start_y), 0));
    while let Some((position, score)) = queue.pop_front() {
        if score >= *scores_per_tile.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        scores_per_tile.insert(position, score);

        if position == (goal_x, goal_y) {
            if score < lowest_picoseconds {
                lowest_picoseconds = score;
            }
            continue;
        }

        let (x, y) = position;
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if !map[(y + dy) as usize][(x + dx) as usize] {
                continue;
            }

            queue.push_back(((x + dx, y + dy), score + 1));
        }
    }

    scores_per_tile
        .iter()
        .map(|((x, y), score)| {
            [
                (-2, 0),
                (-1, -1),
                (0, -2),
                (1, -1),
                (2, 0),
                (1, 1),
                (0, 2),
                (-1, 1),
            ]
            .iter()
            .fold(0, |count, (dx, dy)| {
                if x + dx < 0
                    || x + dx >= map[0].len() as isize
                    || y + dy < 0
                    || y + dy >= map.len() as isize
                {
                    return count;
                } else if let Some(score_for_tile) = scores_per_tile.get(&(x + dx, y + dy)) {
                    if *score + 2 + save_at_least <= *score_for_tile {
                        return count + 1;
                    }
                }
                count
            })
        })
        .sum()
}

fn part_2(input: &str, save_at_least: usize) -> usize {
    let (mut start_x, mut start_y): (isize, isize) = (0, 0);
    let (mut goal_x, mut goal_y): (isize, isize) = (0, 0);

    let map: Vec<Vec<bool>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map({
                    |(col, symbol)| match symbol {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            (start_x, start_y) = (col as isize, row as isize);
                            true
                        }
                        'E' => {
                            (goal_x, goal_y) = (col as isize, row as isize);
                            true
                        }
                        _ => unreachable!("found unhandled symbol '{}'", symbol),
                    }
                })
                .collect()
        })
        .collect();

    let mut lowest_picoseconds = usize::MAX;
    let mut scores_per_tile = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(((start_x, start_y), 0));
    while let Some((position, score)) = queue.pop_front() {
        if score >= *scores_per_tile.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        scores_per_tile.insert(position, score);

        if position == (goal_x, goal_y) {
            if score < lowest_picoseconds {
                lowest_picoseconds = score;
            }
            continue;
        }

        let (x, y) = position;
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if !map[(y + dy) as usize][(x + dx) as usize] {
                continue;
            }

            queue.push_back(((x + dx, y + dy), score + 1));
        }
    }

    scores_per_tile
        .iter()
        .map(|((x, y), score)| {
            scores_per_tile
                .iter()
                .filter(|((t_x, t_y), score_for_tile)| {
                    let distance = ((x - t_x).abs() + (y - t_y).abs()) as usize;
                    distance >= 2
                        && distance <= 20
                        && *score + distance + save_at_least <= **score_for_tile
                })
                .count()
        })
        .sum()
}

fn main() {
    println!("day 20");
    println!("part 1 (test): {}", part_1(TEST, 1));
    println!("part 2 (test): {}", part_2(TEST, 50));
    println!("part 1: {}", part_1(INPUT, 100));
    println!("part 2: {}", part_2(INPUT, 100));
}
