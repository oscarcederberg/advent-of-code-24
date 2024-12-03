use regex::Regex;

const TEST_1: &'static str = include_str!("../../input/03/test_1.txt");
const TEST_2: &'static str = include_str!("../../input/03/test_2.txt");
const INPUT: &'static str = include_str!("../../input/03/input.txt");

fn part_1(input: &str) -> u32 {
    let expression = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    expression
        .captures_iter(input)
        .map(|c| c.extract::<2>())
        .fold(0, |count, (_, values)| {
            count + (values[0].parse::<u32>().unwrap() * values[1].parse::<u32>().unwrap())
        })
}

fn part_2(input: &str) -> u32 {
    let expression = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    expression
        .captures_iter(input)
        .map(|c| c.iter().collect::<Vec<Option<_>>>())
        .fold((0, true), |(count, enabled), values| match values[..] {
            [_, _, _, Some(_), _] => (count, true),
            [_, _, _, _, Some(_)] => (count, false),
            [_, Some(x), Some(y), _, _] => {
                if enabled {
                    (
                        count
                            + (x.as_str().parse::<u32>().unwrap()
                                * y.as_str().parse::<u32>().unwrap()),
                        enabled,
                    )
                } else {
                    (count, enabled)
                }
            }
            _ => unreachable!(
                "should not produce output of size other than 5 or with all members None: {:?}",
                values
            ),
        })
        .0
}

fn main() {
    println!("day 3");
    println!("part 1 (test): {}", part_1(TEST_1));
    println!("part 2 (test): {}", part_2(TEST_2));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
