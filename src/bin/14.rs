use regex::Regex;

const TEST: &'static str = include_str!("../../input/14/test.txt");
const INPUT: &'static str = include_str!("../../input/14/input.txt");

fn part_1(input: &str, width: i64, height: i64) -> usize {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let (_, values): (&str, [&str; 4]) = robot_regex.captures(line).unwrap().extract();
            values.map(|value| value.parse::<i64>().unwrap())
        })
        .map(|[x, y, dx, dy]| {
            (
                (x + 100 * dx).rem_euclid(width),
                (y + 100 * dy).rem_euclid(height),
            )
        })
        .fold([0; 4], |mut quadrants: [usize; 4], (x, y)| {
            let (half_width, half_height) = (width / 2, height / 2);
            if x > half_width && y < half_height {
                quadrants[0] += 1;
            } else if x < half_width && y < half_height {
                quadrants[1] += 1;
            } else if x < half_width && y > half_height {
                quadrants[2] += 1;
            } else if x > half_width && y > half_height {
                quadrants[3] += 1;
            }
            quadrants
        })
        .iter()
        .fold(1, |sigma, count| sigma * count)
}

fn part_2(input: &str, width: i64, height: i64) -> usize {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, values): (&str, [&str; 4]) = robot_regex.captures(line).unwrap().extract();
            values.map(|value| value.parse::<i64>().unwrap())
        })
        .collect();

    let mut second = 0;
    loop {
        let mut overlap = false;
        let mut map: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect();

        robots = robots
            .into_iter()
            .map(|[x, y, dx, dy]| {
                if map[y as usize][x as usize] {
                    overlap = true;
                }
                map[y as usize][x as usize] = true;

                [
                    (x + dx).rem_euclid(width),
                    (y + dy).rem_euclid(height),
                    dx,
                    dy,
                ]
            })
            .collect();

        if !overlap {
            map.iter().for_each(|row| {
                row.iter()
                    .for_each(|slot| print!("{}", if *slot { "#" } else { "." }));
                println!("");
            });
            return second;
        }

        second += 1;
    }
}

fn main() {
    println!("day 14");
    println!("part 1 (test): {}", part_1(TEST, 11, 7));
    println!("part 2 (test): {}", part_2(TEST, 11, 7));
    println!("part 1: {}", part_1(INPUT, 101, 103));
    println!("part 2: {}", part_2(INPUT, 101, 103));
}
