use std::fmt::Display;

fn is_reflection(a: &str, b: &str) -> bool {
    a.chars().rev().zip(b.chars()).all(|(ac, bc)| ac == bc)
}

fn is_reflection_rows(a: Vec<&str>, b: Vec<&str>) -> bool {
    a.iter().rev().zip(b.iter()).all(|(ac, bc)| ac == bc)
}

fn part1(input: String) -> impl Display {
    let mut ans = 0;
    for pattern in input.split("\n\n") {
        let lines: Vec<&str> = pattern.lines().collect::<Vec<_>>();
        let column = (1..lines[0].len())
            .find(|c| {
                lines.iter().all(|l| {
                    is_reflection(
                        l.chars().take(*c).collect::<String>().as_str(),
                        l.chars().skip(*c).collect::<String>().as_str(),
                    )
                })
            })
            .unwrap_or(0);

        ans += column;

        let row = (1..lines.len())
            .find(|r| {
                is_reflection_rows(
                    lines.iter().cloned().take(*r).collect(),
                    lines.iter().cloned().skip(*r).collect(),
                )
            })
            .unwrap_or(0);

        ans += row * 100;
    }

    ans
}

fn reflection_diffs(a: &str, b: &str) -> usize {
    a.chars()
        .rev()
        .zip(b.chars())
        .filter(|(ac, bc)| ac != bc)
        .count()
}

fn is_reflection_rows_smudge(a: Vec<&str>, b: Vec<&str>) -> bool {
    a.iter()
        .rev()
        .zip(b.iter())
        .map(|(al, bl)| {
            al.chars()
                .zip(bl.chars())
                .filter(|(ac, bc)| ac != bc)
                .count()
        })
        .sum::<usize>()
        == 1
}

fn part2(input: String) -> impl Display {
    let mut ans = 0;
    for pattern in input.split("\n\n") {
        let lines: Vec<&str> = pattern.lines().collect::<Vec<_>>();
        let column = (1..lines[0].len())
            .find(|c| {
                lines
                    .iter()
                    .map(|l| {
                        reflection_diffs(
                            l.chars().take(*c).collect::<String>().as_str(),
                            l.chars().skip(*c).collect::<String>().as_str(),
                        )
                    })
                    .sum::<usize>()
                    == 1
            })
            .unwrap_or(0);

        ans += column;

        let row = (1..lines.len())
            .find(|r| {
                is_reflection_rows_smudge(
                    lines.iter().cloned().take(*r).collect(),
                    lines.iter().cloned().skip(*r).collect(),
                )
            })
            .unwrap_or(0);

        ans += row * 100;
    }

    ans
}

fn main() {
    let input: String = include_str!("../input/13").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
