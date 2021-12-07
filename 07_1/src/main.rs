use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(37, get_min_fuel(load_input("test.txt")));

    let fuel = get_min_fuel(load_input("input.txt"));
    println!("min fuel need {}", fuel);
    Ok(())
}

fn get_min_fuel(data: BTreeMap<i64, i64>) -> i64 {
    let mut target_to_cost: BTreeMap<i64, i64> = BTreeMap::new();
    let min = data.iter().next().unwrap().0.clone();
    let max = data.iter().next_back().unwrap().0.clone();
    for target in min..max {
        let sum = data.iter().map(|(pos, cnt)| (target - pos).abs() * cnt).sum();
        target_to_cost.insert(sum, target);
    }
    target_to_cost.iter().next().unwrap().0.clone()
}

fn load_input(name: &str) -> BTreeMap<i64, i64> {
    let file = File::open(Path::new(name)).unwrap();
    let mut data = BTreeMap::new();
    BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .split(",")
                .map(|e| e.parse::<i64>().unwrap())
                .for_each(|n| *data.entry(n).or_insert(0) += 1);
        }).for_each(|_| {});
    data
}

