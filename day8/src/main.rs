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

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut instructions = Vec::<(String, i32, usize)>::new();

    for line in reader.lines() {
        let line = line?;
        let caps = RE.captures(&line).unwrap();

        let instruction = (caps[1].to_string(),
                           caps[2].parse::<i32>()?,
                           0);

        instructions.push(instruction);
    }

    let mut count: usize = 0;
    let mut index: usize = 0;
    let mut global_acc = 0;
        
    loop {
        let step = &instructions[index];
        
        if step.2 != 0 {
            println!("Break at index: {}; instruction: {:?}; count: {}; global_acc: {}", index, step, count, global_acc);
            break;
        }

        let jmp_num;
        
        match step.0.as_ref() {
            "nop" => {
                // it does nothing
                jmp_num = 1;
            },
            "acc" => {
                global_acc += step.1;
                jmp_num = 1;
            },
            "jmp" => {
                jmp_num = step.1;
            },
            _ => panic!(),
        }

        count += 1;        
        instructions[index].2 = count;

        index = (i32::try_from(index).unwrap() + jmp_num) as usize;
    }

    Ok(())
}
