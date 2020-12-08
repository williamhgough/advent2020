use itertools::Itertools;
use std::collections::HashSet;

static INPUT: &str = std::include_str!("../../../input/8.txt");

fn main() {
    let instruction_feed = INPUT.split('\n').collect_vec();
    let parsed_instructions = instruction_feed
        .iter()
        .map(|op| Instruction::from_str(op.trim()))
        .collect_vec();

    let part_one_result = execute_instructions(&parsed_instructions);
    let part_two_result = part_two(&parsed_instructions);
    println!("Part 1: {}", part_one_result.0);
    println!("Part 2: {}", part_two_result);
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Instruction {
    NoOp(String),
    JumpForward(i32),
    JumpBackward(i32),
    Accumulate(i32),
    Unknown,
}

impl Instruction {
    fn from_str(instruction: &'static str) -> Self {
        let calculate_val = |val: &str| -> i32 { val.parse::<i32>().unwrap() };
        match &instruction[0..3] {
            "nop" => Instruction::NoOp(instruction[4..].to_owned()),
            "acc" => Instruction::Accumulate(calculate_val(&instruction[4..])),
            "jmp" => match &instruction.chars().nth(4).unwrap() {
                '-' => Instruction::JumpBackward(calculate_val(&instruction[5..])),
                '+' => Instruction::JumpForward(calculate_val(&instruction[5..])),
                _ => Instruction::Unknown,
            },
            _ => Instruction::Unknown,
        }
    }
}

fn execute_instructions(instructions: &[Instruction]) -> (i32, usize) {
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut idx = 0;
    while !visited.contains(&idx) && idx < instructions.len() {
        match instructions[idx] {
            Instruction::NoOp(_) => {
                visited.insert(idx);
                idx += 1;
            }
            Instruction::JumpForward(x) => {
                visited.insert(idx);
                idx = idx.checked_add(x as usize).unwrap();
            }
            Instruction::JumpBackward(x) => {
                visited.insert(idx);
                idx = idx.checked_sub(x as usize).unwrap();
            }
            Instruction::Accumulate(x) => {
                acc += x;
                visited.insert(idx);
                idx += 1;
            }
            _ => {}
        };
    }
    (acc, idx)
}

fn part_two(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            match &instructions[idx] {
                Instruction::JumpBackward(num) => {
                    return swap_and_execute(
                        instructions,
                        idx,
                        Instruction::NoOp("-".to_owned() + &*num.to_string()),
                    );
                }
                Instruction::JumpForward(num) => {
                    return swap_and_execute(
                        instructions,
                        idx,
                        Instruction::NoOp("+".to_owned() + &*num.to_string()),
                    );
                }
                Instruction::NoOp(x) => {
                    let calculate_val = |val: &str| -> i32 { val.parse::<i32>().unwrap() };
                    match &x.chars().next().unwrap() {
                        '-' => {
                            return swap_and_execute(
                                instructions,
                                idx,
                                Instruction::JumpBackward(calculate_val(&x[1..])),
                            );
                        }
                        '+' => {
                            return swap_and_execute(
                                instructions,
                                idx,
                                Instruction::JumpForward(calculate_val(&x[1..])),
                            );
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            0
        })
        .sum()
}

fn swap_and_execute(instructions: &[Instruction], idx: usize, instruction: Instruction) -> i32 {
    let mut swapped_instructions = instructions.to_owned();
    swapped_instructions[idx] = instruction;
    let res = execute_instructions(&swapped_instructions);
    if res.1 != instructions.len() {
        return 0;
    }
    res.0
}
