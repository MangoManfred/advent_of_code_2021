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

pub fn solve() {
    println!("  - Day Two:");
    println![
        "    - Question One: {}",
        one(&read_input("inputs/day_two/one.txt"))
    ];

    println![
        "    - Question Two: {}",
        two(&read_input("inputs/day_two/two.txt"))
    ];
}
