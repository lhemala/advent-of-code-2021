use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let increments = count_increments(load_input());

    println!("part one: {}", increments);
}

fn part_two() {
    let lines = load_input();
    let windows: Vec<i64> = lines.iter().zip(&lines[1..lines.len()])
        .zip(&lines[2..lines.len()])
        .map(|((first, second), third)| first + second + third)
        .collect();
    let increments = count_increments(windows);

    println!("part two: {}", increments);
}

fn load_input() -> Vec<i64> {
    let file = File::open(Path::new("input1.txt")).unwrap();
    return io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();
}

fn count_increments(input: Vec<i64>) -> i64 {
    return input.iter().zip(&input[1..input.len()])
        .map(|(first, next)| if next > first { 1 } else { 0 })
        .sum();
}
