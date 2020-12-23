use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use anyhow::anyhow;
use regex::Regex;

#[derive(Debug)]
struct PassportVec {
    data: Vec<PassportData>,
}

#[derive(Debug)]
struct PassportData {
    key: String,
    value: String,
}

impl PassportVec {
    pub fn new() -> Self {
	Self {
	    data: Vec::<PassportData>::new(),
	}
    }

    pub fn push(&self, passport: PassportData) {

    }
}

impl PassportData {
    pub fn new() -> Self {
	Self {
	    key: String::new(),
	    value: String::new(),
	}
    }
}

fn main() -> Result<()>{
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let raw_passport_vec = &buffer.split("\n\n").collect::<Vec<_>>();
    let re_passport = Regex::new(r"(([[:alpha:]]{3}):(#|\d|[[:alnum:]]+)([\s\n]))")?;

    let mut passport_vec = PassportVec::new();    
    for p in raw_passport_vec {
	let raw_passport = p.trim().split(&[' ', '\n'][..]).collect::<Vec<_>>();
	println!("{:#?}", &raw_passport);

	for raw in raw_passport {
	    let data = raw.parse::<PassportData>()?;
	    let key = &data[1];
	    let value = &data[2];

	    let passport = PassportData {
		    key,
		    value,
		};
	    passport_vec.push(passport);		
	}
    }

    Ok(())
}
