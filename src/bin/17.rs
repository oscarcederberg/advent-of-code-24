use crate::Instruction::*;

const TEST_1: &'static str = include_str!("../../input/17/test_1.txt");
const TEST_2: &'static str = include_str!("../../input/17/test_2.txt");
const INPUT: &'static str = include_str!("../../input/17/input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    ADV, // performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
    BXL, // calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
    BST, // calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    JNZ, // does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
    BXC, // calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
    OUT, // calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    BDV, // works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
    CDV, // works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
}

fn opcode_to_instruction(opcode: &usize) -> Instruction {
    match opcode {
        0 => ADV,
        1 => BXL,
        2 => BST,
        3 => JNZ,
        4 => BXC,
        5 => OUT,
        6 => BDV,
        7 => CDV,
        _ => unreachable!("got non-handled symbol: {}", *opcode),
    }
}

fn combo_operand_to_value(operand: &usize, registers: &Vec<usize>) -> usize {
    match operand {
        0..=3 => *operand,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!("got non-handled symbol: {}", *operand),
    }
}

fn read(instructions: &Vec<usize>, instruction_pointer: &mut usize) -> usize {
    *instruction_pointer += 1;
    instructions[*instruction_pointer - 1]
}

fn part_1(input: &str) -> usize {
    let mut instruction_pointer = 0;
    let mut out: Vec<usize> = Vec::new();

    let (registers, instructions) = input.split_once("\n\n").unwrap();
    let mut registers: Vec<usize> = registers
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect();
    let instructions: Vec<usize> = instructions
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|symbol| symbol.parse().unwrap())
        .collect();

    while instruction_pointer < instructions.len() {
        let instruction = opcode_to_instruction(&read(&instructions, &mut instruction_pointer));

        if instruction_pointer == instructions.len() {
            break;
        }

        // println!("A: {}\nB: {}\nC: {}", registers[0], registers[1], registers[2]);
        // println!("instr: {:?}\ninstr_ptr: {}", instruction, instruction_pointer);
        // println!("next: {}\nout: {:?}\n", instructions[instruction_pointer], out);

        match instruction {
            ADV => {
                let numerator = registers[0];
                let denominator = 1
                    << combo_operand_to_value(
                        &read(&instructions, &mut instruction_pointer),
                        &registers,
                    );
                registers[0] = numerator / denominator;
            }
            BXL => {
                let literal = read(&instructions, &mut instruction_pointer);
                registers[1] = registers[1] ^ literal;
            }
            BST => {
                let value = combo_operand_to_value(
                    &read(&instructions, &mut instruction_pointer),
                    &registers,
                )
                .rem_euclid(8);
                registers[1] = value;
            }
            JNZ => {
                if registers[0] != 0 {
                    instruction_pointer = read(&instructions, &mut instruction_pointer);
                } else {
                    instruction_pointer += 1;
                }
            }
            BXC => {
                let value = registers[1] ^ registers[2];
                registers[1] = value;
                instruction_pointer += 1;
            }
            OUT => {
                let value = combo_operand_to_value(
                    &read(&instructions, &mut instruction_pointer),
                    &registers,
                )
                .rem_euclid(8);
                out.push(value);
            }
            BDV => {
                let numerator = registers[0];
                let denominator = 1
                    << combo_operand_to_value(
                        &read(&instructions, &mut instruction_pointer),
                        &registers,
                    );
                registers[1] = numerator / denominator;
            }
            CDV => {
                let numerator = registers[0];
                let denominator = 1
                    << combo_operand_to_value(
                        &read(&instructions, &mut instruction_pointer),
                        &registers,
                    );
                registers[2] = numerator / denominator;
            }
        }
    }

    println!(
        "{}",
        out.iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    out.iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let (registers, instructions) = input.split_once("\n\n").unwrap();
    let registers: Vec<usize> = registers
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect();
    let instructions: Vec<usize> = instructions
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|symbol| symbol.parse().unwrap())
        .collect();

    let mut a = registers[0];
    loop {
        let mut registers = registers.clone();
        let mut instruction_pointer = 0;
        let mut out: Vec<usize> = Vec::new();

        registers[0] = a;

        while instruction_pointer < instructions.len() {
            let instruction = opcode_to_instruction(&read(&instructions, &mut instruction_pointer));

            if instruction_pointer == instructions.len() {
                break;
            }

            match instruction {
                ADV => {
                    let numerator = registers[0];
                    let denominator = 1
                        << combo_operand_to_value(
                            &read(&instructions, &mut instruction_pointer),
                            &registers,
                        );
                    registers[0] = numerator / denominator;
                }
                BXL => {
                    let literal = read(&instructions, &mut instruction_pointer);
                    registers[1] = registers[1] ^ literal;
                }
                BST => {
                    let value = combo_operand_to_value(
                        &read(&instructions, &mut instruction_pointer),
                        &registers,
                    )
                    .rem_euclid(8);
                    registers[1] = value;
                }
                JNZ => {
                    if registers[0] != 0 {
                        instruction_pointer = read(&instructions, &mut instruction_pointer);
                    } else {
                        instruction_pointer += 1;
                    }
                }
                BXC => {
                    let value = registers[1] ^ registers[2];
                    registers[1] = value;
                    instruction_pointer += 1;
                }
                OUT => {
                    let value = combo_operand_to_value(
                        &read(&instructions, &mut instruction_pointer),
                        &registers,
                    )
                    .rem_euclid(8);
                    out.push(value);
                }
                BDV => {
                    let numerator = registers[0];
                    let denominator = 1
                        << combo_operand_to_value(
                            &read(&instructions, &mut instruction_pointer),
                            &registers,
                        );
                    registers[1] = numerator / denominator;
                }
                CDV => {
                    let numerator = registers[0];
                    let denominator = 1
                        << combo_operand_to_value(
                            &read(&instructions, &mut instruction_pointer),
                            &registers,
                        );
                    registers[2] = numerator / denominator;
                }
            }
        }

        if out == instructions {
            break;
        }

        if instructions.len() > out.len() {
            a *= 2;
        } else if instructions.len() == out.len() {
            for index in (0..instructions.len()).rev() {
                if instructions[index] != out[index] {
                    a += 8_usize.pow(index as u32);
                    break;
                }
            }
        } else {
            a /= 2;
        }
    }

    a
}

fn main() {
    println!("day 17");
    println!("part 1 (test): {}", part_1(TEST_1));
    println!("part 2 (test): {}", part_2(TEST_2));
    println!("part 1: {}", part_1(INPUT));
    println!("part 2: {}", part_2(INPUT));
}
