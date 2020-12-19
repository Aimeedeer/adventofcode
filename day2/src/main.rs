use std::fs::File;
use std::io::{BufReader, prelude::*};
use anyhow::Result;
use anyhow::anyhow;

fn main() -> Result<()> {    
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut num: i32 = 0;
    
    for line in reader.lines() {
	let line = line?;
	let mut rules_password = line.split(':');
	let rules = rules_password.next().ok_or(anyhow!("parse rules failed"))?;
	let password = rules_password.next().ok_or(anyhow!("parse password failed"))?;

	let mut range_char = rules.split(&['-', ' '][..]);
	let index_1 = range_char
	    .next()
	    .ok_or(anyhow!("parse index one failed"))?
	    .parse::<usize>()?;
	let index_2 = range_char
	    .next()
	    .ok_or(anyhow!("parse index two failed"))?
	    .parse::<usize>()?;
	let valid_char = range_char
	    .next()
	    .ok_or(anyhow!("parse valid char failed"))?
	    .parse::<char>()?;

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
