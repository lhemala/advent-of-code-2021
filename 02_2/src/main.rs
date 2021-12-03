use std::fs::File;
use std::{fmt, io};
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut sub = Submarine::new();
    let commands = load_input();

    commands.iter().for_each(|cmd| sub.execute(cmd));

    println!("{}", sub);
}

fn load_input() -> Vec<Command> {
    let file = File::open(Path::new("input.txt")).unwrap();
    return io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<Command>().unwrap())
        .collect();
}

struct Submarine {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Display for Submarine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(h: {}, z: {}, a: {}, h*z: {}) ", self.horizontal, self.depth, self.aim, self.horizontal * self.depth)
    }
}

impl Submarine {
    fn new() -> Submarine {
        Submarine { horizontal: 0, depth: 0, aim: 0 }
    }

    fn execute(&mut self, cmd: &Command) {
        match cmd.dir {
            Direction::FORWARD => {
                self.horizontal += cmd.distance;
                self.depth += self.aim * cmd.distance;
            }
            Direction::UP => self.aim -= cmd.distance,
            Direction::DOWN => self.aim += cmd.distance,
        }
    }
}

enum Direction {
    FORWARD,
    UP,
    DOWN,
}

struct Command {
    dir: Direction,
    distance: i64,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(Direction::FORWARD),
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
            _ => Err(fmt::format(format_args!("failed to parse '{}' as Direction", input))),
        }
    }
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let element: Vec<&str> = input.split(" ").collect();
        Ok(Command {
            dir: element[0].parse::<Direction>().unwrap(),
            distance: element[1].parse::<i64>().unwrap(),
        })
    }
}
