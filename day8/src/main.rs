extern crate lazy_static;
extern crate regex;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{prelude::*, BufReader};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w+)\s([-+]?\d+)$").unwrap();
}

#[derive(Debug, Clone)]
struct Operation {
    op: String,
    num: i32,
    sequence: usize,
}

fn main() -> Result<()> {
    let instructions = parser("input.txt")?;
    correct_instructions(instructions)
}

fn parser(file: &str) -> Result<Vec<Operation>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut instructions = Vec::<Operation>::new();

    for line in reader.lines() {
        let line = line?;
        let caps = RE.captures(&line).unwrap();

        let operation = Operation {
            op: caps[1].to_string(),
            num: caps[2].parse::<i32>()?,
            sequence: 0,
        };

        instructions.push(operation);
    }

    Ok(instructions)
}

fn correct_instructions(instructions: Vec<Operation>) -> Result<()> {
    for (index, operation) in instructions.iter().enumerate() {
        let mut ins = instructions.to_vec();

        match operation.op.as_ref() {
            "nop" => {
                ins[index].op = "jmp".to_string();
            }
            "acc" => {}
            "jmp" => {
                ins[index].op = "nop".to_string();
            }
            _ => unreachable!(),
        }

        if does_program_terminate(ins.clone()) {
            println!("It works!");
            break;
        }
    }

    Ok(())
}

fn does_program_terminate(mut instructions: Vec<Operation>) -> bool {
    let mut op_sequence = 0;
    let mut op_index = 0;
    let mut global_acc = 0;
    let len = instructions.len();
    let mut result = false;

    loop {
        if op_index == len - 1 {
            println!("Loop terminates, and the global_acc is: {}", global_acc);
            result = true;
            break;
        }

        if op_index >= len {
            println!("Infinite loop caused by index: {}", op_index);
            break;
        }

        let operation = &instructions[op_index];
        if operation.sequence != 0 {
            println!(
                "Infinite loop breaks at index: {}; operation: {:?}; sequence: {}; global_acc: {}",
                op_index, operation, op_sequence, global_acc
            );

            break;
        }

        let jmp_num;

        match operation.op.as_ref() {
            "nop" => {
                jmp_num = 1;
            }
            "acc" => {
                global_acc += operation.num;
                jmp_num = 1;
            }
            "jmp" => {
                jmp_num = operation.num;
            }
            _ => unreachable!(),
        }

        op_sequence += 1;
        instructions[op_index].sequence = op_sequence;

        op_index = (i32::try_from(op_index).unwrap() + jmp_num) as usize;
    }

    result
}
