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

    let mut ans = 1;

    for i in 0..times.len() {
        let mut ways = 0;
        for h in 1..times[i] {
            let dist = (times[i] - h) * h;
            if dist > records[i] {
                ways += 1
            }
        }

        ans *= ways;
    }

    ans
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

    let mut ans = 0;

    for h in 1..time {
        let dist = (time - h) * h;
        if dist > record {
            ans += 1
        }
    }

    ans
}

fn main() {
    let input: String = include_str!("../input/06").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
