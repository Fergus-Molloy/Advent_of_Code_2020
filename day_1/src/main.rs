use std::{fs, process::exit};

fn main() {
    let nums: Vec<i32> = fs::read_to_string("input")
        .expect("unable to read file")
        .lines()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    // do the actual question
    for i in nums.iter() {
        for j in nums.iter() {
            for k in nums.iter() {
                if i + j + k == 2020 {
                    println!("i: {}, j: {}, k: {}\n{}", i, j, k, i * j * k);
                    exit(0);
                }
            }
        }
    }
}
