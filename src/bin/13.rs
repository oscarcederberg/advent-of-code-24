#![feature(iter_array_chunks)]

use regex::Regex;
use z3::{
    ast::{Ast, Int},
    Config, Context, Optimize,
};

const TEST: &'static str = include_str!("../../input/13/test.txt");
const INPUT: &'static str = include_str!("../../input/13/input.txt");

fn part_1(input: &str) -> u64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let button_regex = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let pos_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    input
        .lines()
        .array_chunks::<4>()
        .map(|[a, b, pos, _]| {
            let (_, [a_x, a_y]) = button_regex.captures(a).unwrap().extract();
            let (_, [b_x, b_y]) = button_regex.captures(b).unwrap().extract();
            let (_, [p_x, p_y]) = pos_regex.captures(pos).unwrap().extract();
            [a_x, a_y, b_x, b_y, p_x, p_y].map(|value| value.parse().unwrap())
        })
        .enumerate()
        .map(|(i, [a_x, a_y, b_x, b_y, p_x, p_y])| {
            let opt = Optimize::new(&ctx);
            let a = Int::new_const(&ctx, "a");
            let b = Int::new_const(&ctx, "b");
            let assumptions = [
                Int::add(
                    &ctx,
                    &[
                        &Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, a_x)]),
                        &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, b_x)]),
                    ],
                )
                ._eq(&Int::from_i64(&ctx, p_x)),
                Int::add(
                    &ctx,
                    &[
                        &Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, a_y)]),
                        &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, b_y)]),
                    ],
                )
                ._eq(&Int::from_i64(&ctx, p_y)),
                a.ge(&Int::from_i64(&ctx, 0)),
                b.ge(&Int::from_i64(&ctx, 0)),
                a.le(&Int::from_i64(&ctx, 100)),
                b.le(&Int::from_i64(&ctx, 100)),
            ];
            let objective = Int::add(&ctx, &[&Int::mul(&ctx, &[&Int::from_i64(&ctx, 3), &a]), &b]);
            opt.minimize(&objective);

            if opt.check(&assumptions) == z3::SatResult::Sat {
                let model = opt.get_model().unwrap();
                let a_value = model.eval(&a, true).unwrap().as_u64().unwrap();
                let b_value = model.eval(&b, true).unwrap().as_u64().unwrap();
                3 * a_value + b_value
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let button_regex = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let pos_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    input
        .lines()
        .array_chunks::<4>()
        .map(|[a, b, pos, _]| {
            let (_, [a_x, a_y]) = button_regex.captures(a).unwrap().extract();
            let (_, [b_x, b_y]) = button_regex.captures(b).unwrap().extract();
            let (_, [p_x, p_y]) = pos_regex.captures(pos).unwrap().extract();
            [a_x, a_y, b_x, b_y, p_x, p_y].map(|value| value.parse().unwrap())
        })
        .enumerate()
        .map(|(i, [a_x, a_y, b_x, b_y, p_x, p_y])| {
            let p_x = p_x + 10000000000000;
            let p_y = p_y + 10000000000000;
            let opt = Optimize::new(&ctx);
            let a = Int::new_const(&ctx, "a");
            let b = Int::new_const(&ctx, "b");
            let assumptions = [
                Int::add(
                    &ctx,
                    &[
                        &Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, a_x)]),
                        &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, b_x)]),
                    ],
                )
                ._eq(&Int::from_i64(&ctx, p_x)),
                Int::add(
                    &ctx,
                    &[
                        &Int::mul(&ctx, &[&a, &Int::from_i64(&ctx, a_y)]),
                        &Int::mul(&ctx, &[&b, &Int::from_i64(&ctx, b_y)]),
                    ],
                )
                ._eq(&Int::from_i64(&ctx, p_y)),
                a.ge(&Int::from_i64(&ctx, 0)),
                b.ge(&Int::from_i64(&ctx, 0)),
            ];
            let objective = Int::add(&ctx, &[&Int::mul(&ctx, &[&Int::from_i64(&ctx, 3), &a]), &b]);
            opt.minimize(&objective);

            if opt.check(&assumptions) == z3::SatResult::Sat {
                let model = opt.get_model().unwrap();
                let a_value = model.eval(&a, true).unwrap().as_u64().unwrap();
                let b_value = model.eval(&b, true).unwrap().as_u64().unwrap();
                3 * a_value + b_value
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("day 13");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
