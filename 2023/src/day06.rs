use std::fmt::Display;

use regex::Regex;

fn part1(input: String) -> impl Display {
    let lines = input.lines().collect::<Vec<_>>();
    let times = Regex::new(r"\d+")
        .unwrap()
        .captures_iter(lines[0])
        .map(|n| n[0].parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let records = Regex::new(r"\d+")
        .unwrap()
        .captures_iter(lines[1])
        .map(|n| n[0].parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    times
        .iter()
        .zip(records.iter())
        .map(|(t, r)| (1..*t).filter(|h| (*t - h) * h > *r).count())
        .product::<usize>()
}

fn part2(input: String) -> impl Display {
    let lines = input.lines().collect::<Vec<_>>();
    let time = Regex::new(r"\d+")
        .unwrap()
        .captures_iter(lines[0])
        .map(|c| c[0].to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let record = Regex::new(r"\d+")
        .unwrap()
        .captures_iter(lines[1])
        .map(|c| c[0].to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    (1..time).filter(|h| (time - h) * h > record).count()
}

fn main() {
    let input: String = include_str!("../input/06").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
