use std::fs::File;
use std::io::{BufReader, prelude::*};
use anyhow::Result;

fn main() -> Result<()> {    
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut num: i32 = 0;
    
    for line in reader.lines() {
	let line = line?;
	let rules_password = line.split(':').collect::<Vec<_>>();
	let rules = rules_password[0];
	let password = rules_password[1];

	let range_char = rules.split(&['-', ' '][..]).collect::<Vec<_>>();
	let index_1 = range_char[0].parse::<usize>()?;
	let index_2 = range_char[1].parse::<usize>()?;
	let valid_char = range_char[2].parse::<char>()?;

	/*
	// Old ones
	let rules = rules.trim().split(' ').collect::<Vec<_>>();

	let valid_char = rules[1].trim_end_matches(':').parse::<char>()?;
	let password = &rules[2];

	let indexes = rules[0].split('-').collect::<Vec<_>>();
	let index_1 = indexes[0].parse::<usize>()?;
	let index_2 = indexes[1].parse::<usize>()?;
	 */
	
	// part 1
	let num_valid_char = password.chars().filter(|c| *c == valid_char).count();

	if num_valid_char >= index_1 && num_valid_char <= index_2 {
	    num += 1;
	}


	// part 2
/*
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
*/
    }
    println!("{} valid passwords", num);

    Ok(())
}
