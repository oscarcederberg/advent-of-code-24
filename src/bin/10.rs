use std::collections::HashSet;

const TEST_1: &'static str = include_str!("../../input/10/test_1.txt");
const TEST_2: &'static str = include_str!("../../input/10/test_2.txt");
const INPUT: &'static str = include_str!("../../input/10/input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct BufferItem {
    trailhead_x: usize,
    trailhead_y: usize,
    x: usize,
    y: usize,
}

fn part_1(input: &str) -> usize {
    let mut buffer: Vec<BufferItem> = Vec::new();
    let mut reachable: HashSet<BufferItem> = HashSet::new();
    let map: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| match symbol.to_digit(10).unwrap() {
                    0 => {
                        buffer.push(BufferItem {
                            trailhead_x: col,
                            trailhead_y: row,
                            x: col,
                            y: row,
                        });
                        0
                    }
                    height => height,
                })
                .collect()
        })
        .collect();
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);

    while buffer.len() > 0 {
        let item = buffer.pop().unwrap();
        let height = map[item.y][item.x];

        if height == 9 {
            reachable.insert(item);
        } else {
            [(-1,0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .for_each(|(dx, dy)| {
                    let (check_x, check_y) = (item.x as i32 + dx, item.y as i32 + dy);
                    if check_x >= 0 && check_x < cols && check_y >= 0 && check_y < rows {
                        let (pos_x, pos_y) = (check_x as usize, check_y as usize);
                        if map[pos_y][pos_x] == height + 1 {
                            buffer.push(BufferItem {
                                trailhead_x: item.trailhead_x,
                                trailhead_y: item.trailhead_y,
                                x: pos_x,
                                y: pos_y,
                            })
                        }
                    }
                });
        }
    }

    reachable.len()
}

fn part_2(input: &str) -> usize {
    let mut buffer: Vec<BufferItem> = Vec::new();
    let mut reachable = 0;
    let map: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| match symbol.to_digit(10).unwrap() {
                    0 => {
                        buffer.push(BufferItem {
                            trailhead_x: col,
                            trailhead_y: row,
                            x: col,
                            y: row,
                        });
                        0
                    }
                    height => height,
                })
                .collect()
        })
        .collect();
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);

    while buffer.len() > 0 {
        let item = buffer.pop().unwrap();
        let height = map[item.y][item.x];

        if height == 9 {
            reachable += 1;
        } else {
            [(-1,0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .for_each(|(dx, dy)| {
                    let (check_x, check_y) = (item.x as i32 + dx, item.y as i32 + dy);
                    if check_x >= 0 && check_x < cols && check_y >= 0 && check_y < rows {
                        let (pos_x, pos_y) = (check_x as usize, check_y as usize);
                        if map[pos_y][pos_x] == height + 1 {
                            buffer.push(BufferItem {
                                trailhead_x: item.trailhead_x,
                                trailhead_y: item.trailhead_y,
                                x: pos_x,
                                y: pos_y,
                            })
                        }
                    }
                });
        }
    }

    reachable
}

fn main() {
    println!("day 10");
    println!("part 1 (test, 1): {}", part_1(TEST_1));
    println!("part 2 (test, 1): {}", part_2(TEST_1));
    println!("part 1 (test, 2): {}", part_1(TEST_2));
    println!("part 2 (test, 2): {}", part_2(TEST_2));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
