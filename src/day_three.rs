fn filter_for_small(lines: Vec<&str>, at_index: usize) -> Vec<&str> {
    if lines.len() == 1 {
        return lines;
    }

    let mut ones: Vec<&str> = vec![];
    let mut zeros: Vec<&str> = vec![];
    let mut one_counter = 0;
    let mut zero_counter = 0;

    for line in lines {
        if line.chars().nth(at_index).unwrap() == '0' {
            zero_counter += 1;
            zeros.push(line);
        } else {
            one_counter += 1;
            ones.push(line);
        }
    }

    if zero_counter <= one_counter {
        zeros
    } else {
        ones
    }
}

fn filter_for(lines: Vec<&str>, at_index: usize) -> Vec<&str> {
    if lines.len() == 1 {
        return lines;
    }

    let mut ones: Vec<&str> = vec![];
    let mut zeros: Vec<&str> = vec![];
    let mut one_counter = 0;
    let mut zero_counter = 0;

    for line in lines {
        if line.chars().nth(at_index).unwrap() == '1' {
            one_counter += 1;
            ones.push(line);
        } else {
            zeros.push(line);
            zero_counter += 1;
        }
    }

    if one_counter >= zero_counter {
        ones
    } else {
        zeros
    }
}

fn convert_result(number: Vec<&str>) -> i32 {
    i32::from_str_radix(&number.iter().map(|i| i.to_string()).collect::<String>(), 2).unwrap()
}

fn two() {
    let input = include_str!("../inputs/day_three/two.txt");

    let all: Vec<&str> = input.lines().collect();
    let all_small: Vec<&str> = input.lines().collect();

    let mut result = all;
    let mut result_small = all_small;

    for i in 0..12 {
        result = filter_for(result, i);
        result_small = filter_for_small(result_small, i);
    }

    let oxy = convert_result(result);
    let co2 = convert_result(result_small);

    println!("    - Question Two: {}", oxy * co2);
}

fn one() {
    let input = include_str!("../inputs/day_three/one.txt");
    let mut gamma = [0; 12];
    let mut epsilon = [0; 12];

    let one_counter = input.lines().fold([0; 12], |mut counter, line| {
        for (index, char) in line.chars().enumerate() {
            match char {
                '1' => {
                    counter[index] = counter[index] + 1;
                }

                _ => {}
            }
        }

        counter
    });

    for (index, count) in one_counter.iter().enumerate() {
        if *count > 500 {
            gamma[index] = 1;
            epsilon[index] = 0;
        } else {
            gamma[index] = 0;
            epsilon[index] = 1;
        }
    }

    let gamma_number: i32 =
        i32::from_str_radix(&gamma.iter().map(|i| i.to_string()).collect::<String>(), 2).unwrap();
    let epsion_number: i32 = i32::from_str_radix(
        &epsilon.iter().map(|i| i.to_string()).collect::<String>(),
        2,
    )
    .unwrap();

    println!("    - Question One: {}", gamma_number * epsion_number);
}

fn one_alternative() {
    const WIDTH: usize = 12;
    const COUNT: usize = 1000;

    let gamma = include_str!("../inputs/day_three/one.txt")
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; WIDTH], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
        .sum::<u32>();

    println!(
        "    - Question One Alternative: {}",
        gamma * (!gamma & ((1 << WIDTH) - 1))
    );
}

fn two_alternative() {
    const WIDTH: usize = 12;

    let nums = include_str!("../inputs/day_three/two.txt")
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    println!("    - Question Two Alternative: {}", oxy * co2);
}

pub fn solve(alternative: bool) {
    println!("  - Day Two:");
    if alternative {
        one_alternative();
        two_alternative();
    } else {
        one();
        two();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two() {
        let result_small = vec!["01010"];
        let mut result = result_small;

        result = filter_for_small(result, 3);

        let co2 = convert_result(result);

        assert_eq!(10, co2);
    }
}
