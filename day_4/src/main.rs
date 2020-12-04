#![allow(dead_code)]
use std::fs;
//byr (Birth Year)
//iyr (Issue Year)
//eyr (Expiration Year)
//hgt (Height)
//hcl (Hair Color)
//ecl (Eye Color)
//pid (Passport ID)
//cid (Country ID) - Optional

fn main() {
    let content = fs::read_to_string("input").expect("unable to read file");
    println!("Part One: {}", part_one(&content));
    println!("Part Two: {}", part_two(&content));
}

pub fn part_one(s: &str) -> u32 {
    //we need to find empty lines and split on them
    //empty lines will be empty strings in this array
    let lines: Vec<&str> = s.split("\n").collect();
    let mut total = 0;
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    
    for x in lines.iter(){
        if x.len() == 0{
            //check if passport is valid
            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                total += 1;
            }
            //reset checks
            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
        }
        else {
            let fields: Vec<&str> = x.split_whitespace().collect();
            for y in fields.iter(){
                match y.get(0..3).unwrap() {
                    "byr" => byr=true,
                    "iyr" => iyr=true,
                    "eyr" => eyr=true,
                    "hgt" => hgt=true,
                    "hcl" => hcl=true,
                    "ecl" => ecl=true,
                    "pid" => pid=true,
                    _ => continue
                }
            }
        }
    }
    total
}

pub fn part_two(s: &str) -> u32 {
    //we need to find empty lines and split on them
    //empty lines will be empty strings in this array
    let lines: Vec<&str> = s.split("\n").collect();
    let mut total = 0;
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    
    for x in lines.iter(){
        if x.len() == 0{
            //check if passport is valid
            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                total += 1;
            }
            //reset checks
            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
        }
        else {
            let fields: Vec<&str> = x.split_whitespace().collect();
            for y in fields.iter(){
                match y.get(0..3).unwrap() {
                    "byr" => byr=check_byr(&y),
                    "iyr" => iyr=check_iyr(&y),
                    "eyr" => eyr=check_eyr(&y),
                    "hgt" => hgt=check_hgt(&y),
                    "hcl" => hcl=check_hcl(&y),
                    "ecl" => ecl=check_ecl(&y),
                    "pid" => pid=check_pid(&y),
                    _ => continue
                }
            }
        }
    }
    total
}
fn check_byr(s: &str) -> bool {
    let value = s.split(':').last().unwrap();
    if value.len() != 4 {
       return false
    }
    let value: u32 = value.parse().unwrap();
    value >= 1920 && value <= 2002
}
fn check_iyr(s: &str) -> bool {
    let value = s.split(':').last().unwrap();
    if value.len() != 4 {
       return false
    }
    let value: u32 = value.parse().unwrap();
    value >= 2010 && value <= 2020
}
fn check_eyr(s: &str) -> bool {
    let value = s.split(':').last().unwrap();
    if value.len() != 4 {
       return false
    }
    let value: u32 = value.parse().unwrap();
    value >= 2020 && value <= 2030
}
fn check_hgt(s: &str) -> bool {
    let value = s.split(':').last().unwrap();
    if value.ends_with("cm") {
        let num:u32 = value.get(0..(value.len()-2)).unwrap().parse().unwrap();
        num >= 150 && num <= 193
    }
    else if value.ends_with("in") {
        let num:u32 = value.get(0..(value.len()-2)).unwrap().parse().unwrap();
        num >= 59 && num <= 76
    }
    else{
        false
    }
}
fn check_hcl(s: &str) -> bool {
    let value = s.split(':').last().unwrap();
    if value.len() != 7 {
        return false
    }
    if value.chars().next().unwrap() == '#' {
        let num = value.trim_start_matches("#");
        let num = u32::from_str_radix(num, 16);
        match num {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    else {
        false
    }
}
fn check_ecl(s: &str) -> bool {
    let value = s.split(':').last().unwrap();
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}
fn check_pid(s: &str) -> bool {
    let value = s.split(':').last().unwrap();
    if value.len() != 9 {
        false
    }
    else{
        let num = value.parse::<u32>();
        match num {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
