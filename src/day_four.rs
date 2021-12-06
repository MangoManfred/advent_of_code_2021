pub fn solve(_alternative: bool) {
    one();
    two();
}

use std::collections::HashMap;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

pub fn two() {
    let (nums, boards) = include_str!("../inputs/day_four/two.txt")
        .split_once("\n\n")
        .unwrap();

    let mut boards: Vec<(HashMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (board, mark, num) = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .filter_map(|n| {
            boards
                .drain_filter(|(b, m)| {
                    b.get(&n)
                        .map(|i| *m |= 1 << *i)
                        .map(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                        .unwrap_or(false)
                })
                .map(|(b, m)| (b, m, n))
                .next()
        })
        .last()
        .unwrap();

    println!(
        "{}",
        board
            .into_iter()
            .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * num as u32)
            .sum::<u32>()
    );
}

fn one() {
    let (nums, boards) = include_str!("../inputs/day_four/one_test.txt")
        .split_once("\n\n")
        .unwrap();

    let mut boards: Vec<Vec<Option<u8>>> = boards
        .split("\n\n")
        .map(|b| b.split_whitespace().map(|n| n.parse().ok()).collect())
        .collect();

    let (board, num) = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .find_map(|num| {
            boards.iter_mut().find_map(|board| {
                board.iter_mut().for_each(|c| *c = c.filter(|v| *v != num));
                ((0..5).any(|r| board[r * 5..][..5].iter().all(Option::is_none))
                    || (0..5).any(|c| board.iter().skip(c).step_by(5).all(Option::is_none)))
                .then(|| (board.clone(), num))
            })
        })
        .unwrap();

    println!(
        "{}",
        board.iter().map(|n| n.unwrap_or(0) as u32).sum::<u32>() * num as u32,
    );
}
