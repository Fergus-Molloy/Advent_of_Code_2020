use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("unable to read file");
    let lines: Vec<&str> = contents.lines().collect();
    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

pub fn part_one(lines: &Vec<&str>) -> u32 {
    let mut big =0;
    for line in lines.iter() {
        let mut row: u8 = 127;
        let mut iter = line.chars();
        for x in (0..7).rev() {
            if iter.next().unwrap() == 'F'{
                row &= 127 - (2 as u8).pow(x);
            }
        }
        let mut col: u8 = 7;
        for x in (0..3).rev() {
            if iter.next().unwrap() == 'L' {
                col &= 7 - (2 as u8).pow(x);
            }
        }
        let ID:u32 = (row as u32 *8)+col as u32;
        if ID > big {
            big = ID;
        }
    }
    big
}

pub fn part_two(lines: &Vec<&str>) -> u32 {
    let mut plane: [[bool; 8];128] = [[false;8];128];
    for line in lines.iter() {
        let mut row: usize = 127;
        let mut iter = line.chars();
        for x in (0..7).rev() {
            if iter.next().unwrap() == 'F'{
                row &= 127 - (2 as usize).pow(x);
            }
        }
        let mut col: usize = 7;
        for x in (0..3).rev() {
            if iter.next().unwrap() == 'L' {
                col &= 7 - (2 as usize).pow(x);
            }
        }
        plane[row][col]= true;
    }

    //for finding out how many rows to ignore
    //for x in plane.iter() {
    //    println!("{:?}", x)
    //}

    // should have a map of filled seats i know mine 
    // is in the middle somewhere
    for x in 4..120 { //can't be in first or last row
        for y in 0..7 {
            if !plane[x][y]{
                return ((x*8)+y) as u32;
            }
        }
    }


    
    0
}
