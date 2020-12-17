use std::fs::File;
use std::io::{BufReader, prelude::*};
use anyhow::Result;

fn main() -> Result<()> {    
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut num: i32 = 0;
    
    for line in reader.lines() {
	let rules = line?;
	let rules = rules.trim().split(' ').collect::<Vec<_>>();

	let valid_char = rules[1].trim_end_matches(':').parse::<char>()?;
	let password = &rules[2];

	let indexes = rules[0].split('-').collect::<Vec<_>>();
	let index_1 = indexes[0].parse::<usize>()?;
	let index_2 = indexes[1].parse::<usize>()?;

	// part 1
	/*
	let mut num_valid_char = 0;
	for c in password.chars() {
	    if c == valid_char {
		num_valid_char += 1;
	    }
	}

	if num_valid_char >= index_1 && num_valid_char <= index_2 {
	    num += 1;
	} else {
	    continue;
	}
	*/

	// part 2
	let index_1 = index_1 - 1;
	let index_2 = index_2 - 1;

	if password.chars().nth(index_1) == password.chars().nth(index_2) {
	    continue;
	}
	if (password.chars().nth(index_1) == Some(valid_char))
	    || (password.chars().nth(index_2) == Some(valid_char)) {
		num += 1;
	    } else {
		continue;
	    }

    }
    println!("{} valid passwords", num);

    Ok(())
}


