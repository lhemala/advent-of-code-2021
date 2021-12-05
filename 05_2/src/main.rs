use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{repeat};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let mut map = Map::new();
    load_input().iter()
        .for_each(|l| {
            map.draw_line(l);
        });
    println!("{}", map);
}

#[derive(Default)]
struct Map {
    rows_and_columns: Vec<Vec<i64>>,
}

impl Map {
    fn new() -> Self {
        Map { rows_and_columns: vec![vec![0]] }
    }

    fn draw_line(&mut self, line: &Line) {
        println!("drawing {}", line);

        let dx: i64 = if line.head.x - line.tail.x > 0 { 1 } else { -1 };
        let dy: i64 = if line.head.y - line.tail.y > 0 { 1 } else { -1 };

        let mut p = line.tail.clone();
        self.draw_point(&p);
        while p != line.head {
            if p.x != line.head.x {
                p.x += dx;
            }
            if p.y != line.head.y {
                p.y += dy;
            }
            self.draw_point(&p);
        }
    }

    fn draw_point(&mut self, point: &Point) {
        while self.rows_and_columns.len() <= point.y as usize {
            println!("growing rows {} -> {}", self.rows_and_columns.len(), point.y);

            self.rows_and_columns.push(repeat(0)
                .take(self.rows_and_columns[0].len())
                .collect());
        }

        while self.rows_and_columns[0].len() <= point.x as usize {
            println!("growing columns {} -> {}", self.rows_and_columns[0].len(), point.x);

            for col in 0..self.rows_and_columns.len() {
                self.rows_and_columns[col].push(0);
            }
        }

        println!("drawing {}", point);
        self.rows_and_columns[point.y as usize][point.x as usize] += 1;

        //println!("{}", self);
    }

    fn count_overlaps(&self) -> usize {
        self.rows_and_columns.iter()
            .flatten()
            .filter(|cnt| **cnt > 1)
            .count()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows_and_columns.as_slice() {
            for field in row {
                write!(f, "{:2}", field)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "overlaps {}", self.count_overlaps())?;
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({} {})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
        Ok(Point { x: parts[0], y: parts[1] })
    }
}

struct Line {
    head: Point,
    tail: Point,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({} -> {})", self.tail, self.head)
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(Line { tail: parts[0].parse()?, head: parts[2].parse()? })
    }
}

fn load_input() -> Vec<Line> {
    let file = File::open(Path::new("input.txt")).unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}
