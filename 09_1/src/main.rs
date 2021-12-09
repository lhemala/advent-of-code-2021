use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_input("test.txt");
    assert_eq!(15, calculate_risk(lines));

    let lines = load_input("input.txt");
    println!("risk: {}", calculate_risk(lines));

    Ok(())
}

fn calculate_risk(rows: Vec<Vec<u8>>) -> i64 {
    let max_y = rows.len() - 1;
    let max_x = rows.first().unwrap().len() - 1;
    rows.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, cell)| {
            let above = x > 0 && rows[y][x - 1] <= *cell;
            let below = x < max_x && rows[y][x + 1] <= *cell;
            let left = y > 0 && rows[y - 1][x] <= *cell;
            let right = y < max_y && rows[y + 1][x] <= *cell;
            if !(above || below || left || right) {
                println!("a: {} b: {} l: {} r: {}", above, below, left, right);
                println!("x: {:2} y:{:2} ({}) -> {}", x, y, *cell, *cell + 1);
                *cell as i64 + 1
            } else {
                0
            }
        }).sum::<i64>()
    }).sum::<i64>()
}

fn load_input(name: &str) -> Vec<Vec<u8>> {
    let file = File::open(Path::new(name)).unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| {
            let x = l.unwrap();
            println!("{}", x);
            x.chars()
                .map(|n| n.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        }).collect()
}

