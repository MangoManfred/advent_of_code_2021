mod day_one;
mod day_two;

pub fn solve_all(alternative: bool) {
    println!("Answers:");
    day_one::solve(alternative);
    day_two::solve(alternative);
}
