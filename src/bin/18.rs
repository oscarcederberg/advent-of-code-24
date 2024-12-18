use std::collections::{HashMap, VecDeque};

const TEST: &'static str = include_str!("../../input/18/test.txt");
const INPUT: &'static str = include_str!("../../input/18/input.txt");

fn solve(corrupted: &Vec<Vec<bool>>, grid_size: usize) -> Option<usize> {
    let mut found = false;
    let mut lowest_score = usize::MAX;
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(((0_isize, 0_isize), 0));
    while let Some((position, score)) = queue.pop_front() {
        if score >= *visited.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        visited.insert(position, score);

        let (x, y) = position;
        if (x as usize, y as usize) == (grid_size, grid_size) {
            found = true;
            if score < lowest_score {
                lowest_score = score;
            }
            continue;
        }

        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if x + dx < 0
                || x + dx > grid_size as isize
                || y + dy < 0
                || y + dy > grid_size as isize
            {
                continue;
            } else if corrupted[(y + dy) as usize][(x + dx) as usize] {
                continue;
            }

            queue.push_back(((x + dx, y + dy), score + 1));
        }
    }

    if found {
        Some(lowest_score)
    } else {
        None
    }
}

fn part_1(input: &str, grid_size: usize, iterations: usize) -> usize {
    let mut corrupted: Vec<Vec<bool>> = (0..=grid_size)
        .map(|_| (0..=grid_size).map(|_| false).collect())
        .collect();

    input.lines().take(iterations).for_each(|line| {
        let (x, y) = line.split_once(",").unwrap();
        let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
        corrupted[y][x] = true;
    });

    solve(&corrupted, grid_size).unwrap()
}

fn part_2(input: &str, grid_size: usize) -> (usize, usize) {
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut left = 0;
    let mut right = bytes.len() - 1;

    loop {
        let mid = (left + right) / 2;
        let mut corrupted: Vec<Vec<bool>> = (0..=grid_size)
            .map(|_| (0..=grid_size).map(|_| false).collect())
            .collect();

        bytes.iter().take(mid).for_each(|(x, y)| {
            corrupted[*y][*x] = true;
        });

        match solve(&corrupted, grid_size) {
            Some(_) => left = mid + 1,
            None => right = mid - 1,
        }

        if left > right {
            return bytes[mid];
        }
    }
}

fn main() {
    println!("day 18");
    println!("part 1 (test): {}", part_1(TEST, 6, 12));
    println!("part 2 (test): {:?}", part_2(TEST, 6));
    println!("part 1: {}", part_1(INPUT, 70, 1024));
    println!("part 2: {:?}", part_2(INPUT, 70));
}
