const TEST: &'static str = include_str!("../../input/07/test.txt");
const INPUT: &'static str = include_str!("../../input/07/input.txt");

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (test, equation) = line.split_once(": ").unwrap();
            (
                test.parse().unwrap(),
                equation
                    .split(" ")
                    .map(|value| value.parse().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .filter(|(test, equation)| {
            equation
                .iter()
                .fold(Vec::with_capacity(1), |mut buffer, value| {
                    if buffer.len() == 0 {
                        buffer.push(*value);
                        buffer
                    } else {
                        let mut new_buffer = Vec::with_capacity(buffer.len() * 2);
                        for previous in buffer {
                            new_buffer.push(previous + value);
                            new_buffer.push(previous * value);
                        }

                        new_buffer
                    }
                })
                .iter()
                .any(|value| value == test)
        })
        .fold(0, |sum, (test, _)| sum + test)
}

fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (test, equation) = line.split_once(": ").unwrap();
            (
                test.parse().unwrap(),
                equation
                    .split(" ")
                    .map(|value| value.parse().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .filter(|(test, equation)| {
            equation
                .iter()
                .fold(Vec::with_capacity(1), |mut buffer, value| {
                    if buffer.len() == 0 {
                        buffer.push(*value);
                        buffer
                    } else {
                        let mut new_buffer = Vec::with_capacity(buffer.len() * 2);
                        for previous in buffer {
                            new_buffer.push(previous + value);
                            new_buffer.push(previous * value);
                            new_buffer.push(format!("{}{}", previous, value).parse().unwrap())
                        }

                        new_buffer
                    }
                })
                .iter()
                .any(|value| value == test)
        })
        .fold(0, |sum, (test, _)| sum + test)
}

fn main() {
    println!("day 7");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
