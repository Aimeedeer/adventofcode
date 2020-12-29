use anyhow::Result;
use anyhow::anyhow;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut seat_id_vec: Vec<u32> = Vec::new();
    
    for line in reader.lines() {
	let rules = line?;
	let rule_row = &rules[0..7];
	let rule_column = &rules[7..10];
	
	let row_id = get_row_id(&rule_row)?;
	let column_id = get_column_id(&rule_column)?;

	let seat_id = row_id * 8 + column_id;
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
    let mut range = create_vec(128);
    
    for c in input.chars() {
	let len = range.len();
	let (front, back) = range.split_at(len/2);

	match c {
	    'F' => {
		range = front.to_vec();
	    },
	    'B' => {
		range = back.to_vec();
	    },
	    _ => { anyhow!("get row id failed"); },
	}
    }
    
    let row_id = range[0];
    Ok(row_id)
}

fn get_column_id(input: &str) -> Result<u32> {
    let mut range = create_vec(8);

    for c in input.chars() {
	let len = range.len();
	let (left, right) = range.split_at(len/2);

	match c {
	    'L' => {
		range = left.to_vec();
	    },
	    'R' => {
		range = right.to_vec();
	    }
	    _ => { anyhow!("get column id failed"); },
	}
    }

    let column_id = range[0];    
    Ok(column_id)
}

fn create_vec(len: u32) -> Vec<u32> {
    let mut rows: Vec<u32> = Vec::with_capacity(len as usize);
    
    for i in 0..len {
        rows.push(i);
    }
    rows
}
