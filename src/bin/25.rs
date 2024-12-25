const TEST: &'static str = include_str!("../../input/25/test.txt");
const INPUT: &'static str = include_str!("../../input/25/input.txt");

const COLUMNS: usize = 5;
const ROWS: usize = 7;

fn schematic_to_lock(schematic: &Vec<Vec<char>>) -> [usize; COLUMNS] {
    let mut lock = [0; COLUMNS];
    for column in 0..COLUMNS {
        for row in 1..ROWS {
            if schematic[row][column] == '.' {
                lock[column] = row - 1;
                break;
            }
        }
    }

    lock
}

fn schematic_to_key(schematic: &Vec<Vec<char>>) -> [usize; COLUMNS] {
    let mut key = [0; COLUMNS];
    for column in 0..COLUMNS {
        for row in 1..ROWS {
            if schematic[ROWS - 1 - row][column] == '.' {
                key[column] = row - 1;
                break;
            }
        }
    }

    key
}

fn overlaps(key: &[usize; COLUMNS], lock: &[usize; COLUMNS]) -> bool {
    for column in 0..COLUMNS {
        if key[column] + lock[column] >= ROWS - 1 {
            return true;
        }
    }

    false
}

fn part_1(input: &str) -> usize {
    let mut locks: Vec<[usize; COLUMNS]> = Vec::new();
    let mut keys: Vec<[usize; COLUMNS]> = Vec::new();
    input.split("\n\n").for_each(|block| {
        let schematic: Vec<Vec<_>> = block.lines().map(|line| line.chars().collect()).collect();

        if schematic[0][0] == '#' {
            let lock = schematic_to_lock(&schematic);
            locks.push(lock);
        } else {
            let key = schematic_to_key(&schematic);
            keys.push(key);
        }
    });

    let mut fits = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if !overlaps(key, lock) {
                fits += 1;
            }
        }
    }

    fits
}

fn main() {
    println!("day 25");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 1: {}", part_1(INPUT));
}
