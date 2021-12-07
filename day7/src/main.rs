use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let mut positions: Vec<i32> = Vec::new();

    for line in BufReader::new(file).lines() {
        positions.extend(line.unwrap().split(",").map(|x| x.parse::<i32>().unwrap()));
    }

    {
        let limit = *positions.iter().max().unwrap();
        let mut lo = 0;
        let mut hi = limit;
        while lo < hi - 1 {
            let mid = (lo + hi) / 2;
            let prev = mid - 1;

            let mid_cost = positions.iter().map(|x| i32::abs(x - mid)).sum::<i32>();
            let prev_cost = positions.iter().map(|x| i32::abs(x - prev)).sum::<i32>();

            if prev_cost < mid_cost {
                hi = mid;
            } else if prev_cost >= mid_cost {
                lo = mid;
            }
        }

        let cost = positions.iter().map(|x| i32::abs(x - lo)).sum::<i32>();
        println!("Part 1: {}", cost);
    }

    {
        let limit = *positions.iter().max().unwrap();
        let mut lo = 0;
        let mut hi = limit;
        while lo < hi - 1 {
            let mid = (lo + hi) / 2;
            let prev = mid - 1;

            let mid_cost = positions
                .iter()
                .map(|x| i32::abs(x - mid))
                .map(|x| x * (x + 1) / 2)
                .sum::<i32>();
            let prev_cost = positions
                .iter()
                .map(|x| i32::abs(x - prev))
                .map(|x| x * (x + 1) / 2)
                .sum::<i32>();

            if prev_cost < mid_cost {
                hi = mid;
            } else if prev_cost >= mid_cost {
                lo = mid;
            }
        }

        let cost = positions
            .iter()
            .map(|x| i32::abs(x - lo))
            .map(|x| x * (x + 1) / 2)
            .sum::<i32>();
        println!("Part 2: {}", cost);
    }
}
