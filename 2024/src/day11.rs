use itertools::Itertools;
use std::collections::HashMap;

fn part1(input: &str) -> impl std::fmt::Display {
    let mut stones = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .counts();

    for _ in 0..25 {
        let mut new_stones = HashMap::new();
        for (s, c) in stones {
            let digs = (s * 10).checked_ilog10().unwrap_or(0);

            if s == 0 {
                let e = new_stones.entry(1).or_insert(0);
                *e += c;
            } else if digs % 2 == 0 {
                let d = 10usize.pow(digs / 2);
                let lower = s % d;
                let upper = s / d;

                let e = new_stones.entry(lower).or_insert(0);
                *e += c;
                let e = new_stones.entry(upper).or_insert(0);
                *e += c;
            } else {
                let e = new_stones.entry(s * 2024).or_insert(0);
                *e += c;
            }
        }
        stones = new_stones;
    }

    stones.into_values().sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut stones = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .counts();

    for _ in 0..75 {
        let mut new_stones = HashMap::new();
        for (s, c) in stones {
            let digs = (s * 10).checked_ilog10().unwrap_or(0);

            if s == 0 {
                let e = new_stones.entry(1).or_insert(0);
                *e += c;
            } else if digs % 2 == 0 {
                let d = 10usize.pow(digs / 2);
                let lower = s % d;
                let upper = s / d;

                let e = new_stones.entry(lower).or_insert(0);
                *e += c;
                let e = new_stones.entry(upper).or_insert(0);
                *e += c;
            } else {
                let e = new_stones.entry(s * 2024).or_insert(0);
                *e += c;
            }
        }
        stones = new_stones;
    }

    stones.into_values().sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/11.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
