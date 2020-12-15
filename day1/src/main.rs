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
    num_list.sort();

    'outloop: for num1 in &num_list {
	for num2 in &num_list {
	    if *num2 == 2020 - num1 {
		println!("num1: {} * num2: {} is {}", num1, num2, num1*num2);
		break 'outloop;
	    }
	}
    }

    Ok(())
}

