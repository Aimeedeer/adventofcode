use anyhow::Result;
use anyhow::bail;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut seat_id_vec: Vec<u32> = Vec::new();

    for line in reader.lines() {
	let rules = line?;
	let rule_row = &rules[0..7];
	let rule_column = &rules[7..10];
	
	let row_id = get_row_id(&rule_row)?;
	let column_id = get_column_id(&rule_column)?;

	const NUM_COLUMNS: u32 = 8; 
	let seat_id = row_id * NUM_COLUMNS + column_id;
	seat_id_vec.push(seat_id);
    }
    seat_id_vec.sort();

    let higest_seat_id = seat_id_vec[seat_id_vec.len() - 1];
    println!("The highest seat ID: {}", higest_seat_id);

    let my_seat_id = get_my_seat_id(seat_id_vec);
    println!("My seat ID: {}", my_seat_id);
    
    Ok(())
}

fn get_my_seat_id(all_seat_ids: Vec<u32>) -> u32 {
    let mut id = 0;
    let iter = all_seat_ids.windows(2);

    for pair in iter {
	if pair[1] - pair[0] == 2 {
	    id = pair[0] + 1;
	}
    }

    id
}

fn get_row_id(input: &str) -> Result<u32> {
    search_id(input, 128, 'F', 'B', "row id")
}

fn get_column_id(input: &str) -> Result<u32> {
    search_id(input, 8, 'L', 'R', "column id")
}

fn search_id(
    input: &str,
    len: u32,
    match_char_one: char,
    match_char_two: char,
    msg: &str	     
) -> Result<u32> {
    let range = (0..len).into_iter().collect::<Vec<_>>();
    let mut range = range.as_slice();

    for c in input.chars() {
	let temp_len = range.len();
	let (one, two) = range.split_at(temp_len/2);

	if c == match_char_one {
	    range = one;
	} else if c == match_char_two {
	    range = two;
	} else {
	    bail!("get {} failed", msg); 
	}
    }
    
    let id = range[0];    
    Ok(id)
}
