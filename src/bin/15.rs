const TEST: &'static str = include_str!("../../input/15/test.txt");
const INPUT: &'static str = include_str!("../../input/15/input.txt");

enum Symbol {
    Empty,
    Box,
    Wall,
}

fn part_1(input: &str) -> usize {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let (mut robot_x, mut robot_y) = (0, 0);
    let mut map: Vec<Vec<Symbol>> = map
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| match symbol {
                    '.' => Symbol::Empty,
                    '@' => {
                        (robot_x, robot_y) = (col as isize, row as isize);
                        Symbol::Empty
                    }
                    'O' => Symbol::Box,
                    '#' => Symbol::Wall,
                    _ => unreachable!("found non-handled symbol {}", symbol),
                })
                .collect()
        })
        .collect();

    instructions
        .lines()
        .flat_map(|line| line.trim().chars())
        .for_each(|instruction| {
            let (dx, dy) = match instruction {
                '<' => (-1, 0),
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                _ => unreachable!("found non-handled instruction {}", instruction),
            };
            let (mut check_x, mut check_y) = (robot_x, robot_y);
            let mut boxes = 0;
            loop {
                (check_x, check_y) = (check_x + dx, check_y + dy);
                match map[check_y as usize][check_x as usize] {
                    Symbol::Wall => break,
                    Symbol::Box => boxes += 1,
                    Symbol::Empty => {
                        if boxes == 0 {
                            (robot_x, robot_y) = (robot_x + dx, robot_y + dy);
                            break;
                        } else {
                            (robot_x, robot_y) = (robot_x + dx, robot_y + dy);
                            map[robot_y as usize][robot_x as usize] = Symbol::Empty;
                            map[check_y as usize][check_x as usize] = Symbol::Box;
                            break;
                        }
                    }
                }
            }
        });

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            print!(
                "{}",
                match map[row][col] {
                    Symbol::Empty => ".",
                    Symbol::Box => "O",
                    Symbol::Wall => "#",
                }
            )
        }
        println!("");
    }

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().fold(0, |sum, (x, symbol)| {
                if matches!(symbol, Symbol::Box) {
                    sum + 100 * y + x
                } else {
                    sum
                }
            })
        })
        .sum()
}

#[derive(Clone)]
enum WideSymbol {
    Empty,
    LeftBox,
    RightBox,
    Wall,
}

fn part_2(input: &str) -> usize {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let (mut robot_x, mut robot_y) = (0, 0);
    let mut map: Vec<Vec<WideSymbol>> = map
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(col, symbol)| match symbol {
                    '.' => [WideSymbol::Empty, WideSymbol::Empty],
                    '@' => {
                        (robot_x, robot_y) = (2 * col as isize, row as isize);
                        [WideSymbol::Empty, WideSymbol::Empty]
                    }
                    'O' => [WideSymbol::LeftBox, WideSymbol::RightBox],
                    '#' => [WideSymbol::Wall, WideSymbol::Wall],
                    _ => unreachable!("found non-handled symbol {}", symbol),
                })
                .collect()
        })
        .collect();

    instructions
        .lines()
        .flat_map(|line| line.trim().chars())
        .for_each(|instruction| {
            let (dx, dy) = match instruction {
                '<' => (-1, 0),
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                _ => unreachable!("found non-handled instruction {}", instruction),
            };

            let (mut check_x, mut check_y) = (robot_x, robot_y);
            if dx != 0 {
                let mut boxes = 0;
                loop {
                    (check_x, check_y) = (check_x + dx, check_y);
                    match map[check_y as usize][check_x as usize] {
                        WideSymbol::Wall => break,
                        WideSymbol::LeftBox | WideSymbol::RightBox => boxes += 1,
                        WideSymbol::Empty => {
                            (robot_x, robot_y) = (robot_x + dx, robot_y);
                            if boxes == 0 {
                                break;
                            } else {
                                for iter in 0..=boxes {
                                    map[(check_y) as usize][(check_x - iter * dx) as usize] = map
                                        [(check_y) as usize]
                                        [(check_x - (iter + 1) * dx) as usize]
                                        .clone();
                                }
                                break;
                            }
                        }
                    }
                }
            } else {
                let mut buffer: Vec<(isize, isize)> = Vec::new();
                let mut checked: Vec<(isize, isize)> = Vec::new();
                (check_x, check_y) = (check_x + dx, check_y + dy);

                match map[check_y as usize][check_x as usize] {
                    WideSymbol::Empty => (),
                    WideSymbol::LeftBox => {
                        checked.push((check_x, check_y));
                        buffer.push((check_x, check_y))
                    }
                    WideSymbol::RightBox => {
                        checked.push((check_x - 1, check_y));
                        buffer.push((check_x - 1, check_y))
                    }
                    WideSymbol::Wall => return,
                }

                while let Some((check_x, check_y)) = checked.pop() {
                    match map[(check_y + dy) as usize][check_x as usize] {
                        WideSymbol::Empty => (),
                        WideSymbol::LeftBox => {
                            checked.push((check_x, check_y + dy));
                            buffer.push((check_x, check_y + dy))
                        }
                        WideSymbol::RightBox => {
                            checked.push((check_x - 1, check_y + dy));
                            buffer.push((check_x - 1, check_y + dy))
                        }
                        WideSymbol::Wall => return,
                    }
                    match map[(check_y + dy) as usize][check_x as usize + 1] {
                        WideSymbol::Empty => (),
                        WideSymbol::LeftBox => {
                            checked.push((check_x + 1, check_y + dy));
                            buffer.push((check_x + 1, check_y + dy))
                        }
                        WideSymbol::RightBox => (),
                        WideSymbol::Wall => return,
                    }
                }

                if dy < 0 {
                    buffer.sort_unstable_by_key(|(_, y)| *y);
                } else {
                    buffer.sort_unstable_by_key(|(_, y)| -(*y));
                }

                buffer.iter().for_each(|(x, y)| {
                    let (x, y) = (*x, *y);
                    map[y as usize][x as usize] = WideSymbol::Empty;
                    map[y as usize][(x + 1) as usize] = WideSymbol::Empty;
                    map[(y + dy) as usize][x as usize] = WideSymbol::LeftBox;
                    map[(y + dy) as usize][(x + 1) as usize] = WideSymbol::RightBox;
                });

                (robot_x, robot_y) = (robot_x, robot_y + dy);
            }
        });

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if (col, row) == (robot_x as usize, robot_y as usize) {
                print!("@");
            } else {
                print!(
                    "{}",
                    match map[row][col] {
                        WideSymbol::Empty => ".",
                        WideSymbol::LeftBox => "[",
                        WideSymbol::RightBox => "]",
                        WideSymbol::Wall => "#",
                    }
                )
            }
        }
        println!("");
    }

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().fold(0, |sum, (x, symbol)| {
                if matches!(symbol, WideSymbol::LeftBox) {
                    sum + 100 * y + x
                } else {
                    sum
                }
            })
        })
        .sum()
}

fn main() {
    println!("day 15");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
