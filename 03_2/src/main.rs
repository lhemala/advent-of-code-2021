use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let data = load_input();
    let mut oxy = data.clone();
    let mut co2 = data;
    for pos in (0..12).rev() {
        oxy = rating_step(oxy, pos, true);
        if oxy.len() == 1 {
            break;
        }
    }

    for pos in (0..12).rev() {
        co2 = rating_step(co2, pos, false);
        if co2.len() == 1 {
            break;
        }
    }

    println!("oxy: {} co2: {} lsr: {}", co2[0], oxy[0], co2[0] as i64 * oxy[0] as i64);
}

fn rating_step(input: Vec<u16>, pos: isize, search_most_common: bool) -> Vec<u16> {
    let acc = input.iter().fold((0, 0), frequency(pos));
    let ones_are_common = acc.0 <= acc.1;
    input.iter().filter(filter_bit(pos, ones_are_common == search_most_common)).copied().collect::<Vec<_>>()
}

fn frequency(pos: isize) -> impl FnMut((i64, i64), &u16) -> (i64, i64) {
    move |acc: (i64, i64), e: &u16| -> (i64, i64) {
        if e & 1 << pos == 0 {
            (acc.0 + 1, acc.1)
        } else {
            (acc.0, acc.1 + 1)
        }
    }
}

fn filter_bit(pos: isize, filter_for_ones: bool) -> impl FnMut(&&u16) -> bool {
    move |e: &&u16| -> bool {
        ((**e & (1 << pos)) != 0) == filter_for_ones
    }
}

fn load_input() -> Vec<u16> {
    let file = File::open(Path::new("input.txt")).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| u16::from_str_radix(&*l.unwrap(), 2).unwrap())
        .collect()
}
