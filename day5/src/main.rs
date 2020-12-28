use anyhow::Result;
use anyhow::anyhow;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<()>{
    let mut file = File::open("input.txt")?;
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

    // todo: sort seat_id_vec and find the highest num
    let higest_seat_id = get_highest_seat_id(&seat_id_vec);
    println!("The highest seat ID: {}", higest_seat_id);

    Ok(())
}

fn get_row_id(input: &str) -> Result<u32> {
    let mut row_id = 0;
    let mut current_rows = create_rows();
    
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
    
    Ok(row_id)
}

fn get_column_id(input: &str) -> Result<u32> {
    let mut column_id = 0;
    
    Ok(column_id)
}

fn get_highest_seat_id(list: &Vec<u32>) -> u32 {
    let mut seat_id = 0;

    seat_id
}

fn create_rows() -> Vec<u32> {
    let mut rows: Vec<u32> = Vec::with_capacity(128);
    
    for i in 0..128 {
        rows.push(i);
    }
    dbg!(&rows);
    rows
}
