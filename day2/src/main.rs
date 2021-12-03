use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;

fn part1() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let path = &args[1];
    let file = File::open(path).unwrap();

    let mut depth = 0;
    let mut distance = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let arg: i32 = parts[1].parse().unwrap();

        if parts[0].eq("forward") {
            distance = distance + arg;
        } else if parts[0].eq("down") {
            depth = depth + arg;
        } else if parts[0].eq("up") {
            depth = depth - arg;
        }
    }

    println!("Part 1: {}", depth * distance);
}

fn part2() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let path = &args[1];
    let file = File::open(path)?;

    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let command = parts[0];
        let arg: i32 = parts[1].parse()?;

        if command == "forward" {
            distance = distance + arg;
            depth = depth + aim * arg;
        } else if command == "down" {
            aim = aim + arg;
        } else if command == "up" {
            aim = aim - arg;
        }
    }

    println!("Part 2: {}", depth * distance);
    return Ok();
}

fn main() {
    part1();
    part2().unwrap();
}
