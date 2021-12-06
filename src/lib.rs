#![feature(drain_filter)]

mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;

pub fn solve_all(alternative: bool) {
    println!("Answers:");
    day_one::solve(alternative);
    day_two::solve(alternative);
    day_three::solve(alternative);
    day_four::solve(alternative);
    day_five::solve(alternative);
}
