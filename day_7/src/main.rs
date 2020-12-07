use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

fn main() {
    let contents = fs::read_to_string("input").expect("unable to read file");
    let lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    println!("part one: {}", part_one(&lines));
    println!("part two: {}", part_two(&lines));
}

fn parse(lines: &Vec<String>) -> HashMap<String, Vec<(u32, String)>> {
    let mut map: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    for s in lines.iter() {
        //first two words is key
        let words: Vec<String> = s
            .split(' ')
            .map(|x| x.trim_matches(|c| c == ',' || c == '.').to_string())
            .collect();
        let bag = format!("{} {}", words[0], words[1]);

        // get possible inner bags
        let to_parse: Vec<String> = words.into_iter().skip(4).collect();
        let mut contain: Vec<(u32, String)> = Vec::new();
        for x in (0..to_parse.len()).step_by(4) {
            if to_parse[x] == "no" {
                continue; // no other bags go inside
            }
            let temp = format!("{} {}", to_parse[x + 1], to_parse[x + 2]);
            contain.push((to_parse[x].parse::<u32>().unwrap(), temp));
        }
        map.insert(bag, contain);
    }
    map
}

fn part_one(lines: &Vec<String>) -> u32 {
    let map: HashMap<String, Vec<(u32, String)>> = parse(lines);
    let mut total = 0;
    // cloneing references is much faster
    let map_ref = Rc::new(map);
    for key in map_ref.keys() {
        if golden(key, &map_ref) {
            total += 1
        }
    }
    total
}

fn part_two(lines: &Vec<String>) -> u32 {
    let map: HashMap<String, Vec<(u32, String)>> = parse(lines);
    let map_ref = map;
    count(&"shiny gold".to_string(), &map_ref) - 1
}

fn count(key: &String, map: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    // if empty count 1 bag
    if map[key].len() == 0 {
        return 1;
    } else {
        let sum: u32 = map[key].iter().map(|(x, k)| x * count(k, &map)).sum();
        return sum + 1;
    }
}

fn golden(key: &String, map: &HashMap<String, Vec<(u32, String)>>) -> bool {
    // extract strings
    let mut strings = Vec::new();
    for x in map[key].iter() {
        let (_, string) = x;
        strings.push(string);
    }

    if strings.contains(&&"shiny gold".to_string()) {
        return true;
    } else if strings.len() == 0 {
        return false;
    } else {
        return strings.iter().any(|x| golden(x, &map));
    }
}
