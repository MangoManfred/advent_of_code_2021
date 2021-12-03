fn filter_for_small(lines: Vec<&str>, at_index: usize) -> Vec<&str>{
    if lines.len() == 1 {
      return lines
    }

    let mut ones: Vec<&str> = vec![];
    let mut zeros: Vec<&str> = vec![];
    let mut one_counter = 0;
    let mut zero_counter = 0;

    for line in lines{
      println!("Line {}", line);
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
    }else{
      ones
    }
}


fn filter_for(lines: Vec<&str>, at_index: usize) -> Vec<&str>{
    if lines.len() == 1 {
      return lines;
    }

    let mut ones: Vec<&str> = vec![];
    let mut zeros: Vec<&str> = vec![];
    let mut one_counter = 0;
    let mut zero_counter = 0;

    for line in lines{
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
    }else{
      zeros
    }
}

fn convert_result(number: Vec<&str>) -> i32{
  i32::from_str_radix(&number.iter().map(|i| i.to_string()).collect::<String>(), 2).unwrap()
}

fn two(){
    let input = include_str!("../inputs/day_three/two.txt");

    let all: Vec<&str>= input.lines().collect();
    let all_small: Vec<&str> = input.lines().collect();

    let mut result= all;
    let mut result_small = all_small;

    for i in 0..12{
      result = filter_for(result, i);
      result_small = filter_for_small(result_small, i);
    }

    let oxy = convert_result(result);
    let co2 = convert_result(result_small);

    println!("{}", oxy * co2);
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

                _ => {},
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

pub fn solve(alternative: bool) {
    println!("  - Day Two:");
    if alternative {
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
