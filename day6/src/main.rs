use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let groups = &buffer.split("\n\n").collect::<Vec<_>>();

    let mut num = 0;
    for g in groups {
        let g = g.replace("\n", "");
//        println!("after: {:?}", &g);

        let mut question_vec = Vec::new();
        for c in g.chars() {
            if !question_vec.contains(&c) {
                question_vec.push(c);
            }
        }
        num += question_vec.len();
        
        println!("num: {}", num);
        println!("---------------------------------------");
    }
    
    Ok(())
}
