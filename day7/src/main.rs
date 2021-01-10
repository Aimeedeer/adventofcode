use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> Result<()> {
    const MY_BAG: &str = "shiny gold";

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut map = HashMap::<String, String>::new();
    let mut possiable_bags = Vec::new();
    let mut num = 0;
    
    for line in reader.lines() {
        let bags = line?;
	let bags = bags.split("contain").collect::<Vec<_>>();
        map.insert(bags[0].to_string(), bags[1].to_string());

        if bags[1].contains(&MY_BAG) {
            possiable_bags.push(bags[0].trim().to_string());
        }
    }
    println!("possiable_bags: {:?}", possiable_bags);

    possiable_bags.push(MY_BAG.to_string());
    println!("possiable_bags_2: {:?}", possiable_bags);

    
    for (k, v) in map.iter() {
        for bag in &possiable_bags {
            // println!("map: {}, {}", k, v);

            if v.contains(bag) {
                num += 1;
            }
        }
    }

    println!("num: {}", num);
    
    Ok(())
}
