use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_input("test.txt");
    assert_eq!(288957, calc(lines));

    let lines = load_input("input.txt");
    println!("points: {}", calc(lines));

    Ok(())
}

fn is_opening(char: u8) -> bool {
    match char {
        b'(' => true,
        b'[' => true,
        b'{' => true,
        b'<' => true,
        _ => false,
    }
}

fn get_matching(char: u8) -> u8 {
    match char {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => panic!("no matching found")
    }
}

fn points(char: u8) -> i64 {
    match char {
        b'(' => 1,
        b'[' => 2,
        b'{' => 3,
        b'<' => 4,
        _ => panic!("char not found")
    }
}

fn calc(input: Vec<Vec<u8>>) -> i64 {
    let mut stack: Vec<u8> = vec![0; 100];

    let mut points = input.iter().filter_map(|line| {
        stack.clear();
        let bracket = line.iter().find(|char| {
            if is_opening(**char) {
                stack.push(**char);
                false
            } else {
                let expected = stack.pop().unwrap();
                expected != get_matching(**char)
            }
        });
        if let None = bracket {
            println!("stack: {:?}", stack);
            Some(stack.iter().rev().fold(0, |acc, char| acc * 5 + points(*char)))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    points.sort();
    println!("points: {:?}", points);
    points[points.len() / 2]
}

fn load_input(name: &str) -> Vec<Vec<u8>> {
    let file = File::open(Path::new(name)).unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .as_bytes()
                .to_vec()
        }).collect()
}

