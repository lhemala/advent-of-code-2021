use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_input("test.txt");
    assert_eq!(1134, calculate_risk(lines));

    let lines = load_input("input.txt");
    println!("risk: {}", calculate_risk(lines));

    Ok(())
}

fn calculate_risk(rows: Vec<Vec<u8>>) -> i64 {
    let max_y = rows.len() - 1;
    let max_x = rows.first().unwrap().len() - 1;
    let mut basin_elements = vec![vec![false; rows.first().unwrap().len()]; rows.len()];
    let mut basins = rows.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(|(x, cell)| {
            let above = x > 0 && rows[y][x - 1] <= *cell;
            let below = x < max_x && rows[y][x + 1] <= *cell;
            let left = y > 0 && rows[y - 1][x] <= *cell;
            let right = y < max_y && rows[y + 1][x] <= *cell;
            if !(above || below || left || right) {
                Some((x, y))
            } else {
                None
            }
        }).map(|(x, y)| {
            expand_basin(x, y, &rows, &mut basin_elements)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    basins.sort();
    println!("basins: {:?}", basins);
    basins.iter().rev().take(3).cloned().reduce(|a, b| a * b).unwrap()
}

fn expand_basin(x: usize, y: usize, rows: &Vec<Vec<u8>>, basin_elements: &mut Vec<Vec<bool>>) -> i64 {
    let max_y = rows.len() - 1;
    let max_x = rows.first().unwrap().len() - 1;

    if rows[y][x] == 9 {
        return 0;
    }
    if basin_elements[y][x] {
        return 0;
    }

    basin_elements[y][x] = true;
    let mut sum: i64 = 1;
    if x > 0 { sum += expand_basin(x - 1, y, rows, basin_elements) }
    if x < max_x { sum += expand_basin(x + 1, y, rows, basin_elements) }
    if y > 0 { sum += expand_basin(x, y - 1, rows, basin_elements) }
    if y < max_y { sum += expand_basin(x, y + 1, rows, basin_elements) }
    sum
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

