use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let (cost, pos) = get_min_fuel(load_input("test.txt"));
    assert_eq!(cost, 168);
    assert_eq!(pos, 5);

    let (cost, pos) = get_min_fuel(load_input("input.txt"));
    println!("cost {} pos {}", cost, pos);
    Ok(())
}

fn get_min_fuel(data: BTreeMap<i64, i64>) -> (i64, i64) {
    let mut target_to_cost: BTreeMap<i64, i64> = BTreeMap::new();
    let min = data.iter().next().unwrap().0.clone();
    let max = data.iter().next_back().unwrap().0.clone();
    for target in min..max {
        let sum = data.iter().map(|(pos, cnt)| small_gauss((target - pos).abs()) * cnt).sum();
        target_to_cost.insert(sum, target);
    }
    let cheapest = target_to_cost.iter().next().unwrap();
    (cheapest.0.clone(), cheapest.1.clone())
}

fn small_gauss(steps: i64) -> i64 {
    (steps * (steps + 1)) / 2
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

