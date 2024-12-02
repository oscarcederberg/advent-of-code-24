const TEST: &'static str = include_str!("../../input/02/test.txt");
const INPUT: &'static str = include_str!("../../input/02/input.txt");

struct ProblemDampenedReport {
    pub state: ReportState,
    pub dampened: bool,
}

enum ReportState {
    Init,
    First(u32),
    Second(u32, u32),
    Increasing(u32),
    Decreasing(u32),
    Unsafe
}

impl ProblemDampenedReport {
    fn default() -> Self {
        ProblemDampenedReport {state: ReportState::Init, dampened: false}
    }

    fn set_state(mut self, state: ReportState) -> Self {
        self.state = state;
        self
    }

    fn dampen(mut self) -> Self {
        if self.dampened {
            self.set_state(ReportState::Unsafe)
        } else {
            self.dampened = true;
            self
        }
    }
}

fn is_increasing(x: u32, y: u32) -> bool {
    x < y && y - x <= 3
}

fn is_decreasing(x: u32, y: u32) -> bool {
    y < x && x - y <= 3
}

fn part_1(input: &str) -> u32 {
    use ReportState::*;
    input.lines()
        .map(|x| x.split(" ")
            .map(
                |x| x.parse::<u32>().unwrap()
            ).fold(Init, |state, x| {
                match state {
                    Init => First(x),
                    Second(_, _) => unreachable!("part 1 does not utilize Second(u32, u32)"),
                    First(value) => {
                        if is_increasing(value, x) {
                            Increasing(x)
                        } else if is_decreasing(value, x) {
                            Decreasing(x)
                        } else {
                            Unsafe
                        }
                    },
                    Increasing(value) => {
                        if is_increasing(value, x) {
                            Increasing(x)
                        } else {
                            Unsafe
                        }
                    },
                    Decreasing(value) => {
                        if is_decreasing(value, x) {
                            Decreasing(x)
                        } else {
                            Unsafe
                        }
                    },
                    Unsafe => Unsafe,
                }
            })
        ).fold(0, |count, r| {
            match r {
                Unsafe => count,
                _ => count + 1,
            }
        })
}

fn part_2(input: &str) -> i32 {
    use ReportState::*;
    input.lines()
        .map(|x| x.split(" ")
            .map(
                |x| x.parse::<u32>().unwrap()
            ).fold(ProblemDampenedReport::default(), |report, x| {
                match report.state {
                    Init => report.set_state(First(x)),
                    First(value) => {
                        if is_increasing(value, x) {
                            report.set_state(Second(value, x))
                        } else if is_decreasing(value, x) {
                            report.set_state(Second(value, x))
                        } else {
                            report.set_state(Second(value, x))
                                .dampen()
                        }
                    },
                    Second(first, second) => {
                        if report.dampened {
                            if is_increasing(first, x) {
                                report.set_state(Increasing(x))
                            } else if is_decreasing(first, x) {
                                report.set_state(Decreasing(x))
                            } else if is_increasing(second, x) {
                                report.set_state(Increasing(x))
                            } else if is_decreasing(second, x) {
                                report.set_state(Decreasing(x))
                            } else {
                                report.set_state(Unsafe)
                            }
                        } else {
                            if is_increasing(second, x) {
                                report.set_state(Increasing(x))
                            } else if is_decreasing(second, x) {
                                report.set_state(Decreasing(x))
                            } else if is_increasing(first, x) {
                                report.set_state(Increasing(x))
                            } else if is_decreasing(first, x) {
                                report.set_state(Decreasing(x))
                            } else if is_increasing(first, second) {
                                report.set_state(Increasing(second))
                                    .dampen()
                            } else if is_decreasing(first, second) {
                                report.set_state(Decreasing(second))
                                    .dampen()
                            } else {
                                report.set_state(Unsafe)
                                    .dampen()
                            }
                        }
                    },
                    Increasing(value) => {
                        if is_increasing(value, x) {
                            report.set_state(Increasing(x))
                        } else {
                            report.dampen()
                        }
                    },
                    Decreasing(value) => {
                        if is_decreasing(value, x) {
                            report.set_state(Decreasing(x))
                        } else {
                            report.dampen()
                        }
                    },
                    Unsafe => report,
                }
            })
        ).fold(0, |count, r| {
            match r.state {
                Unsafe => count,
                _ => count + 1,
            }
        })
}

fn main() {
    println!("day 2");
    println!("part 1 (test): {}", part_1(TEST));
    println!("part 2 (test): {}", part_2(TEST));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}