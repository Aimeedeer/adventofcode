use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use anyhow::Result;
use anyhow::anyhow;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Passport {
    pub pid: Option<String>,
    pub cid: Option<u32>,
    pub	eyr: Option<u32>,
    pub	byr: Option<u32>,
    pub	iyr: Option<u32>,
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

    pub fn is_valid(&self) -> bool {
	self.pid.is_some()
	    && self.eyr.is_some()
	    && self.byr.is_some()
	    && self.iyr.is_some() 
	    && self.ecl.is_some() 
	    && self.hcl.is_some() 
	    && self.hgt.is_some() 
    }
}

fn main() -> Result<()>{
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let raw_passport_vec = &buffer.split("\n\n").collect::<Vec<_>>();
    let mut passport_vec: Vec<Passport> = vec![];
    
    for p in raw_passport_vec {
	let raw_passport = p.trim().split(&[' ', '\n'][..]).collect::<Vec<_>>();
	
	let mut pid = None;
	let mut cid = None;
	let mut iyr = None;
	let mut eyr = None;
	let mut byr = None;
	let mut ecl = None;
	let mut hcl = None;
	let mut hgt = None;

	fn validate<T>(dest: &mut Option<T>,
		       reference: &str,
		       v: impl FnOnce(&str) -> Result<T>) {
	    *dest = v(reference).ok();	    	    
	}
	
	for raw_item in raw_passport {
	    let item = raw_item.split(':').collect::<Vec<_>>();
	    match item[0] {	
		"pid" => validate(&mut pid, item[1], validate_pid),
		"cid" => validate(&mut cid, item[1], validate_cid),
		"eyr" => validate(&mut eyr, item[1], validate_eyr),
		"byr" => validate(&mut byr, item[1], validate_byr),
		"iyr" => validate(&mut iyr, item[1], validate_iyr),
		"ecl" => validate(&mut ecl, item[1], validate_ecl),
		"hcl" => validate(&mut hcl, item[1], validate_hcl),
		"hgt" => validate(&mut hgt, item[1], validate_hgt),
		_ => {},
	    };
	}

	let passport = Passport::new(pid, cid, eyr, byr, iyr, ecl, hcl, hgt);
	if passport.is_valid() {
	    println!("{}\n", p);
	}
	
	passport_vec.push(passport);
    }

    println!("Valid passports: {}", verify(passport_vec));

    Ok(())
}

fn verify(passport_vec: Vec<Passport>) -> u32 {
    let mut num = 0;
    for passport in passport_vec {
	if passport.is_valid() {
	    num += 1;
	}
    }
    num
}

pub fn validate_pid(pid: &str) -> Result<String> {
    let re = Regex::new(r"(\d{9})").unwrap();
    let caps = re.captures(pid).ok_or(anyhow!("invalid pid: {:?}", pid))?;
    let pid = caps[1].to_string();

    Ok(pid)
}

pub fn validate_cid(cid: &str) -> Result<u32> {
    Ok(cid.parse::<u32>()?)
}

fn parse_and_capture<T: FromStr>(rule: &str, input: &str, msg: &str) -> Result<T> {
    let re = Regex::new(rule).unwrap();
    let caps = re.captures(input).ok_or(anyhow!("invalid {}: {}", msg, input))?;

    let output = caps[1].parse::<T>().map_err(|_| anyhow!("error parsing {}: {}", msg, input))?;
    Ok(output)
}

pub fn validate_eyr(year: &str) -> Result<u32> {
    let year = parse_and_capture(r"(\d{4})", year, "year")?;
    
    if year >= 2020 && year <= 2030 {
	Ok(year)
    } else {
	Err(anyhow!("invalid eyr: {}", year))
    }
}

pub fn validate_iyr(year: &str) -> Result<u32> {
    let year = parse_and_capture(r"(\d{4})", year, "year")?;

    if year >= 2010 && year <= 2020 {
	Ok(year)
    } else {
	Err(anyhow!("invalid iyr: {}", year))
    }
}

pub fn validate_byr(year: &str) -> Result<u32> {
    let year = parse_and_capture(r"(\d{4})", year, "year")?;

    if year >= 1920 && year <= 2002 {
	Ok(year)
    } else {
	Err(anyhow!("invalid byr: {}", year))
    }
}

pub fn validate_hcl(hcl: &str) -> Result<String> {
    let hcl = parse_and_capture(r"#(([a-f]|[0-9]){6})", hcl, "hcl")?;

    Ok(hcl)
}

pub fn validate_ecl(ecl: &str) -> Result<String> {
    let ecl = parse_and_capture(r"(amb|blu|brn|gry|grn|hzl|oth)", ecl, "ecl")?;
    Ok(ecl)
}

pub fn validate_hgt(hgt: &str) -> Result<String> {
    let err = Err(anyhow!("invalid hgt: {:?}", hgt));
    let len = hgt.len();
    let num = hgt[0..(len - 2)].parse::<i32>()?;

    if hgt.ends_with("cm") {	
	if num >= 150 && num <= 193 {
	    Ok(hgt.to_string())
	} else {
	    err
	}
    } else if hgt.ends_with("in") {
	if num >= 59 && num <= 76 {
	    Ok(hgt.to_string())
	} else {
	    err
	}
    } else {
	err 
    }
}

