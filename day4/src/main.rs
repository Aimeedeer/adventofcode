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
    pub pid: Option<String>,
    pub cid: Option<u32>,
    pub eyr: Option<u32>,
    pub byr: Option<u32>,
    pub iyr: Option<u32>,
    pub ecl: Option<String>,
    pub hcl: Option<String>,
    pub hgt: Option<String>,
}

impl Passport {
    pub fn new(
	pid: Option<String>,
	cid: Option<u32>,
	eyr: Option<u32>,
	byr: Option<u32>,
	iyr: Option<u32>,
	ecl: Option<String>,
	hcl: Option<String>,
	hgt: Option<String>) -> Self {
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
    let mut passport_vec: Vec<Passport> = vec![];

    for p in raw_passport_vec {
//	dbg!(&p);
	let raw_passport = p.trim().split(&[' ', '\n'][..]).collect::<Vec<_>>();
	
	let mut pid = None;
	let mut cid = None;
	let mut iyr = None;
	let mut eyr = None;
	let mut byr = None;
	let mut ecl = None;
	let mut hcl = None;
	let mut hgt = None;

	for raw_item in raw_passport {
	    let item = raw_item.split(':').collect::<Vec<_>>();

	    match item[0] {
		"pid" => { pid = Some(item[1].to_string()); },
		"cid" => { cid = Some(item[1].parse::<u32>()?); },
		"iyr" => { iyr = Some(item[1].parse::<u32>()?); },
		"eyr" => { eyr = Some(item[1].parse::<u32>()?); },
		"byr" => { byr = Some(item[1].parse::<u32>()?); },
		"ecl" => { ecl = Some(item[1].to_string()); },
		"hcl" => { hcl = Some(item[1].to_string()); },
		"hgt" => { hgt = Some(item[1].to_string()); },
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
	if passport.pid != None
	    && passport.eyr != None
	    && passport.byr != None
	    && passport.iyr != None
	    && passport.ecl != None
	    && passport.hcl != None
	    && passport.hgt != None	    
	{	    
	    num += 1;
	}
    }
    num
}
