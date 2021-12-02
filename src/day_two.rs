use std::fs;

fn read_input(input: &str) -> Vec<(String, usize)> {
    fs::read_to_string(input)
        .expect("cant open file")
        .lines()
        .map(|line| {
            let mut elements = line.split_whitespace();
            (
                elements.next().unwrap().to_string(),
                elements.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn one(pairs: &[(String, usize)]) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;

    for (location, distance) in pairs {
        let temp = &location[..];
        match temp {
            "forward" => {
                horizontal = horizontal + distance;
            }
            "down" => {
                depth = depth + distance;
            }
            "up" => {
                depth = depth - distance;
            }

            _ => {}
        }
    }

    depth * horizontal
}

fn two(pairs: &[(String, usize)]) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for (location, distance) in pairs {
        match &location[..] {
            "forward" => {
                horizontal = horizontal + distance;
                depth = depth + aim * distance;
            }
            "down" => {
                aim = aim + distance;
            }
            "up" => {
                aim = aim - distance;
            }

            _ => {}
        }
    }

    depth * horizontal
}

pub fn one_alternative() {
    let (f, d) = include_str!("../inputs/day_two/one.txt")
        .lines()
        .flat_map(|l| l.split_once(" "))
        .fold((0, 0), |(f, d), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d),
                ("down", v) => (f, d + v),
                ("up", v) => (f, d - v),
                _ => unreachable!(),
            }
        });

    println!("    - Question Two Alternative: {}", f * d);
}

pub fn two_alternative() {
    let (f, d, _) = include_str!("../inputs/day_two/one.txt")
        .lines()
        .flat_map(|l| l.split_once(" "))
        .fold((0, 0, 0), |(f, d, a), (k, v)| {
            match (k, v.parse::<i32>().unwrap()) {
                ("forward", v) => (f + v, d + a * v, a),
                ("down", v) => (f, d, a + v),
                ("up", v) => (f, d, a - v),
                _ => unreachable!(),
            }
        });

    println!("    - Question Two Alternative: {}", f * d);
}

pub fn solve(alternative: bool) {
    println!("  - Day Two:");
    if alternative {
        one_alternative();
        two_alternative();
    } else {
        println![
            "    - Question One: {}",
            one(&read_input("inputs/day_two/one.txt"))
        ];

        println![
            "    - Question Two: {}",
            two(&read_input("inputs/day_two/two.txt"))
        ];
    }
}
