use std::fmt::Display;

fn part1(input: String) -> impl Display {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            s.bytes()
                .map(|c| c as usize)
                .fold(0, |acc, c| ((acc + c) * 17) % 256)
        })
        .sum::<usize>()
}

fn part2(input: String) -> impl Display {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    for s in input.lines().next().unwrap().split(',') {
        let key = s
            .chars()
            .take_while(|c| c != &'=' && c != &'-')
            .collect::<String>();
        let hash = key
            .bytes()
            .take_while(|b| *b != b'=' && *b != b'-')
            .map(|c| c as usize)
            .fold(0, |acc, c| ((acc + c) * 17) % 256);

        if s.contains('=') {
            let sp = s.split('=').collect::<Vec<_>>();
            if let Some((i, _)) = boxes[hash]
                .iter()
                .enumerate()
                .find(|(_, (ss, _))| *ss == key)
            {
                boxes[hash][i] = (sp[0], sp[1].parse().unwrap())
            } else {
                boxes[hash].push((sp[0], sp[1].parse().unwrap()));
            }
        } else {
            boxes[hash].retain(|(ss, _)| *ss != key);
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(bi, s)| {
            s.iter()
                .enumerate()
                .map(|(i, (_, l))| l * (i + 1))
                .sum::<usize>()
                * (bi + 1)
        })
        .sum::<usize>()
}

fn main() {
    let input: String = include_str!("../input/15").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
