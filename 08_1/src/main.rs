use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_input("test.txt");
    assert_eq!(26, count_unique(lines));

    let lines = load_input("input.txt");
    println!("unique: {}", count_unique(lines));

    Ok(())
}
fn count_unique(lines: Vec<(Vec<String>, Vec<String>)>) -> i64 {
    lines.iter()
        .flat_map(|(_, segments)| {
            segments.iter()
                .map(|segment| segment.len() as i64)
        })
        .map(|e| {
            match e {
                2 => 1, // 1
                4 => 1, // 4
                3 => 1, // 7
                7 => 1, // 8
                _ => 0,
            }
        }).sum()
}

fn load_input(name: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open(Path::new(name)).unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| {
            let full = l.unwrap()
                .split("|")
                .map(|x| x.split_whitespace().map(|x| String::from(x))
                    .collect::<Vec<String>>())
                .collect::<Vec<_>>();
            (full[0].clone(), full[1].clone())
        }).collect()
}

