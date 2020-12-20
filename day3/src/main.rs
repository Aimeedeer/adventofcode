use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut tree_num = 0;

    // index moving
    let mut index = 3;

    // how many chars in a line
    let mut char_num = 0;

    // ignore the first line
    for line in reader.lines().skip(1) {
	let rules = line?;
	let rules = rules.chars().collect::<Vec<char>>();

	if char_num == 0 {
	    char_num = rules.len();
	}

	if index >= char_num {
	    index = index % char_num;
	}

	if rules[index] == '#' {
	    tree_num += 1;
	}

	index += 3;
    }

    println!("{} trees", tree_num);

    Ok(())
}
