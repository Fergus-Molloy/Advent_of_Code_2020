use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("unable to read file");
    let mut seats: [[char; 92]; 90] = [['.'; 92]; 90];
    let mut line = 0;
    let mut col = 0;
    for x in contents.lines() {
        for y in x.chars() {
            seats[line][col] = y;
            col += 1;
        }
        col = 0;
        line += 1;
    }
    println!("Part One: {}", part_one(&mut seats));
}

pub fn part_one(seats: &mut [[char; 92]; 90]) -> u32 {
    let mut change = true;
    let mut changes: HashMap<(usize, usize), u32> = HashMap::new();
    while change {
        change = false;
        for x in 0..seats.len() {
            for y in 0..seats[0].len() {
                let mut occ = 0;
                if seats[x][y] == '.' {
                    continue;
                }
                if x > 0 {
                    //top 3
                    if seats[x - 1][y] == '#' {
                        occ += 1;
                    }
                    if y > 0 {
                        if seats[x - 1][y - 1] == '#' {
                            occ += 1;
                        }
                    }
                    if y < seats[0].len() - 1 {
                        if seats[x - 1][y + 1] == '#' {
                            occ += 1;
                        }
                    }
                }
                if y > 0 {
                    //left one
                    if seats[x][y - 1] == '#' {
                        occ += 1;
                    }
                }
                if x < seats.len() - 1 {
                    // bottom 3
                    if seats[x + 1][y] == '#' {
                        occ += 1;
                    }
                    if y > 0 {
                        if seats[x + 1][y - 1] == '#' {
                            occ += 1;
                        }
                    }
                    if y < seats[0].len() - 1 {
                        if seats[x + 1][y + 1] == '#' {
                            occ += 1;
                        }
                    }
                }
                if y < seats[0].len() - 1 {
                    if seats[x][y + 1] == '#' {
                        occ += 1;
                    }
                }
                changes.insert((x, y), occ);
            }
        }
        for ((x, y), occ) in changes.iter() {
            if occ == &0 && seats[*x][*y] == 'L' {
                change = true;
                seats[*x][*y] = '#';
            }
            if occ >= &4 && seats[*x][*y] == '#' {
                change = true;
                seats[*x][*y] = 'L';
            }
        }
    }
    let mut count = 0;
    for x in seats.iter() {
        for y in x.iter() {
            if *y == '#' {
                count += 1;
            }
        }
    }
    count
}
