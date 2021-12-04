use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut dec = Decoder::new();
    let data = load_input();

    data.for_each(|x| dec.decode(x));
    let (gamma, epsilon) = dec.calc_rates();

    println!("g: {}, e {}, pc: {}", gamma, epsilon, gamma as i64 * epsilon as i64);
}

#[derive(Default)]
struct Decoder {
    elements: isize,
    ones: [i64; 12],
}

impl Decoder {
    fn new() -> Self { Decoder { ..Default::default() } }

    fn decode(&mut self, input: u16) {
        self.elements+=1;
        for i in 0..11 {
            if input & (1 << i) > 0 {
                self.ones[i] += 1;
            }
        }
    }

    fn calc_rates(&self) -> (u16, u16) {
        let pivot = self.elements as i64;
        let gamma = self.ones.iter()
            .enumerate()
            .fold::<u16, _>(0, |acc, (i, cnt)| {
                return if *cnt > pivot / 2 {
                    acc | 1 << i
                } else {
                    acc
                };
            });

        let epsilon = !gamma & 0xFFF;
        (gamma, epsilon)
    }
}

fn load_input() -> impl Iterator<Item=u16> {
    let file = File::open(Path::new("input.txt")).unwrap();
    return io::BufReader::new(file)
        .lines()
        .map(|l| u16::from_str_radix(&*l.unwrap(), 2).unwrap());
}
