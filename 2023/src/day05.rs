use std::fmt::Display;

use itertools::Itertools;
use regex::Regex;

fn part1(input: String) -> impl Display {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let mut seeds: Vec<u64> = Regex::new(r"\d+")
        .unwrap()
        .captures_iter(sections[0])
        .map(|c| c[0].parse::<u64>().unwrap())
        .collect();

    for section in sections.iter().skip(1) {
        let mut converted: Vec<u64> = vec![];
        let mut added: Vec<usize> = vec![];

        for cap in Regex::new(r"(?m)^(\d+) (\d+) (\d+)$")
            .unwrap()
            .captures_iter(section)
        {
            let dest = cap[1].parse::<u64>().unwrap();
            let source = cap[2].parse::<u64>().unwrap();
            let length = cap[3].parse::<u64>().unwrap();

            for (i, s) in seeds.iter().enumerate() {
                if *s >= source && *s <= source + length {
                    converted.push(dest + s - source);
                    added.push(i);
                }
            }
        }

        seeds
            .iter()
            .enumerate()
            .filter(|(i, _)| !added.contains(i))
            .for_each(|(_, s)| converted.push(*s));

        seeds = converted.clone();
    }

    *seeds.iter().min().unwrap()
}

fn part2(input: String) -> impl Display {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let mut seeds: Vec<(u64, u64)> = Regex::new(r"\d+")
        .unwrap()
        .captures_iter(sections[0])
        .map(|c| c[0].parse::<u64>().unwrap())
        .tuple_windows()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, t)| t)
        .collect();

    for section in sections.iter().skip(1) {
        let mut converted: Vec<(u64, u64)> = vec![];

        for cap in Regex::new(r"(?m)^(\d+) (\d+) (\d+)$")
            .unwrap()
            .captures_iter(section)
        {
            let dest = cap[1].parse::<u64>().unwrap();
            let source = cap[2].parse::<u64>().unwrap();
            let length = cap[3].parse::<u64>().unwrap();

            let mut i = 0;
            while i < seeds.len() {
                let (s, l) = seeds[i];
                if source >= s && source + length <= s + l {
                    // the mapping is contained within the seedset
                    converted.push((dest + source - s, length));
                    seeds.push((s, source - s));
                    seeds[i].0 = source + length;
                    seeds[i].1 -= s + l - (source + length);
                } else if source <= s && source + length > s {
                    // the mapping contains a lower part of the seedset
                    converted.push((dest + s - source, std::cmp::min(source + length - s, l)));
                    seeds[i].0 += converted[converted.len() - 1].1;
                    seeds[i].1 -= converted[converted.len() - 1].1;
                } else if source >= s && source < s + l {
                    // the mapping contains an upper part of the seedset
                    converted.push((dest, l));
                    seeds[i].1 = source - s;
                }

                i += 1
            }
        }

        seeds
            .iter()
            .filter(|(_, l)| *l > 0)
            .for_each(|sr| converted.push(*sr));

        seeds = converted.clone();
    }

    seeds.iter().min().unwrap().0
}

fn main() {
    let input: String = include_str!("../input/05").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
