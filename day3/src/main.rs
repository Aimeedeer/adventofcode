use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<()> {
    
    let num1 = path_rules(1, 1)?;
    let num2 = path_rules(3, 1)?;
    let num3 = path_rules(5, 1)?;
    let num4 = path_rules(7, 1)?;
    let num5 = path_rules(1, 2)?;

    dbg!(num1);
    dbg!(num2);
    dbg!(num3);
    dbg!(num4);
    dbg!(num5);
    
    println!("{:?} trees", num1 * num2 * num3 * num4 * num5);

    Ok(())
}

fn path_rules(move_right: usize, move_down: usize) -> Result<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut index = move_right;
    let mut tree_num = 0;

    for (line_index, line_value) in reader.lines().enumerate().skip(1) {
	if move_down > 1 {
	    match (line_index, line_value) {
		(line_index, line_value) if line_index % move_down == 0 =>  {
		    let rules = line_value?;
		    let rules = rules.chars().collect::<Vec<char>>();
		    tree_num += counting_with_rules(rules, index);
		}
		_ =>  continue
	    };
	} else {
	    let rules = line_value?;
	    let rules = rules.chars().collect::<Vec<char>>();
	    tree_num += counting_with_rules(rules, index);
	}	

	index += move_right;
    }

    Ok(tree_num)
}

fn counting_with_rules (rules: Vec<char>, index: usize) -> usize {
    let char_num = rules.len();
    let index = index % char_num;

    if rules[index] == '#' {
	1
    } else {
	0
    }
}
