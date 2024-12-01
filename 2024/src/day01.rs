use itertools::Itertools;

fn part1(input: &str) -> impl std::fmt::Display {
    let (mut l1, mut l2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    l1.sort();
    l2.sort();

    l1.into_iter()
        .zip(l2)
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum::<i64>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let (l1, l2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    l1.into_iter()
        .map(|n| n * l2.iter().filter(|n2| **n2 == n).count() as i64)
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../inputs/01.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
