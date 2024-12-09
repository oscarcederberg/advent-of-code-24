use crate::Block::*;

use std::collections::VecDeque;

const TEST: &'static str = include_str!("../../input/09/test.txt");
const INPUT: &'static str = include_str!("../../input/09/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    File(usize),
}

fn part_1(input: &str) -> usize {
    let mut memory = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(index, symbol)| {
            let size = symbol.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                vec![File(index / 2); size]
            } else {
                vec![Free; size]
            }
        })
        .collect::<VecDeque<_>>();
    let mut packed: Vec<usize> = Vec::new();

    while memory.len() > 0 {
        match memory.pop_front().unwrap() {
            File(id) => packed.push(id),
            Free => loop {
                match memory.pop_back() {
                    Some(File(id)) => {
                        packed.push(id);
                        break;
                    }
                    Some(Free) => continue,
                    None => break,
                }
            },
        }
    }

    packed
        .iter()
        .enumerate()
        .fold(0, |checksum, (index, id)| checksum + (index * id))
}

fn part_2(input: &str) -> usize {
    let mut memory = input
        .trim()
        .chars()
        .enumerate()
        .map(|(index, symbol)| {
            let size = symbol.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                (File(index / 2), size)
            } else {
                (Free, size)
            }
        })
        .collect::<VecDeque<_>>();

    let max_id = if let Some(&(File(id), _)) = memory.back() {
        id
    } else {
        unreachable!("last block was not a file")
    };

    let mut max_index = memory.len() - 1;
    for current_id in (0..=max_id).rev() {
        let mut current_index: usize = 0;
        let mut current_size: usize = 0;
        for index in (0..=max_index).rev() {
            match memory[index] {
                (File(id), size) if id == current_id => {
                    current_index = index;
                    current_size = size;
                    max_index = index;
                    break;
                }
                _ => (),
            }
        }

        assert!(current_size != 0);

        for index in 0..max_index {
            match memory[index] {
                (Free, size) if size == current_size => {
                    memory[current_index] = (Free, size);
                    memory[index] = (File(current_id), size);
                    break;
                }
                (Free, size) if size >= current_size => {
                    memory[current_index] = (Free, current_size);
                    memory[index] = (File(current_id), current_size);
                    memory.insert(index + 1, (Free, size - current_size));
                    break;
                }
                _ => (),
            }
        }
    }

    memory
        .iter()
        .flat_map(|(block, size)| vec![*block; *size])
        .enumerate()
        .fold(0, |checksum, (index, block)| match block {
            Free => checksum,
            File(id) => checksum + (id * index),
        })
}

fn main() {
    println!("day 9");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
