use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

/// approach works pretty well since it only computes the 6 distinct input values
/// instead of calculating all 300
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    File::open(Path::new("input.txt"))?.read_to_string(&mut input)?;

    let data = input.split(",")
        .filter(|x| x.is_ascii())
        .map(|x| x.parse::<i16>().unwrap())
        .collect::<Vec<_>>();

    let jobs = 6 as usize;
    let pool = ThreadPool::new(jobs);

    let (tx, rx) = channel();
    for initial in 0..jobs as i16 {
        let tx = tx.clone();
        pool.execute(move || {
            println!("starting {}", initial);
            tx.send((initial, calculate(256 - initial - 1)));
            println!("finished {}", initial);
        });
    }

    let mut results = rx.iter().take(jobs).collect::<Vec<_>>();
    results.sort_by_key(|(initial, _)| *initial);
    let results: Vec<_> = results.iter().map(|(_, result)| result).collect();

    let sum: i64 = data.iter().map(|initial| results[*initial as usize]).sum();

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
