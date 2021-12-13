use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::vec::Vec;

struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn neighbors_of(&self, coord: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::<(i32, i32)>::new();
        let (row, col) = coord;

        let mut try_add = |coord: (i32, i32)| {
            let (row, col) = coord;
            if row < 0
                || row >= self.grid.len() as i32
                || col < 0
                || col >= self.grid[0].len() as i32
            {
                return;
            }
            neighbors.push(coord);
        };

        try_add((row + 0, col + 1));
        try_add((row + 0, col - 1));
        try_add((row + 1, col));
        try_add((row - 1, col));

        return neighbors;
    }

    fn low_points(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::<(i32, i32)>::new();
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let height = self.grid[row][col];
                let mut is_lowest = true;
                for (y, x) in self.neighbors_of((row as i32, col as i32)) {
                    if self.grid[y as usize][x as usize] <= height {
                        is_lowest = false;
                    }
                }
                if is_lowest {
                    points.push((row as i32, col as i32));
                }
            }
        }
        return points;
    }

    fn sum_low_points(&self) -> i32 {
        return self
            .low_points()
            .iter()
            .map(|(row, col)| self.grid[*row as usize][*col as usize] + 1)
            .sum::<i32>();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in BufReader::new(file).lines() {
        grid.push(
            line.unwrap()
                .split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }

    let grid = Grid { grid };
    println!("Part 1: {}", grid.sum_low_points());
}
