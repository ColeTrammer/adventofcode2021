use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::vec::Vec;

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    a: Point,
    b: Point,
}

struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn new(row: i32, col: i32) -> Grid {
        let mut grid: Vec<Vec<i32>> = Vec::new();
        for _ in 0..row {
            let mut r: Vec<i32> = Vec::new();
            for _ in 0..col {
                r.push(0);
            }
            grid.push(r);
        }
        return Grid { grid };
    }

    fn mark(&mut self, line: &Line) {
        let Line { a, b } = line;

        let mut x = a.x;
        let mut y = a.y;
        loop {
            self.grid[y as usize][x as usize] += 1;
            if x == b.x && y == b.y {
                return;
            }
            if x - b.x < 0 {
                x += 1;
            } else if x - b.x > 0 {
                x -= 1;
            }
            if y - b.y < 0 {
                y += 1;
            } else if y - b.y > 0 {
                y -= 1;
            }
        }
    }

    fn count_overlaps(&self) -> i32 {
        return self
            .grid
            .iter()
            .map(|row| row.iter().filter(|x| **x >= 2).count())
            .sum::<usize>() as i32;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify path!");
        exit(1);
    }

    let file = File::open(&args[1]).unwrap();

    let mut lines: Vec<Line> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let parts = line.replace(" -> ", ",");
        let nums: Vec<i32> = parts
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        lines.push(Line {
            a: Point {
                x: nums[0],
                y: nums[1],
            },
            b: Point {
                x: nums[2],
                y: nums[3],
            },
        });
    }

    let rows = lines.iter().map(|l| max(l.a.y, l.b.y)).max().unwrap();
    let cols = lines.iter().map(|l| max(l.a.x, l.b.x)).max().unwrap();
    let mut grid = Grid::new(rows + 1, cols + 1);

    for line in &lines {
        if line.a.x == line.b.x || line.a.y == line.b.y {
            grid.mark(&line);
        }
    }

    println!("Part 1: {}", grid.count_overlaps());

    for line in &lines {
        if line.a.x != line.b.x && line.a.y != line.b.y {
            grid.mark(&line);
        }
    }

    println!("Part 2: {}", grid.count_overlaps());
}
