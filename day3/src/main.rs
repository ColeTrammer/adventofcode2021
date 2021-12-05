use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::vec::Vec;

fn part1(lines: &Vec<String>) {
    let bits = lines[0].len();
    let mut gamma = 0;

    for bit in 0..bits {
        let mut count = 0;
        for line in lines {
            if line.as_bytes()[bit] == "1".as_bytes()[0] {
                count += 1;
            }
        }

        if count >= lines.len() / 2 {
            gamma |= 1 << (bits - 1 - bit);
        }
    }

    let epsilon = !gamma & (u32::MAX >> (32 - bits));

    println!("Part 1: {}", gamma * epsilon);
}

fn part2(lines: &Vec<String>) {
    let mut o2_rating_search = lines.clone();
    let mut co2_rating_search = lines.clone();

    let mut o2_rating = 0;
    let mut co2_rating = 0;

    let bits = lines[0].len();
    for bit in 0..bits {
        let mut count = 0;
        for line in &o2_rating_search {
            if line.as_bytes()[bit] == "1".as_bytes()[0] {
                count += 1;
            }
        }

        let keep_ones = count >= (o2_rating_search.len() + 1) / 2;
        o2_rating_search.retain(|s| keep_ones == (s.as_bytes()[bit] == "1".as_bytes()[0]));

        if o2_rating_search.len() == 1 {
            o2_rating = i32::from_str_radix(&o2_rating_search[0], 2).unwrap();
            break;
        }
    }

    for bit in 0..bits {
        let mut count = 0;
        for line in &co2_rating_search {
            if line.as_bytes()[bit] == "1".as_bytes()[0] {
                count += 1;
            }
        }

        let keep_ones = count < (co2_rating_search.len() + 1) / 2;
        co2_rating_search.retain(|s| keep_ones == (s.as_bytes()[bit] == "1".as_bytes()[0]));

        if co2_rating_search.len() == 1 {
            co2_rating = i32::from_str_radix(&co2_rating_search[0], 2).unwrap();
            break;
        }
    }

    println!("Part 2: {}", o2_rating * co2_rating);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let file = File::open(&args[1]).unwrap();

    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    part1(&lines);
    part2(&lines);
}
