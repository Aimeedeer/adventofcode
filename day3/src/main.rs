use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<()> {
    let num1 = path_rules(1, 1)?;
    let num2 = path_rules(3, 1)?;
    let num3 = path_rules(5, 1)?;
    let num4 = path_rules(7, 1)?;
    let num5 = path_rules(1, 2)?;
    
    println!("{:?} trees", num1 * num2 * num3 * num4 * num5);

    Ok(())
}

fn path_rules(move_right: usize, move_down: usize) -> Result<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut tree_num = 0;
    let mut index = 0;

    // third round/final
    for line in reader.lines().step_by(move_down) {
	let rules = line?;
	let rules = rules.chars().collect::<Vec<char>>();
	let char_num = rules.len();

	if rules[index] == '#' {
	    tree_num += 1;
	} 

	index += move_right;
	index %= char_num;
    }


    /*
    // first round
    let mut index = move_right;
    for (line_index, line_value) in reader.lines().enumerate().skip(1) {
	if line_index % move_down == 0 {
	    let rules = line_value?;
	    let rules = rules.chars().collect::<Vec<char>>();
	    let char_num = rules.len();

	    if rules[index] == '#' {
		tree_num += 1;
	    } 
	    
	    index += move_right;
	    index %= char_num;
	}
    }
     */

    Ok(tree_num)
}

