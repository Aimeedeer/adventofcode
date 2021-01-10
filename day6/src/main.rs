use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let groups = &buffer.split("\n\n").collect::<Vec<_>>();

    let mut num1 = 0;
    let mut num2 = 0;

    for g in groups {
        let g1 = g.replace("\n", "");
        let g2 = g.trim().split('\n').collect::<Vec<_>>();

        // part 1
        let mut question_vec = Vec::new();
        for c in g1.chars() {
            if !question_vec.contains(&c) {
                question_vec.push(c);
            }
        }
        let question_num = question_vec.len();
        num1 += question_num;

        // part 2
        let mut question_vec_2 = Vec::new();
        for c in question_vec.iter() {
            let mut is_answered = true;
            for q in g2.iter() {
                if !q.contains(*c) {
                    is_answered = false;
                    break;
                }
            }
            if is_answered == true {
                question_vec_2.push(c);
            }
        }

        let question_num_2 = question_vec_2.len();
        num2 += question_num_2;
    }

    println!("num1: {}", num1);
    println!("num2: {}", num2);
    
    Ok(())
}
