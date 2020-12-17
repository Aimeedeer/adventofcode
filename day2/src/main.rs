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

	let scope = rules[0].split('-').collect::<Vec<_>>();
	let scope_from = scope[0].parse::<i32>()?;
	let scope_until = scope[1].parse::<i32>()?;

	let include_char = rules[1].trim_end_matches(':').parse::<char>()?;
	let password = &rules[2];

	let mut num_valid = 0;
	for c in password.chars() {
	    if c == include_char {
		num_valid += 1;
	    }
	}

	println!("include_char: {}, count: {}, scope: {}-{}, password: {:?} ", include_char, num_valid, scope_from, scope_until, password);
		
	if num_valid >= scope_from && num_valid <= scope_until {
	    num += 1;
	} else {
	    continue;
	}	
    }
    println!("{} valid passwords", num);

    Ok(())
}


