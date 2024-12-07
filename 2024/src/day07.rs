use rayon::prelude::*;

fn is_valid1(cur: u64, nums: &[u64], target: u64) -> bool {
    if nums.is_empty() {
        return cur == target;
    }

    if cur > target {
        return false;
    }

    let (n, rest) = nums.split_first().unwrap();
    if is_valid1(cur + n, rest, target) || is_valid1(cur * n, rest, target) {
        return true;
    }
    false
}

fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .par_bridge()
        .map(|l| {
            let (t, nums) = l.split_once(":").unwrap();
            let t = t.parse::<u64>().unwrap();
            let nums: Vec<_> = nums
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            let (fst, rest) = nums.split_first().unwrap();

            if is_valid1(*fst, rest, t) {
                t
            } else {
                0
            }
        })
        .sum::<u64>()
}

fn is_valid2(cur: u64, nums: &[u64], target: u64) -> bool {
    if nums.is_empty() {
        return cur == target;
    }

    if cur > target {
        return false;
    }

    let (n, rest) = nums.split_first().unwrap();
    if is_valid2(cur + n, rest, target)
        || is_valid2(cur * n, rest, target)
        || is_valid2(cur * 10u64.pow((n * 10).ilog10()) + n, rest, target)
    {
        return true;
    }
    false
}

fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .par_bridge()
        .map(|l| {
            let (t, nums) = l.split_once(":").unwrap();
            let t = t.parse::<u64>().unwrap();
            let nums: Vec<_> = nums
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            let (fst, rest) = nums.split_first().unwrap();

            if is_valid2(*fst, rest, t) {
                t
            } else {
                0
            }
        })
        .sum::<u64>()
}

fn main() {
    let input = include_str!("../inputs/07.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
