#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};
use std::fs::read_to_string;
use std::ops::{AddAssign, Mul};

//================================================================================================
//| TYPE DEFS                                                                                    |
//================================================================================================
#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Ship {
    location: Point,
    facing: Direction,
    waypoint: Point,
}

//================================================================================================
//| TRAITS                                                                                       |
//================================================================================================

// impl Display trait so i can print Directions and Points
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// cannot make impl blocks for primitive types
// so must be implenmented as a trait
trait Negate {
    fn neg(&self) -> Self;
}

impl Negate for i32 {
    fn neg(&self) -> Self {
        -(self)
    }
}

// for multiplying a point by an int
impl Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

// so i can use the += operator on points
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

//================================================================================================
//| IMPLS                                                                                        |
//================================================================================================

impl Point {
    // made this so initialising ships is prettier
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // for the final calculation (assumes you started at (0,0))
    pub fn manhattan_origin(p: Point) -> i32 {
        p.x.abs() + p.y.abs()
    }

    // rotates a point around the origin
    pub fn rotate(&mut self, degrees: i32) {
        // (degrees/90)%4 give the number of rotations needed
        let mut degrees = (degrees / 90) % 4;
        // change negative rotations to thier positive equivilents
        if degrees.is_negative() {
            degrees = match degrees {
                -1 => 3,
                -2 => 2,
                -3 => 1,
                _ => panic!("{} out of range", degrees), //should never run due to %4
            };
        }
        match degrees {
            1 => self.rotate_right(),
            2 => self.rotate_180(),
            3 => self.rotate_left(),
            _ => panic!("{} out of range", degrees), //should never run due to %4
        }
    }

    fn rotate_left(&mut self) {
        let tempy = self.y;
        if self.x.is_positive() {
            // east of ship -> north of ship
            self.y = self.x;
        } else {
            //west of ship -> south of ship
            self.y = self.x;
        }
        //y values changed here
        if tempy.is_positive() {
            // north of ship -> west of ship
            self.x = tempy.neg();
        } else {
            // south of ship -> east of ship
            self.x = tempy.abs();
        }
    }

    fn rotate_right(&mut self) {
        let tempy = self.y;
        if self.x.is_positive() {
            // east of ship -> south of ship
            self.y = self.x.neg();
        } else {
            //west of ship -> north of ship
            self.y = self.x.abs();
        }
        //y values changed here
        if tempy.is_positive() {
            // north of ship -> east of ship
            self.x = tempy;
        } else {
            // north of ship -> west of ship
            self.x = tempy;
        }
    }

    fn rotate_180(&mut self) {
        // flip east -> west or west -> east
        if let true = self.x.is_positive() {
            self.x = self.x.neg()
        } else {
            self.x = self.x.abs()
        }
        // flip north -> south or south -> north
        if let true = self.y.is_positive() {
            self.y = self.y.neg()
        } else {
            self.y = self.y.abs()
        }
    }
}

impl Ship {
    // can't just call it move since that's a keyword
    pub fn move_by(&mut self, x: i32, y: i32) {
        self.location.x += x;
        self.location.y += y;
    }

    pub fn move_waypoint(&mut self, x: i32, y: i32) {
        self.waypoint.x += x;
        self.waypoint.y += y;
    }

    // move this ship in the direction it's facing by amount
    pub fn forward(&mut self, amount: i32) {
        match self.facing {
            Direction::North => self.move_by(0, amount),
            Direction::East => self.move_by(amount, 0),
            Direction::South => self.move_by(0, -amount),
            Direction::West => self.move_by(-amount, 0),
        };
    }

    // move ship towards the waypoint amount times
    pub fn go_to_waypoint(&mut self, amount: i32) {
        //go to waypoint (rel distance away) amount times
        self.location += amount * self.waypoint;
    }

    // change the direction the ship is facing
    pub fn rotate(&mut self, degrees: i32) {
        //cba to add Direction in front of these
        use Direction::{East, North, South, West};

        // (degrees/90)%4 give the number of rotations needed
        let mut degrees = (degrees / 90) % 4;
        // change negative rotations to thier positive equivilents
        if degrees.is_negative() {
            degrees = match degrees {
                -1 => 3,
                -2 => 2,
                -3 => 1,
                _ => panic!("{} out of range", degrees), //should never run due to %4
            };
        }
        // calculate new direction (% 4 makes it wrap around)
        let new_dir = (self.facing as i32 + degrees) % 4;
        let dirs = [North, East, South, West];
        self.facing = dirs[new_dir as usize];
    }
}

//================================================================================================
//| TASKS                                                                                        |
//================================================================================================

fn main() {
    let content = read_to_string("input").expect("unable to read file");
    let lines: Vec<String> = content.lines().map(|x| x.to_string()).collect();
    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

pub fn part_one(lines: &Vec<String>) -> i32 {
    // create ship (waypoint is irrelevant for this part)
    let mut ship = Ship {
        location: Point::new(0, 0),
        facing: Direction::East,
        waypoint: Point::new(10, 1), //10 units east 1 unit north
    };
    for line in lines.iter() {
        let mut iter = line.chars();
        let inst = iter.next().unwrap(); // get first char (the instruction)

        // get the `operand` of the instruction
        let amount: String = iter.take(line.len() - 1).collect();
        let amount: i32 = amount.parse().unwrap();

        // execute the correct instruction
        match inst {
            'N' => ship.move_by(0, amount),
            'E' => ship.move_by(amount, 0),
            'S' => ship.move_by(0, -amount),
            'W' => ship.move_by(-amount, 0),
            'L' => ship.rotate(-amount),
            'R' => ship.rotate(amount),
            'F' => ship.forward(amount),
            _ => panic!("didn't recognise instruction {}", inst),
        }
    }
    // do final calculation
    Point::manhattan_origin(ship.location)
}

pub fn part_two(lines: &Vec<String>) -> i32 {
    // create ship (waypoint *is* relevent)
    let mut ship = Ship {
        location: Point::new(0, 0),
        facing: Direction::East,
        waypoint: Point::new(10, 1), //10 units east 1 unit north
    };

    for line in lines.iter() {
        let mut iter = line.chars();
        let inst = iter.next().unwrap(); // get first char (the instruction)

        // get the `operand` for this instruction
        let amount: String = iter.take(line.len() - 1).collect();
        let amount: i32 = amount.parse().unwrap();

        // execute the correct instructions
        match inst {
            'N' => ship.move_waypoint(0, amount),
            'E' => ship.move_waypoint(amount, 0),
            'S' => ship.move_waypoint(0, -amount),
            'W' => ship.move_waypoint(-amount, 0),
            'L' => ship.waypoint.rotate(-amount),
            'R' => ship.waypoint.rotate(amount),
            'F' => ship.go_to_waypoint(amount),
            _ => panic!("didn't recognise instruction {}", inst),
        }
    }
    // do the final calculation
    Point::manhattan_origin(ship.location)
}
