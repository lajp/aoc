use num::Integer;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;

fn part1(input: String) -> impl Display {
    let mut steps = 0;
    let sp = input.split("\n\n").collect::<Vec<_>>();
    let dirs = sp[0].to_string();
    let mut cur = "AAA".to_string();

    let adj: HashMap<String, (String, String)> = Regex::new(r"(?m)^(\w+) = \((\w+), (\w+)\)$")
        .unwrap()
        .captures_iter(sp[1])
        .map(|c| (c[1].to_string(), (c[2].to_string(), c[3].to_string())))
        .collect();

    let mut i = 0;
    loop {
        let ins = dirs.chars().nth(i).unwrap();
        match ins {
            'R' => cur = adj[&cur].1.clone(),
            'L' => cur = adj[&cur].0.clone(),
            _ => unreachable!(),
        }

        steps += 1;
        i += 1;
        i %= dirs.len();

        if cur == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2(input: String) -> impl Display {
    let mut steps = 0;
    let sp = input.split("\n\n").collect::<Vec<_>>();
    let dirs = sp[0].to_string();
    let adj: HashMap<String, (String, String)> = Regex::new(r"(?m)^(\w+) = \((\w+), (\w+)\)$")
        .unwrap()
        .captures_iter(sp[1])
        .map(|c| (c[1].to_string(), (c[2].to_string(), c[3].to_string())))
        .collect();

    let mut curs: Vec<String> = adj
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut lengths: Vec<u64> = vec![];
    loop {
        let ins = dirs.chars().nth(i).unwrap();
        let mut j = 0;
        while j < curs.len() {
            match ins {
                'R' => {
                    let a = adj[&curs[j]].1.clone();
                    curs[j] = a;
                }
                'L' => {
                    let a = adj[&curs[j]].0.clone();
                    curs[j] = a;
                }
                _ => unreachable!(),
            }
            j += 1;
        }

        steps += 1;
        i += 1;
        i %= dirs.len();

        curs.iter()
            .filter(|c| c.ends_with('Z'))
            .for_each(|_| lengths.push(steps as u64));
        curs.retain(|c| !c.ends_with('Z'));

        if curs.is_empty() {
            break;
        }
    }

    lengths.iter().fold(1, |a, c| a.lcm(c))
}

fn main() {
    let input: String = include_str!("../input/08").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
