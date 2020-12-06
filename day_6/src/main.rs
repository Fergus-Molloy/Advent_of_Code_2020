use std::fs;
use std::collections::HashMap;
fn main() {
    let contents = fs::read_to_string("input").expect("unable to read file");
    println!("Part One: {}", part_one(&contents));
    println!("Part Two: {}", part_two(&contents));

}

fn part_one(contents: &String) -> u32 {
    let mut total = 0;
    let mut count = 0;
    let mut answers: HashMap<char, u32> = HashMap::new();

    for x in contents.split('\n'){
        if x.len() == 0 {
            // add to total and reset
            total += count;
            count = 0;
            answers = HashMap::new();
        } else {
            for y in x.chars(){
                if !answers.contains_key(&y) {
                    answers.insert(y, 1);
                    count +=1
                }
            }
        }
    }
    total
}

fn part_two(contents: &String) -> u32 {
    let mut total = 0;
    let mut count = 0;
    let mut lines = 0;
    let mut answers: HashMap<char, u32> = HashMap::new();

    for x in contents.split('\n'){
        if x.len() == 0 {
            for (_, val) in answers.iter(){
                if *val == lines {
                    count +=1;
                }
            }
            total += count;

            // reset everything
            count = 0;
            lines = 0;
            answers = HashMap::new();
        } else {
            lines+=1;
            for y in x.chars(){
                if !answers.contains_key(&y) {
                    answers.insert(y, 1);
                }
                else{
                    //keep track of number of answers
                    *answers.get_mut(&y).unwrap() += 1;
                }
            }
        }
    }
    total
}
