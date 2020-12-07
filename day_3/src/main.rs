use std::fs;

//               X   ,  Y
struct Toboggan(usize, usize);

pub fn get_trees(lines: &Vec<&str>, v_x: usize, v_y: usize) -> u32 {
    let mut tob = Toboggan(0, 0);
    let mut trees = 0;

    while tob.1 <= (lines.len() - 1) {
        // get y coord
        let line = lines.iter().nth(tob.1).unwrap();
        // since patttern is repeating can use modulo to get index
        let x = tob.0 % line.len();

        if line.chars().nth(x).unwrap() == '#' {
            trees += 1;
        }

        tob.0 += v_x;
        tob.1 += v_y;
    }
    trees
}

#[allow(dead_code)]
fn main() {
    let content = fs::read_to_string("input").expect("unable to read file");
    let lines: Vec<&str> = content.lines().map(|x| x.trim()).collect();

    //part one
    println!("trees encontered: {}", get_trees(&lines, 3, 1));
    //part two
    let result = get_trees(&lines, 1, 1)
        * get_trees(&lines, 3, 1)
        * get_trees(&lines, 5, 1)
        * get_trees(&lines, 7, 1)
        * get_trees(&lines, 1, 2);
    println!("result: {}", result);
}
