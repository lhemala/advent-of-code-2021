use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = load_input("test.txt");
    assert_eq!(61229, decode_and_sum(lines));

    let lines = load_input("input.txt");
    println!("sum: {}", decode_and_sum(lines));

    Ok(())
}

fn decode_and_sum(lines: Vec<(Vec<String>, Vec<String>)>) -> i64 {
    lines.iter().map(|(all_segments, solution_segements)| {
        let decoded = decode(all_segments);
        let sum = solution_segements.iter().map(segment_as_binary).map(|digit| {
            decoded.iter().enumerate().filter_map(|(i, seg)| {
                if *seg == digit {
                    Some(i)
                } else {
                    None
                }
            }).next().unwrap() as i64
        }).rev().enumerate().map(|(i, num)| 10_i64.pow(i as u32)*num).sum::<i64>();
        sum
    }).sum::<i64>()
}

fn decode(input: &Vec<String>) -> Vec<u8> {
    let mut unknowns = input.iter().map(segment_as_binary).collect::<Vec<_>>();
    let mut nums: Vec<u8> = vec![0; 10];
    nums[1] = unknowns.iter().find(|e| e.count_ones() == 2).unwrap().clone();
    nums[4] = unknowns.iter().find(|e| e.count_ones() == 4).unwrap().clone();
    nums[7] = unknowns.iter().find(|e| e.count_ones() == 3).unwrap().clone();
    nums[8] = unknowns.iter().find(|e| e.count_ones() == 7).unwrap().clone();
    nums.iter().for_each(|n| unknowns.retain(|other| *other != *n));

    nums[9] = unknowns.iter().find(|e| (nums[4] | nums[7]) & **e == (nums[4] | nums[7])).unwrap().clone();
    nums.iter().for_each(|n| unknowns.retain(|other| *other != *n));

    nums[5] = unknowns.iter().find(|e| (nums[1] | **e) == nums[9]).unwrap().clone();
    nums.iter().for_each(|n| unknowns.retain(|other| *other != *n));

    nums[6] = unknowns.iter().find(|e| nums[1] | **e == nums[8]).unwrap().clone();
    nums.iter().for_each(|n| unknowns.retain(|other| *other != *n));

    nums[3] = unknowns.iter().find(|e| (nums[4] | **e) == nums[9]).unwrap().clone();
    nums.iter().for_each(|n| unknowns.retain(|other| *other != *n));

    nums[0] = unknowns.iter().find(|e| (nums[3] | **e) == nums[8]).unwrap().clone();
    nums.iter().for_each(|n| unknowns.retain(|other| *other != *n));

    nums[2] = unknowns.iter().next().unwrap().clone();

    nums
}

fn segment_as_binary(segment: &String) -> u8 {
    let mut num: u8 = 0;
    segment.as_bytes().iter().for_each(|letter| {
        num |= 1 << letter - b'a'
    });
    num
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

