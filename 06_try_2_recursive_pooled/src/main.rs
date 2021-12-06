use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

/// this only is already okayish for 256 rounds but takes ~1.5h with 12 threads
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open(Path::new("input.txt"))?.read_to_string(&mut input)?;

    let pool = ThreadPool::new(12);

    let data = input.split(",")
        .filter(|x| x.is_ascii())
        .map(|x| x.parse::<i16>().unwrap())
        .collect::<Vec<_>>();

    let x = data.len();
    let (tx, rx) = channel();
    let mut cnt = 0;
    for initial in data {
        let tx = tx.clone();
        let cur = cnt;
        cnt += 1;
        pool.execute(move || {
            println!("starting {}/{}", cur, x);
            tx.send(calculate(256 - initial - 1));
            println!("finished {}/{}", cur, x);
        });
    }

    let sum: i64 = rx.iter().take(x).sum();

    println!("fish count {}", sum);
    Ok(())
}

fn calculate(rounds_left: i16) -> i64 {
    let mut rounds_left = rounds_left;
    let mut sum = 1;
    while rounds_left >= 0 {
        sum += calculate(rounds_left - 9);
        rounds_left -= 7;
    }
    sum
}
