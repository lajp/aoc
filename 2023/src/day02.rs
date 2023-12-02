use regex::Regex;
use std::cmp::max;
use std::fmt::Display;

fn part1(input: String) -> impl Display {
    let redmax = 12;
    let greenmax = 13;
    let bluemax = 14;
    let mut ans = 0;

    for (id, line) in input.lines().enumerate() {
        let mut possible = true;
        for cap in Regex::new(r"(\d+) (blue|red|green)")
            .unwrap()
            .captures_iter(line)
        {
            let num = cap[1].parse::<i32>().unwrap();
            match &cap[2] {
                "red" => {
                    possible = possible && num <= redmax;
                }
                "green" => {
                    possible = possible && num <= greenmax;
                }
                "blue" => {
                    possible = possible && num <= bluemax;
                }
                _ => unreachable!(),
            }
        }

        if possible {
            ans += id + 1
        }
    }
    ans
}

fn part2(input: String) -> impl Display {
    let mut ans = 0;
    for line in input.lines() {
        let mut redmax = 0;
        let mut greenmax = 0;
        let mut bluemax = 0;

        for cap in Regex::new(r"(\d+) (blue|red|green)")
            .unwrap()
            .captures_iter(line)
        {
            let num = cap[1].parse::<i32>().unwrap();
            match &cap[2] {
                "red" => {
                    redmax = max(redmax, num);
                }
                "green" => {
                    greenmax = max(greenmax, num);
                }
                "blue" => {
                    bluemax = max(bluemax, num);
                }
                _ => unreachable!(),
            }
        }

        ans += bluemax * greenmax * redmax;
    }
    ans
}

fn main() {
    let input: String = include_str!("../input/02").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
