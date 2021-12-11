use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_input("test.txt");
    assert_eq!(195, calc(lines));

    let lines = load_input("input.txt");
    println!("steps: {}", calc(lines));

    Ok(())
}

fn calc(mut octopi: Vec<Octopus>) -> u64 {
    for step in 1..u64::MAX {
        let mut flashes = 0;
        println!("<<<<<<<<<<<<<<<<<<<< step {}", step);
        let mut todo = (0..octopi.len()).collect::<Vec<_>>();
        while !todo.is_empty() {
            todo = todo.iter().cloned()
                .flat_map(| idx| {
                    let to_check = octopi[idx].increment(step);
                    if !to_check.is_empty() { flashes += 1 }
                    to_check
                }).collect();
            println!("len: {} flashes: {}", todo.len(), flashes);
        }
        println!("step: {} flashes: {}", step, flashes);
        if flashes == octopi.len() {
            return step
        }
    }
    0
}

#[derive(Default, Clone)]
struct Octopus {
    neighbours: Vec<usize>,
    level: u8,
    last_flashed_at: u64,
    x: u8,
    y: u8,
}

impl Octopus {
    fn new() -> Self { Self { ..Default::default() } }

    fn increment(&mut self, step: u64) -> Vec<usize> {
        if self.last_flashed_at == step {
            //println!("done @ {},{} ({})",  self.x, self.y, self.last_flashed_at);
            vec![]
        }else{
            self.level += 1;
            //println!("inc @ {},{} ({})",  self.x, self.y, self.level);
            if self.level == 10 {
                println!("flash @ {},{} ({})",  self.x, self.y, self.last_flashed_at);
                self.level = 0;
                self.last_flashed_at = step;
                self.neighbours.clone()
            } else {
                vec![]
            }
        }
    }
}

fn load_input(name: &str) -> Vec<Octopus> {
    let file = File::open(Path::new(name)).unwrap();
    let levels = BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let max_y = levels.len();
    let max_x = levels[0].len();
    let mut octopi: Vec<Octopus> = vec![Octopus::new(); max_y * max_x];

    for (y, row) in levels.iter().enumerate() {
        for (x, level) in row.iter().enumerate() {
            let (x, y, max_x, max_y) = (x as isize, y as isize, max_x as isize, max_y as isize);
            let positions = &[(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
            let others = positions.iter().filter_map(|(x, y)| {
                if *x >= 0 && *y >= 0 && *x < max_x && *y < max_y {
                    Some((x * max_x + y) as usize)
                } else {
                    None
                }
            }).collect::<Vec<_>>();
            let mut o = &mut octopi[(x * max_x + y) as usize];
            o.level = *level;
            o.neighbours = others;
            o.x = x as u8;
            o.y = y as u8;
        }
    }

    octopi
}

