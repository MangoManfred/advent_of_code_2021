use atoi::atoi;
use nom::*;
use std::collections::HashMap;

fn one() {
    let map: HashMap<(usize, usize), usize> = include_str!("../inputs/05/01.txt")
        .lines()
        .flat_map(|line| line.split(" -> "))
        .flat_map(|line| line.split(","))
        .map(|number| number.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(4)
        .filter(|window| window[0] == window[2] || window[1] == window[3])
        .collect::<Vec<&[usize]>>()
        .iter()
        .fold(HashMap::new(), |mut acc, window| {
            let x0 = window[0];
            let y0 = window[1];

            let x1 = window[2];
            let y1 = window[3];

            if x0 == x1 {
                (y0.min(y1)..=y0.max(y1)).for_each(|y| *acc.entry((x0, y)).or_default() += 1);
            } else if y0 == y1 {
                (x0.min(x1)..=x0.max(x1)).for_each(|x| *acc.entry((x, y0)).or_default() += 1);
            }

            acc
        });
    println!("    - Question One: {}", map.values().filter(|value| **value >= 2).count());
}

fn one_alternative() {
    let map = include_bytes!("../inputs/05/01.txt")
        .split(|b| *b == b'\n')
        .map(|entry| {
            let ((x, y), (xx, yy)) = line(entry).unwrap().1;
            (x.min(xx), y.min(yy), x.max(xx), y.max(yy))
        })
        .filter(|(x, y, xx, yy)| x == xx || y == yy)
        .fold(vec![0u8; 1000 * 1000], |mut map, (x, y, xx, yy)| {
            if x == xx {
                (y..=yy).for_each(|y| map[x + y * 1000] += 1);
            } else {
                (x..=xx).for_each(|x| map[x + y * 1000] += 1);
            }
            map
        });

    println!("    - Question One Alternative: {}", map.into_iter().filter(|c| *c >= 2).count());
}

named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi));
named!(coord<&[u8], (usize, usize)>, separated_pair!(usize, char!(','), usize));
named!(line<&[u8], ((usize, usize), (usize, usize))>, separated_pair!(coord, tag!(" -> "), coord));


pub fn solve(alternative: bool) {
  println!("  - Day Five");
  if alternative {
    one_alternative();
  } else {
    one();
  }
}