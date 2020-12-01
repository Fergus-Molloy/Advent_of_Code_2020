use std::{process::exit, fs};

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename)
        .expect("unable to read file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut nums = Vec::new();

    for x in lines.iter(){
        let item = match x.parse::<i32>() {
            Ok(i) => i,
            _ => continue
        };
        nums.push(item);
    }

    // do the actual question
    for i in nums.iter(){
        for j in nums.iter() {
            for k in nums.iter() {
                if i+j+k==2020{
                    println!("i: {}, j: {}, k: {}\n{}", i, j, k, i*j*k);
                    exit(0);
                }
            }
        }
    }
}
