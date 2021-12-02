use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

fn part1(numbers: &Vec<i32>) {
    let mut count = 0;
    for i in 1..numbers.len() {
        if numbers[i - 1] < numbers[i] {
            count = count + 1;
        }
    }

    println!("Part 1: {}", count);
}

fn part2(numbers: &Vec<i32>) {
    let mut count = 0;
    let mut prev_sum = i32::max_value();
    for window in numbers.windows(3) {
        let sum = window[0] + window[1] + window[2];
        if prev_sum < sum {
            count = count + 1;
        }
        prev_sum = sum;
    }

    println!("Part 2: {}", count);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        numbers.push(line.unwrap().parse::<i32>().unwrap());
    }

    part1(&numbers);
    part2(&numbers);
}
