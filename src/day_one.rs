use std::fs;

fn _read_file_alternative(input_file: &str) -> Vec<usize> {
    fs::read_to_string(input_file)
        .expect("cant read file")
        .lines()
        .flat_map(|s| s.parse())
        .collect()
}

fn _question_one_altervative(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn _question_two_alternative(input: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..input.len() - 3 {
        if input[i + 3] > input[i] {
            count = count + 1;
        }
    }

    count
}

fn read_file(input_file: &str) -> Vec<i32> {
    fs::read_to_string(input_file)
        .expect("cant open file")
        .split_whitespace()
        .map(|number| number.parse().expect("Cant convert String to number"))
        .collect()
}

fn question_one(input_file: &str) -> i32 {
    let numbers = read_file(input_file);

    let mut increase_count = 0;
    let mut previous = numbers[0];

    for number in numbers {
        if number > previous {
            increase_count = increase_count + 1;
        }

        previous = number;
    }

    increase_count
}

fn question_two(input_file: &str) -> i32 {
    let numbers = read_file(input_file);

    let mut increase_count = 0;
    let mut previous = numbers[0] + numbers[1] + numbers[2];

    let mut i = 0;

    while i + 2 < numbers.len() {
        let current = numbers[i] + numbers[i + 1] + numbers[i + 2];

        if current > previous {
            increase_count = increase_count + 1;
        }

        previous = current;
        i = i + 1;
    }

    increase_count
}

pub fn solve() {
    println!("  - Day One: ");
    println!(
        "    - Question One: {}",
        question_one("inputs/day_one/day_one.txt")
    );
    println!(
        "    - Question Two: {}",
        question_two("inputs/day_one/day_one_two.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_one() {
        assert_eq!(6, question_one("inputs/day_one/day_one_test.txt"));
        assert_eq!(1, question_two("inputs/day_one/day_one_two_test.txt"));
    }
}
