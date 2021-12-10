use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::Chars;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_input("test.txt");
    assert_eq!(26397, calc(lines));

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
         b')'  => b'(',
         b']'  => b'[',
         b'}'  => b'{',
         b'>'  => b'<',
        _ => panic!("no matching found")
    }
}

fn points(char: u8) -> i64 {
    match char {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("char not found")
    }
}

fn to_str(char: u8) -> String {
    String::from_utf8(vec![char]).unwrap()
}

fn calc(input: Vec<Vec<u8>>) -> i64 {
    let mut stack: Vec<u8> = vec![0; 100];

    input.iter().map(|line| {
        stack.clear();
        println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
        let bracket = line.iter().find(|char| {
            if is_opening(**char) {
                println!("push {}", to_str(**char));
                stack.push(**char);
                false
            } else {
                let expected = stack.pop().unwrap();
                println!("popped {} != {}", to_str(expected), to_str(**char));
                expected != get_matching(**char)
            }
        });
        if let Some(b) = bracket {
            println!("unmatched {}", to_str(*b));
            points(*b)
        } else {
            0
        }
    }).sum()
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

