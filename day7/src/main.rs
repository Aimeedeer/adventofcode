extern crate regex;
extern crate lazy_static;

use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

const MY_BAG: &str = "shiny gold";

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.*)$").unwrap();
    static ref SUB_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

fn main() -> Result<()> {
    let mut num = 0;
    let bags = parser("input.txt")?;

    for bag in bags.keys() {
        if is_my_bag_in_map(bag, &bags) {
            num += 1;
        } 
    }
    
    println!("num: {}", num);
    Ok(())
}

fn parser(input: &str) -> Result<HashMap<String, HashMap<String, u32>>> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut bag_map = HashMap::<String, HashMap<String, u32>>::new();
        
    for line in reader.lines() {
        let line = line?;
        let caps = RE.captures(&line).unwrap();

        let mut items = HashMap::<String, u32>::new();

        for cap_sub in SUB_RE.captures_iter(&caps[2]) {
            let bag_num = cap_sub[1].parse::<u32>()?;
            let bag_color = &cap_sub[2];
            items.insert(bag_color.to_string(), bag_num);
        }

        bag_map.insert(caps[1].to_string(), items);
    }    
    Ok(bag_map)
}

fn is_my_bag_in_map(candidate_bag: &str, map: &HashMap<String, HashMap<String, u32>>) -> bool {
    let bags = map.get(candidate_bag).unwrap();

    for (k, v) in bags.iter() {
        if k == MY_BAG {
            return true;
        } else {
            if is_my_bag_in_map(k, map) {
                return true;
            } else {
                continue;
            }
        }
    }
    false
}
