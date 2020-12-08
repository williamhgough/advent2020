use std::collections::HashMap;

static INPUT: &str = std::include_str!("../../../input/8.txt");

fn main() {
    let instructions = INPUT.split('\n').collect::<Vec<&str>>();
    let part_one_result = part_one(&instructions);
    println!("Part 1: {}", part_one_result);
}

fn part_one(instructions: &[&str]) -> i32 {
    let mut visited: HashMap<usize, bool> = HashMap::new();
    let mut acc = 0;
    let mut idx = 0;
    while idx < instructions.len() {
        let instruction = Instruction::from_str(instructions[idx].trim());
        if visited.get(&idx).is_some() {
            break;
        }

        match instruction {
            Instruction::NoOp(_) => {
                visited.insert(idx, true);
                idx += 1;
            }
            Instruction::JumpForward(x) => {
                visited.insert(idx, true);
                idx = idx.checked_add(x as usize).unwrap();
            }
            Instruction::JumpBackward(x) => {
                visited.insert(idx, true);
                idx = idx.checked_sub(x as usize).unwrap();
            }
            Instruction::Accumulate(x) => {
                acc += x;
                visited.insert(idx, true);
                idx += 1;
            }
            _ => {}
        };
    }
    acc
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Instruction {
    NoOp(i32),
    JumpForward(i32),
    JumpBackward(i32),
    Accumulate(i32),
    Unknown,
}

impl Instruction {
    fn from_str(str: &str) -> Self {
        let calculate_val = |val: &str| -> i32 { val.parse::<i32>().unwrap() };
        match &str[0..3] {
            "nop" => Instruction::NoOp(calculate_val(&str[4..])),
            "acc" => Instruction::Accumulate(calculate_val(&str[4..])),
            "jmp" => match &str.chars().nth(4).unwrap() {
                '-' => Instruction::JumpBackward(calculate_val(&str[5..])),
                '+' => Instruction::JumpForward(calculate_val(&str[5..])),
                _ => Instruction::Unknown,
            },
            _ => Instruction::Unknown,
        }
    }
}
