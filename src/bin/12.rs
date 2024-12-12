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
                [
                    ((-1, 0), (-1, -1), (0, -1)),
                    ((0, -1), (1, -1), (1, 0)),
                    ((1, 0), (1, 1), (0, 1)),
                    ((0, 1), (-1, 1), (-1, 0)),
                ]
                .into_iter()
                .for_each(|((dx_l, dy_l), (dx_c, dy_c), (dx_r, dy_r))| {
                    let (check_x_l, check_y_l) = (x as i32 + dx_l, y as i32 + dy_l);
                    let l_is_outside =
                        !(check_x_l >= 0 && check_x_l < cols && check_y_l >= 0 && check_y_l < rows);
                    let mut l_is_other = false;
                    if !l_is_outside {
                        let (plot_x, plot_y) = (check_x_l as usize, check_y_l as usize);
                        l_is_other = plots[plot_y][plot_x] != *plant;
                    }

                    let (check_x_r, check_y_r) = (x as i32 + dx_r, y as i32 + dy_r);
                    let r_is_outside =
                        !(check_x_r >= 0 && check_x_r < cols && check_y_r >= 0 && check_y_r < rows);
                    let mut r_is_other = false;
                    if !r_is_outside {
                        let (plot_x, plot_y) = (check_x_r as usize, check_y_r as usize);
                        r_is_other = plots[plot_y][plot_x] != *plant;
                    }

                    let (check_x_c, check_y_c) = (x as i32 + dx_c, y as i32 + dy_c);
                    let c_is_outside =
                        !(check_x_c >= 0 && check_x_c < cols && check_y_c >= 0 && check_y_c < rows);
                    let mut c_is_other = false;
                    if !c_is_outside {
                        let (plot_x, plot_y) = (check_x_c as usize, check_y_c as usize);
                        c_is_other = plots[plot_y][plot_x] != *plant;
                    }

                    if (l_is_outside || l_is_other)
                        && (c_is_outside || c_is_other)
                        && (r_is_outside || r_is_other)
                    {
                        sides += 1;
                    } else if !l_is_other && c_is_other && !r_is_other {
                        sides += 1;
                    } else if l_is_other && !c_is_other && r_is_other {
                        sides += 1;
                    }
                });
            }
            regions.push((*plant, area, sides));
        })
    });

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
