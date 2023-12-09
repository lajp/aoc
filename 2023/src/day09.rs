use std::{collections::VecDeque, fmt::Display};

use itertools::Itertools;
use regex::Regex;

fn part1(input: String) -> impl Display {
    let mut ans = 0;
    for line in input.lines() {
        let mut seqs: Vec<Vec<i64>> = vec![];
        seqs.push(
            Regex::new(r"-?\d+")
                .unwrap()
                .captures_iter(line)
                .map(|c| c[0].parse().unwrap())
                .collect(),
        );

        loop {
            let newseq = seqs
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>();

            if newseq.iter().all(|v| *v == 0) {
                break;
            }

            seqs.push(newseq);
        }

        for i in (1..seqs.len()).rev() {
            let li = seqs[i].last().unwrap();
            let s = seqs[i - 1].last().unwrap() + li;
            seqs[i - 1].push(s);
        }

        ans += seqs[0].last().unwrap();
    }

    ans
}

fn part2(input: String) -> impl Display {
    let mut ans = 0;
    for line in input.lines() {
        let mut seqs: Vec<VecDeque<i64>> = vec![];
        seqs.push(
            Regex::new(r"-?\d+")
                .unwrap()
                .captures_iter(line)
                .map(|c| c[0].parse().unwrap())
                .collect(),
        );

        loop {
            let newseq = seqs
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<VecDeque<_>>();

            if newseq.iter().all(|v| *v == 0) {
                break;
            }

            seqs.push(newseq);
        }

        for i in (1..seqs.len()).rev() {
            let li = seqs[i][0];
            let s = seqs[i - 1][0] - li;
            seqs[i - 1].push_front(s);
        }

        ans += seqs[0][0];
    }

    ans
}

fn main() {
    let input: String = include_str!("../input/09").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
