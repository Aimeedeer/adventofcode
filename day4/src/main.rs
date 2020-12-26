use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use anyhow::anyhow;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Year {
    year: u32,
}

impl Year {
    pub fn eyr(year: u32) -> Result<Year> {
	if year >= 2020 && year <= 2030 {
	    Ok(Year { year, })
	} else {
	    Err(anyhow!("invalid eyr: {}", year))
	}
    }
    
    pub fn iyr(year: u32) -> Result<Year> {
	if year >= 2010 && year <= 2020 {
	    Ok(Year { year, })
	} else {
	    Err(anyhow!("invalid iyr: {}", year))
	}
    }
    
    pub fn byr(year: u32) -> Result<Year> {
	if year >= 1920 && year <= 2002 {
	    Ok(Year { year, })
	} else {
	    Err(anyhow!("invalid byr: {}", year))
	}
    }
}

 

#[derive(Debug, PartialEq, Clone)]
struct Passport {
    pub pid: Option<String>,
    pub cid: Option<u32>,
    pub	eyr: Option<Year>,
    pub	byr: Option<Year>,
    pub	iyr: Option<Year>,
    pub ecl: Option<String>,
    pub hcl: Option<String>,
    pub hgt: Option<String>,
}

impl Passport {
    pub fn new(
	pid: Option<String>,
	cid: Option<u32>,
	eyr: Option<Year>,
	byr: Option<Year>,
	iyr: Option<Year>,
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

	for raw_item in raw_passport {
	    let item = raw_item.split(':').collect::<Vec<_>>();

	    match item[0] {
		"pid" => { pid = pid_is_valid(item[1].to_string()).ok(); },
		"cid" => { cid = Some(item[1].parse::<u32>()?); },
		"eyr" => { eyr = Year::eyr(item[1].parse::<u32>()?).ok(); },
		"byr" => { byr = Year::byr(item[1].parse::<u32>()?).ok(); },
		"iyr" => { iyr = Year::iyr(item[1].parse::<u32>()?).ok(); },
		"ecl" => { ecl = ecl_is_valid(item[1].to_string()).ok(); },
		"hcl" => { hcl = hcl_is_valid(item[1].to_string()).ok(); },
		"hgt" => { hgt = hgt_is_valid(item[1].to_string()).ok(); },
		_ => {},
	    };
	}

	let passport = Passport::new(pid, cid, eyr, byr, iyr, ecl, hcl, hgt);

//	dbg!(&passport);
	passport_vec.push(passport);
    }

    println!("hi &passport_vec.len(): {}", &passport_vec.len());
    println!("Valid passports: {}", verify(passport_vec));

    Ok(())
}

fn verify(passport_vec: Vec<Passport>) -> u32 {
    let mut num = 0;
    for passport in passport_vec {
	if passport.is_valid() {
	    println!("{:#?}", &passport);

	    num += 1;
	}
    }
    num
}

pub fn pid_is_valid(pid: String) -> Result<String> {
    let re = Regex::new(r"(\d{9})").unwrap();
    let caps = re.captures(&pid).ok_or(anyhow!("invalid pid: {:?}", pid))?;
    let pid = caps[1].to_string();

    Ok(pid)
}

pub fn hcl_is_valid(hcl: String) -> Result<String> {
    let re = Regex::new(r"#(([a-f]|[0-9]){6})").unwrap();
    let caps = re.captures(&hcl).ok_or(anyhow!("invalid hcl: {:?}", hcl))?;
    let hcl = caps[1].to_string();
    
    Ok(hcl)
}

pub fn ecl_is_valid(ecl: String) -> Result<String> {
    match ecl.as_ref() {
	"amb" | "blu" |	"brn" |	"gry" |	"grn" |	"hzl" | "oth" => Ok(ecl),
	_ => Err(anyhow!("invalid ecl: {:?}", ecl)),
    }
}

pub fn hgt_is_valid(hgt: String) -> Result<String> {
    let err = Err(anyhow!("invalid hgt: {:?}", hgt));
    let len = hgt.len();
    let num = &hgt[0..(len - 2)].parse::<i32>()?;
    
    if hgt.ends_with("cm") {	
	if *num >= 150 && *num <= 193 {
	    Ok(hgt)
	} else {
	    err
	}
    } else if hgt.ends_with("in") {
	if *num >= 59 && *num <= 76 {
	    Ok(hgt)
	} else {
	    err
	}
    } else {
	err 
    }
}

