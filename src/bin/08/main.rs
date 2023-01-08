extern crate core;

use itertools::Itertools;
use std::collections::HashSet;

type Instruction = (String, isize);

fn main() {
    let instructions = parse_input(include_str!("input.txt"));
    println!("{}", get_cycle_acc(&instructions));
    println!("{}", get_correct_output(&instructions));
}

fn get_cycle_acc(instructions: &[Instruction]) -> isize {
    get_acc(instructions, -1).0
}

fn get_correct_output(instructions: &[Instruction]) -> isize {
    let (swap, _) = instructions
        .iter()
        .enumerate()
        .filter(|(_, (c, _))| c == "nop" || c == "jmp")
        .find(|(i, _)| get_acc(instructions, *i as isize).1)
        .unwrap();

    get_acc(instructions, swap as isize).0
}

fn get_acc(instructions: &[Instruction], swap: isize) -> (isize, bool) {
    let mut end = true;
    let mut acc = 0;
    let mut i: isize = 0;
    let mut visited = HashSet::new();

    while i < instructions.len() as isize {
        if visited.contains(&i) {
            end = false;
            break;
        }

        let (cmd, n) = &instructions[i as usize];
        let mut cmd = &cmd.as_str();

        if swap == i {
            cmd = match *cmd {
                "nop" => &"jmp",
                "jmp" => &"nop",
                _ => cmd,
            }
        }

        visited.insert(i);

        match *cmd {
            "acc" => {
                acc += n;
                i += 1;
            }
            "jmp" => i += n,
            _ => i += 1,
        };
    }

    (acc, end)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            (
                l.split(' ').next().unwrap().to_string(),
                l.split(' ').nth(1).unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect_vec()
}
