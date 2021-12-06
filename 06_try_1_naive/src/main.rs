use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

/// approach works fine for 80 rounds but 256 are unbearable
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open(Path::new("input.txt"))?.read_to_string(&mut input)?;

    let mut data = input.split(",")
        .filter(|x| x.is_ascii())
        .map(|x| match x.parse::<i64>() {
            Err(err) => panic!("{}: '{}'", err, x),
            Ok(val) => val,
        })
        .collect::<Vec<_>>();

    for _ in 0..256 {
        let mut pos = 0;
        while pos < data.len() {
            data[pos] = data[pos] - 1;
            if data[pos] < 0 {
                data[pos] = 6;
                data.insert(pos + 1, 8);
                pos += 1;
            }
            pos += 1;
        }
    }

    println!("fish count {}", data.len());
    Ok(())
}
