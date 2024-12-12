const TEST: &'static str = include_str!("../../input/12/test.txt");
const INPUT: &'static str = include_str!("../../input/12/input.txt");

fn part_1(input: &str) -> usize {
    let plots: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let (rows, cols) = (plots.len() as i32, plots[0].len() as i32);
    let mut regions: Vec<(char, usize, usize)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = (0..rows)
        .map(|_| (0..cols).map(|_| false).collect())
        .collect();

    plots.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, plant)| {
            if visited[y][x] {
                return;
            }

            visited[y][x] = true;

            let mut area = 1;
            let mut perimeter = 0;
            let mut buffer: Vec<(usize, usize)> = vec![(x, y)];
            while let Some((x, y)) = buffer.pop() {
                [(-1, 0), (0, -1), (1, 0), (0, 1)]
                    .into_iter()
                    .for_each(|(dx, dy)| {
                        let (check_x, check_y) = (x as i32 + dx, y as i32 + dy);
                        if check_x >= 0 && check_x < cols && check_y >= 0 && check_y < rows {
                            let (plot_x, plot_y) = (check_x as usize, check_y as usize);
                            if !visited[plot_y][plot_x] && plots[plot_y][plot_x] == *plant {
                                visited[plot_y][plot_x] = true;
                                area += 1;
                                buffer.push((plot_x, plot_y));
                            } else if plots[plot_y][plot_x] != *plant {
                                perimeter += 1;
                            }
                        } else {
                            perimeter += 1
                        }
                    });
            }
            regions.push((*plant, area, perimeter));
        })
    });

    regions
        .iter()
        .fold(0, |total, (_, area, perimeter)| total + (area * perimeter))
}

fn part_2(input: &str) -> usize {
    let plots: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let (rows, cols) = (plots.len() as i32, plots[0].len() as i32);
    let mut regions: Vec<(char, usize, usize)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = (0..rows)
        .map(|_| (0..cols).map(|_| false).collect())
        .collect();

    plots.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, plant)| {
            if visited[y][x] {
                return;
            }

            visited[y][x] = true;

            let mut area = 1;
            let mut sides = 0;
            let mut buffer: Vec<(usize, usize)> = vec![(x, y)];
            while let Some((x, y)) = buffer.pop() {
                [(-1, 0), (0, -1), (1, 0), (0, 1)]
                    .into_iter()
                    .for_each(|(dx, dy)| {
                        let (check_x, check_y) = (x as i32 + dx, y as i32 + dy);
                        if check_x >= 0 && check_x < cols && check_y >= 0 && check_y < rows {
                            let (plot_x, plot_y) = (check_x as usize, check_y as usize);
                            if !visited[plot_y][plot_x] && plots[plot_y][plot_x] == *plant {
                                visited[plot_y][plot_x] = true;
                                area += 1;
                                buffer.push((plot_x, plot_y));
                            }
                        }
                    });
                [(-1, -1), (1, -1), (1, 1), (-1, -1)]
                    .into_iter()
                    .for_each(|(dx, dy)| {
                        let (check_x, check_y) = (x as i32 + dx, y as i32 + dy);
                        if check_x >= 0 && check_x < cols && check_y >= 0 && check_y < rows {
                            let (plot_x, plot_y) = (check_x as usize, check_y as usize);
                            if plots[plot_y][plot_x] != *plant {
                                sides += 1;
                            }
                        }
                    });
            }
            regions.push((*plant, area, sides));
        })
    });

    regions
        .iter()
        .for_each(|(plant, area, sides)| println!("{}: {} * {}", plant, area, sides));

    regions
        .iter()
        .fold(0, |total, (_, area, sides)| total + (area * sides))
}

fn main() {
    println!("day 12");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
