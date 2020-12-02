use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("unable to read file");
    let lines: Vec<&str> = contents.lines()
        .map(|x| x.trim())
        .collect();
    println!("part one: {}", part_one(&lines));
    println!("part two: {}", part_two(&lines));

}

fn part_one(lines: &Vec<&str>) -> u32{
    let mut total = 0;
    for x in lines.iter(){
        let mut count = extract_values(x);
        let password=x.split(':').last().unwrap();
        for y in password.chars() {
            if y == count.c {
                count.count += 1;
            }
        }
        if count.count >= count.low_range && count.count <= count.high_range{
            total +=1;
        }
    }
    total
}

fn part_two(lines: &Vec<&str>) -> u32 {
    let mut total = 0;
    for x in lines.iter() {
        let count = extract_values(x);
        let password = x.split(':').last().unwrap();

        let low_char = password.chars().nth(count.low_range as usize).unwrap();
        let high_char = password.chars().nth(count.high_range as usize).unwrap();

        if low_char == count.c && high_char != count.c{
            total += 1;
        }
        else if low_char != count.c && high_char == count.c{
            total += 1;
        }
    }
    total
}

struct Counter {
    c: char,
    count: u32,
    low_range: u32,
    high_range: u32,
}

fn extract_values(s: &str) -> Counter {
    //split into the first  half which needs data extracting
    let halves: Vec<&str> = s.split(':').collect();
    let mut range: Vec<&str> = halves[0].split('-').collect();
    //get last char in array
    let c: char= range[1].chars().last().unwrap();
    //cut off last 2 chars (should be letter and a space)
    range[1]= &range[1][..range[1].len()-2];
    //parse ranges
    let low_range = range[0].parse().unwrap();
    let high_range = range[1].parse().unwrap();
    //return struct for counting
    Counter {
        c,
        count: 0,
        low_range,
        high_range,
    }
}
