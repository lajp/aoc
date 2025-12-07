use std::collections::{HashMap, HashSet, VecDeque};

fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines: VecDeque<_> = input.lines().collect();
    let mut beams: HashSet<_> = lines
        .pop_front()
        .and_then(|l| l.find('S'))
        .into_iter()
        .collect();

    lines
        .into_iter()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter(|(x, c)| {
                    if *c == '^' && beams.remove(x) {
                        beams.insert(x - 1);
                        beams.insert(x + 1);
                        true
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut lines: VecDeque<_> = input.lines().collect();
    let mut beams: HashMap<_, _> = lines
        .pop_front()
        .and_then(|l| l.find('S').map(|i| (i, 1)))
        .into_iter()
        .collect();

    lines
        .into_iter()
        .map(|l| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^'
                        && let Some(n) = beams.remove(&x)
                    {
                        beams.entry(x + 1).and_modify(|e| *e += n).or_insert(n);
                        beams.entry(x - 1).and_modify(|e| *e += n).or_insert(n);
                        n
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        + 1
}

fn main() {
    let input = include_str!("../inputs/07.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
