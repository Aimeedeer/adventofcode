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

    Ok(())
}

fn get_row_id(input: &str) -> Result<u32> {
    let mut current_rows = create_vec(128);
    
    for c in input.chars() {
	let len = current_rows.len();
	let (front, back) = current_rows.split_at(len/2);

	match c {
	    'F' => {
		current_rows = front.to_vec();
	    },
	    'B' => {
		current_rows = back.to_vec();
	    },
	    _ => { anyhow!("get row id failed"); },
	}
	dbg!(&current_rows);
    }
    
    let row_id = current_rows[0];
    Ok(row_id)
}

fn get_column_id(input: &str) -> Result<u32> {
    let mut current_columns = create_vec(8);

    for c in input.chars() {
	let len = current_columns.len();
	let (left, right) = current_columns.split_at(len/2);

	match c {
	    'L' => {
		current_columns = left.to_vec();
	    },
	    'R' => {
		current_columns = right.to_vec();
	    }
	    _ => { anyhow!("get column id failed"); },
	}
    }

    let column_id = current_columns[0];    
    Ok(column_id)
}

fn create_vec(len: u32) -> Vec<u32> {
    let mut rows: Vec<u32> = Vec::with_capacity(len as usize);
    
    for i in 0..len {
        rows.push(i);
    }
    rows
}
