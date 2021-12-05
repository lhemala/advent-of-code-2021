use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let (ref random_numbers, mut boards) = load_input();
    let sum = random_numbers.iter()
        .map(|&num| {
            boards.iter_mut().filter(|b| b.winning_sum.is_none())
                .map(|b| {
                    b.mark(num);
                    b
                }).filter(|b| b.winning_sum.is_some())
                .map(|z| z.clone())
                .collect::<Vec<_>>()
        }).filter(|x| !x.is_empty())
        .flatten()
        .last()
        .unwrap();

    println!("{:?}", random_numbers);
    println!();
    for b in boards.iter() {
        println!("{}", b);
    }

    print!("{}", sum.winning_sum.unwrap());
}

#[derive(Clone)]
struct Board {
    rows_and_columns: Vec<Vec<(i64, bool)>>,
    winning_sum: Option<i64>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows_and_columns.as_slice() {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl Board {
    fn new(data: Vec<Vec<(i64, bool)>>) -> Self {
        Board { rows_and_columns: data, winning_sum: None }
    }

    fn mark(&mut self, new_num: i64) {
        for row in 0..self.rows_and_columns.len() {
            for col in 0..self.rows_and_columns[row].len() {
                let (num, ref mut marked) = self.rows_and_columns[row][col];
                if
                *marked { continue; }

                if num
                    == new_num && !*marked {
                    *marked = true;
                    if let Some(sum) = self.check(row, col) {
                        self.winning_sum = Some(sum * new_num)
                    }
                }
            }
        }
        ;
    }

    fn check(&self, row_num: usize, col_num: usize) -> Option<i64> {
        if self
            .rows_and_columns[row_num].iter().all(|(_, marked)| *marked) {
            Some(self.sum_unmarked())
        } else if self
            .get_column(col_num).all(|(_, marked)| marked) {
            Some(self.sum_unmarked())
        } else {
            None
        }
    }

    fn get_column(&self, col_num
    : usize)
                  -> impl Iterator<Item=(i64, bool)> + '_ {
        self.rows_and_columns.iter()
            .map(move |row|
                row[col_num])
    }

    fn sum_unmarked(&self) -> i64 {
        self.rows_and_columns.iter()
            .flatten()
            .filter(|(_, marked)| !*marked)
            .map(|(num, _)| num)
            .sum()
    }
}

fn load_input() -> (Vec<i64>, Vec<Board>) {
    let file = File::open(Path::new("input.txt")).unwrap();
    let mut lines = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty());
    let random_numbers: Vec<i64> = lines.nth(0)
        .unwrap()
        .split(",")
        .map(|e| e.parse().unwrap())
        .collect();

    let boards: Vec<Board> = lines.collect::<Vec<_>>()
        .chunks_exact(5)
        .map(|raw_board| raw_board.into_iter()
            .map(|raw_line| raw_line.split_whitespace()
                .map(|raw_num| (raw_num.parse::<i64>().unwrap(), false))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>())
        .map(|data| Board::new(data))
        .collect();

    (random_numbers, boards)
}
