use std::collections::HashSet;
use std::fmt;
use std::fs;

#[allow(non_camel_case_types)]
enum Opcode {
    acc,
    jmp,
    nop,
}

struct Instruction {
    opcode: Opcode,
    operand: i32,
}

struct Computer {
    instructions: Vec<Instruction>,
    pc: i32,
    accumulator: i32,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.opcode, self.operand)
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::acc => write!(f, "acc"),
            Opcode::jmp => write!(f, "jmp"),
            Opcode::nop => write!(f, "nop"),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("unble to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut instructions: Vec<Instruction> = Vec::new();
    for x in lines.iter() {
        let parts: Vec<&str> = x.split(' ').collect();
        let opcode = match parts[0] {
            "acc" => Opcode::acc,
            "jmp" => Opcode::jmp,
            "nop" => Opcode::nop,
            _ => panic!("unknown instruction {}", parts[0]),
        };

        instructions.push(Instruction {
            opcode,
            operand: parts[1].parse::<i32>().unwrap(),
        });
    }
    let mut c = Computer {
        instructions,
        pc: 0,
        accumulator: 0,
    };
    println!("Part One: {}", part_one(&mut c));
    println!("Part Two: {}", part_two(&mut c));
}

fn part_one(c: &mut Computer) -> i32 {
    let mut visited: HashSet<i32> = HashSet::new();
    while !visited.contains(&c.pc) {
        visited.insert(c.pc);

        let current = c.instructions.get(c.pc as usize).unwrap();
        match current.opcode {
            Opcode::acc => {
                c.accumulator += current.operand;
                c.pc += 1
            }
            Opcode::jmp => c.pc += current.operand,
            Opcode::nop => c.pc += 1, //no op
        }
    }
    c.accumulator
}

fn part_two(c: &mut Computer) -> i32 {
    let mut found = 0;
    let mut change = false;
    let mut count = 0;
    while !terminates(c) {
        println!("{}", count);
        if count > 1000 {
            panic!("done more attempts that there are instructions");
        }
        count += 1;
        c.pc = 0;
        c.accumulator = 0;
        for x in found..c.instructions.len() {
            if found != 0 && change {
                // return to previous state
                match c.instructions[found - 1].opcode {
                    Opcode::jmp => c.instructions[x].opcode = Opcode::nop,
                    Opcode::nop => {
                        if c.instructions[found - 1].operand != 0 {
                            c.instructions[found - 1].opcode = Opcode::jmp
                        }
                    }
                    _ => (), //noop
                }
            }
            change = false;
            //alter first jmp/nop we see
            match c.instructions[x].opcode {
                Opcode::jmp => {
                    found = x + 1; // skip this instr next time
                    change = true;
                    c.instructions[x].opcode = Opcode::nop;
                }
                Opcode::nop => {
                    found = x + 1;
                    change = true;
                    if c.instructions[x].operand != 0 {
                        c.instructions[x].opcode = Opcode::jmp;
                    }
                }
                _ => (),
            };
            if change {
                break;
            }
        }
    }
    println!("terminates");
    c.accumulator = 0;
    c.pc = 0;
    run(c)
}

fn terminates(c: &mut Computer) -> bool {
    let mut visited: HashSet<i32> = HashSet::new();
    while !visited.contains(&c.pc) {
        visited.insert(c.pc);

        let current = match c.instructions.get(c.pc as usize) {
            Some(v) => v,
            None => return true,
        };
        match current.opcode {
            Opcode::acc => {
                c.accumulator += current.operand;
                c.pc += 1
            }
            Opcode::jmp => c.pc += current.operand,
            Opcode::nop => c.pc += 1, //no op
        }
    }
    false
}

fn run(c: &mut Computer) -> i32 {
    while c.instructions.get(c.pc as usize).is_some() {
        let current = c.instructions.get(c.pc as usize).unwrap();

        match current.opcode {
            Opcode::acc => {
                c.accumulator += current.operand;
                c.pc += 1;
            }
            Opcode::jmp => c.pc += current.operand,
            Opcode::nop => c.pc += 1,
        };
    }
    c.accumulator
}
