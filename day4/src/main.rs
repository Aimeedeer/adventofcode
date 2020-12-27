use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use anyhow::Result;
use anyhow::anyhow;
use regex::Regex;

fn main() -> Result<()>{
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let raw_passport_vec = &buffer.split("\n\n").collect::<Vec<_>>();
    let mut passport_vec: Vec<Passport> = vec![];
    
    for p in raw_passport_vec {
	let raw_passport = p.trim().split(&[' ', '\n'][..]).collect::<Vec<_>>();

	let mut new_passport = RawPassport::default();
	
	fn validate<T>(dest: &mut Option<T>,
		       reference: &str,
		       v: impl FnOnce(&str) -> Result<T>) {
	    *dest = v(reference).ok();	    	    
	}
	
	for raw_item in raw_passport {
	    let item = raw_item.split(':').collect::<Vec<_>>();
	    match item[0] {	
		"pid" => validate(&mut new_passport.pid, item[1], validate_pid),
		"cid" => validate(&mut new_passport.cid, item[1], validate_cid),
		"eyr" => validate(&mut new_passport.eyr, item[1], validate_eyr),
		"byr" => validate(&mut new_passport.byr, item[1], validate_byr),
		"iyr" => validate(&mut new_passport.iyr, item[1], validate_iyr),
		"ecl" => validate(&mut new_passport.ecl, item[1], validate_ecl),
		"hcl" => validate(&mut new_passport.hcl, item[1], validate_hcl),
		"hgt" => validate(&mut new_passport.hgt, item[1], validate_hgt),
		_ => {},
	    };
	}

	if let Some(passport) = Passport::new(new_passport){
	    passport_vec.push(passport);
	}	
    }

    println!("Valid passports: {}", passport_vec.len());
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Default)]
struct RawPassport {
    pid: Option<String>,
    cid: Option<u32>,
    eyr: Option<u32>,
    byr: Option<u32>,
    iyr: Option<u32>,
    ecl: Option<String>,
    hcl: Option<String>,
    hgt: Option<String>,
}

impl RawPassport {
    fn is_valid(&self) -> bool {
	self.pid.is_some()
	    && self.eyr.is_some()
	    && self.byr.is_some()
	    && self.iyr.is_some() 
	    && self.ecl.is_some() 
	    && self.hcl.is_some() 
	    && self.hgt.is_some() 
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Passport {
    pid: String,
    cid: Option<u32>,
    eyr: u32,
    byr: u32,
    iyr: u32,
    ecl: String,
    hcl: String,
    hgt: String,
}

impl Passport {
    fn new(raw_passport: RawPassport) -> Option<Passport> {
	if raw_passport.is_valid() {
	    Some(Passport {
		pid: raw_passport.pid.unwrap(),
		cid: raw_passport.cid,
		eyr: raw_passport.eyr.unwrap(),
		byr: raw_passport.byr.unwrap(),
		iyr: raw_passport.iyr.unwrap(),
		ecl: raw_passport.ecl.unwrap(),
		hcl: raw_passport.hcl.unwrap(),
		hgt: raw_passport.hgt.unwrap(),
	    })
	} else {
	    None
	}	    
    }
}

fn validate_pid(pid: &str) -> Result<String> {
    parse_and_capture(r"(\d{9})", pid, "pid")
}

fn validate_cid(cid: &str) -> Result<u32> {
    Ok(cid.parse::<u32>()?)
}

fn validate_eyr(year: &str) -> Result<u32> {
    validate_year(year, 2020, 2030, "eyr")    
}

fn validate_iyr(year: &str) -> Result<u32> {
    validate_year(year, 2010, 2020, "iyr")    
}

fn validate_byr(year: &str) -> Result<u32> {
    validate_year(year, 1920, 2002, "byr")    
}

fn validate_hcl(hcl: &str) -> Result<String> {
    parse_and_capture(r"#(([a-f]|[0-9]){6})", hcl, "hcl")
}

fn validate_ecl(ecl: &str) -> Result<String> {
    parse_and_capture(r"(amb|blu|brn|gry|grn|hzl|oth)", ecl, "ecl")
}

fn validate_hgt(hgt: &str) -> Result<String> {
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

fn parse_and_capture<T: FromStr>(rule: &str, input: &str, msg: &str) -> Result<T> {
    let re = Regex::new(rule).unwrap();
    let caps = re.captures(input).ok_or(anyhow!("invalid {}: {}", msg, input))?;

    let output = caps[1].parse::<T>().map_err(|_| anyhow!("error parsing {}: {}", msg, input))?;
    Ok(output)
}

fn validate_year(year: &str, least: u32, most: u32, msg: &str) -> Result<u32> {
    let year = parse_and_capture(r"(\d{4})", year, "year")?;
    
    if year >= least && year <= most {
	Ok(year)
    } else {
	Err(anyhow!("invalid {}: {}", msg, year))
    }
}
