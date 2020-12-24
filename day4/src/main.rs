use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use anyhow::anyhow;
use regex::Regex;

/*
#[derive(Debug)]
struct PassportVec {
    passports: Vec<Passport>,
}
*/

#[derive(Debug)]
struct Passport {
    pub pid: String,
    pub cid: u32,
    pub eyr: u32,
    pub byr: u32,
    pub iyr: u32,
    pub ecl: String,
    pub hcl: String,
    pub hgt: String,
}

impl Passport {
    pub fn new(
	pid: String,
	cid: u32,
	eyr: u32,
	byr: u32,
	iyr: u32,
	ecl: String,
	hcl: String,
	hgt: String) -> Self {
	Self {
	    pid,
	    cid, 
	    eyr,
	    byr,
	    iyr,
	    ecl,
	    hcl,
	    hgt,
	}
    }
}

fn main() -> Result<()>{
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let raw_passport_vec = &buffer.split("\n\n").collect::<Vec<_>>();
//    let re_passport = Regex::new(r"(([[:alpha:]]{3}):(#|\d|[[:alnum:]]+)([\s\n]))")?;
    
    let mut passport_vec: Vec<Passport> = vec![];

    for p in raw_passport_vec {
	let raw_passport = p.trim().split(&[' ', '\n'][..]).collect::<Vec<_>>();
	
	let mut pid = String::new();
	let mut cid = 0;
	let mut iyr = 0;
	let mut eyr = 0;
	let mut byr = 0;
	let mut ecl = String::new();
	let mut hcl = String::new();
	let mut hgt = String::new();

	for raw_item in raw_passport {
//	    dbg!(&raw_item);
	    let item = raw_item.split(':').collect::<Vec<_>>();

	    match item[0] {
		"pid" => {
		    pid = item[1].to_string();
//		    dbg!(&pid);
		},
		"cid" => {
		    cid = item[1].parse::<u32>()?;
//			   dbg!(cid);
		},
		"iyr" => { iyr = item[1].parse::<u32>()?; },
		"eyr" => { eyr = item[1].parse::<u32>()?; },
		"byr" => { byr = item[1].parse::<u32>()?; },
		"ecl" => { ecl = item[1].to_string(); },
		"hcl" => { hcl = item[1].to_string(); },
		"hgt" => { hgt = item[1].to_string(); },
		_ => {},
	    };
	}

	let passport = Passport::new(pid, cid, iyr, eyr, byr, ecl, hcl, hgt);
	passport_vec.push(passport);
    }

    println!("Valid passports: {}", verify(passport_vec));
    Ok(())
}

fn verify(passport_vec: Vec<Passport>) -> u32 {
    let mut num = 0;

    for passport in passport_vec {
	if &passport.pid != ""
	    && passport.eyr != 0
	    && passport.byr != 0
	    && passport.iyr != 0
	    && &passport.ecl != ""
	    && &passport.hcl != ""
	    && &passport.hgt != ""	    
	{	    
	    num += 1;
	}
    }
    num
}
