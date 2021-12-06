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
    let mut fish_by_age: Vec<usize> = Vec::new();
    for _ in 0..9 {
        fish_by_age.push(0);
    }

    for line in BufReader::new(file).lines() {
        line.unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .for_each(|age| fish_by_age[age] += 1);
    }

    for _ in 0..80 {
        let count_will_spawn = fish_by_age[0];
        for i in 0..(fish_by_age.len() - 1) {
            fish_by_age[i] = fish_by_age[i + 1];
        }
        fish_by_age[8] = count_will_spawn;
        fish_by_age[6] += count_will_spawn;
    }
    println!("Part 1: {}", fish_by_age.iter().sum::<usize>());

    for _ in 0..(256 - 80) {
        let count_will_spawn = fish_by_age[0];
        for i in 0..(fish_by_age.len() - 1) {
            fish_by_age[i] = fish_by_age[i + 1];
        }
        fish_by_age[8] = count_will_spawn;
        fish_by_age[6] += count_will_spawn;
    }

    println!("Part 2: {}", fish_by_age.iter().sum::<usize>());
}
