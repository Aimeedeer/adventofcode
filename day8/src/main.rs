extern crate regex;
extern crate lazy_static;

use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use lazy_static::lazy_static;
use std::convert::TryFrom;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w+)\s([-+]?\d+)$").unwrap(); 
}

#[derive(Debug)]
struct Operation {
    op: String,
    num: i32,
    sequence: usize,
}



fn main() -> Result<()> {
    let instructions = parser("input.txt")?;    
    run_instructions(instructions)
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

fn run_instructions(mut instructions: Vec<Operation>) -> Result<()>{
    let mut op_sequence = 0;
    let mut op_index = 0;
    let mut global_acc = 0;
    
    loop {
        let operation = &instructions[op_index];
        
        if operation.sequence != 0 {
            println!("Break at index: {}; operation: {:?}; sequence: {}; global_acc: {}", op_index, operation, op_sequence, global_acc);
            break;
        }

        let jmp_num;
        
        match operation.op.as_ref() {
            "nop" => {
                jmp_num = 1;
            },
            "acc" => {
                global_acc += operation.num;
                jmp_num = 1;
            },
            "jmp" => {
                jmp_num = operation.num;
            },
            _ => unreachable!(),
        }

        op_sequence += 1;        
        instructions[op_index].sequence = op_sequence;

        op_index = (i32::try_from(op_index).unwrap() + jmp_num) as usize;
    }
    
    Ok(())
}
