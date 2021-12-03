
use advent_of_code::solve_all;
use std::{env, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut alternative = false;

    if args.len() > 1 {
        alternative = true;
    }

    println!("Starting the Advent of Code:");
    let now = Instant::now();
    solve_all(alternative);
    println!("Time to solve: {}ms", now.elapsed().as_millis());
}
