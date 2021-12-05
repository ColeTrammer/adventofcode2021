use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

struct Board {
    squares: Vec<Vec<(i32, bool)>>,
    has_won: bool,
}

impl Board {
    fn check_win(self: &Board) -> bool {
        for r in 0..self.squares.len() {
            if self.squares[r].iter().all(|x| x.1) {
                return true;
            }
        }
        for c in 0..self.squares[0].len() {
            if self.squares.iter().map(|row| row[c]).all(|x| x.1) {
                return true;
            }
        }
        return false;
    }

    fn mark(self: &mut Board, value: i32) -> bool {
        for row in &mut self.squares {
            for i in 0..row.len() {
                row[i].1 |= row[i].0 == value;
            }
        }

        self.has_won = self.check_win();
        return self.has_won;
    }

    fn compute_score(self: &Board, value: i32) -> i32 {
        return self
            .squares
            .iter()
            .map(|row| row.iter().filter(|x| !x.1).map(|x| x.0).sum::<i32>())
            .sum::<i32>()
            * value;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let file = File::open(&args[1]).unwrap();

    let mut first_line = true;
    let mut numbers: Vec<i32> = Vec::new();
    let mut grid: Vec<Vec<(i32, bool)>> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if first_line {
            first_line = false;
            numbers.extend(line.split(",").map(|x| x.parse::<i32>().unwrap()));
            continue;
        }

        if line.is_empty() {
            continue;
        }

        grid.push(
            line.split_ascii_whitespace()
                .map(|x| (x.parse::<i32>().unwrap(), false))
                .collect(),
        );

        if grid.len() == 5 {
            boards.push(Board {
                squares: grid.clone(),
                has_won: false,
            });
            grid.clear();
        }
    }

    let total = boards.len();
    let mut count = 0;
    for n in numbers {
        for board in &mut boards {
            if board.has_won {
                continue;
            }

            if board.mark(n) {
                count += 1;
                if count == 1 {
                    println!("Part 1: {}", board.compute_score(n));
                }
                if count == total {
                    println!("Part 2: {}", board.compute_score(n));
                }
                continue;
            }
        }
    }
}
