use itertools::Itertools;

pub fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| l.split_whitespace().flat_map(|n| n.parse::<i64>()))
        .map(|nums| {
            nums.tuple_windows()
                .map(|(prev, cur)| (prev - cur, cur - prev))
        })
        .filter(|v| {
            v.clone().all(|(n, _)| (1..=3).contains(&n))
                || v.clone().all(|(_, n)| (1..=3).contains(&n))
        })
        .count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|n| n.parse::<i64>())
                .collect::<Vec<_>>()
        })
        .filter(|nums| {
            for rm in 0..=nums.len() {
                let diffs = nums
                    .iter()
                    .take(rm)
                    .chain(nums.iter().skip(rm + 1))
                    .tuple_windows()
                    .map(|(prev, cur)| (prev - cur, cur - prev));

                if diffs.clone().all(|(n, _)| (1..=3).contains(&n))
                    || diffs.clone().all(|(_, n)| (1..=3).contains(&n))
                {
                    return true;
                }
            }

            false
        })
        .count()
}

fn main() {
    let input = include_str!("../inputs/02.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
