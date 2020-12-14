#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};
use std::fs::read_to_string;
use std::ops::{AddAssign, Mul};

struct Point {
    x: i32,
    y: i32,
}
#[derive(PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

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

struct Ship {
    location: Point,
    facing: Direction,
    waypoint: Point,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn manhattan_origin(p: Point) -> i32 {
        p.x.abs() + p.y.abs()
    }

    fn get_relative_pos(&self, p: &Point) -> Point {
        Point {
            x: p.x - self.x,
            y: p.y - self.y,
        }
    }
}

impl Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Point {
        Point {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Ship {
    fn move_by(&mut self, x: i32, y: i32) {
        self.location.x += x;
        self.location.y += y;
    }

    fn forward(&mut self, amount: i32) {
        match self.facing {
            Direction::North => self.move_by(0, amount),
            Direction::East => self.move_by(amount, 0),
            Direction::South => self.move_by(0, -amount),
            Direction::West => self.move_by(-amount, 0),
        };
    }

    fn go_to_waypoint(&mut self, amount: i32) {
        let rel = self.location.get_relative_pos(&self.waypoint);
        //go to waypoint (rel distance away) amount times
        self.location += amount * rel;
    }

    fn rotate(&mut self, degrees: i32) {
        //cba to add Direction in front of these
        use Direction::{East, North, South, West};
        // (degrees/90)%4 give the number of rotations needed
        let mut degrees = (degrees / 90) % 4;
        if degrees.is_negative() {
            degrees = match degrees {
                -1 => 3,
                -2 => 2,
                -3 => 1,
                _ => panic!("{} out of range", degrees),
            };
        }
        // calculate new direction (% 4 makes it wrap around)
        let new_dir = (self.facing as i32 + degrees) % 4;
        let dirs = [North, East, South, West];
        self.facing = dirs[new_dir as usize];
    }
}

fn main() {
    let content = read_to_string("input").expect("unable to read file");
    let lines = content.lines();
    let mut ship = Ship {
        location: Point::new(0, 0),
        facing: Direction::East,
        waypoint: Point::new(10, 1), //10 units east 1 unit north
    };
    for line in lines {
        let mut iter = line.chars();
        let inst = iter.next().unwrap();
        let amount: String = iter.take(line.len() - 1).collect();
        let amount: i32 = amount.parse().unwrap();
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
    println!(
        "manhattan distance: {}",
        Point::manhattan_origin(ship.location)
    );
}
