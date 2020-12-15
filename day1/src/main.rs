use std::fs::File;
use std::io::{BufReader, prelude::*};
use anyhow::Result;

fn main() -> Result<()> {    
    let file = File::open("report.txt")?;
    let reader = BufReader::new(file);

    let mut num_list = Vec::new();
    for line in reader.lines() {
	let num = line?.parse::<i32>()?;
	num_list.push(num);
    }

    'outloop: for num1 in &num_list {
	for num2 in &num_list {
	    for num3 in &num_list {
		if num1 + num2 + num3 == 2020 {
		    println!("num1: {} * num2: {} * num3: {} is {}", num1, num2, num3, num1*num2*num3);
		    break 'outloop;
		}
	    }
	}
    }

    Ok(())
}

