use advent_of_code::solve_all;
use std::time::Instant;
fn main() {
    println!("Starting the Advent of Code:");
    let now = Instant::now();
    solve_all();
    println!("Time to solve: {}ms", now.elapsed().as_millis());
}
