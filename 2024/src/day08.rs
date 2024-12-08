use itertools::Itertools;
use num::Integer;

fn part1(input: &str) -> impl std::fmt::Display {
    let nx = input.lines().next().map(|l| l.len()).unwrap();
    let ny = input.lines().count();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().flat_map(move |(x, c)| {
                if c != b'.' {
                    Some((c, (x as i64, y as i64)))
                } else {
                    None
                }
            })
        })
        .into_group_map();

    antennas
        .into_values()
        .flat_map(|vs| {
            vs.iter()
                .cartesian_product(vs.iter())
                .filter(|(i, j)| i != j)
                .flat_map(|((x1, y1), (x2, y2))| {
                    let dx = x1 - x2;
                    let dy = y1 - y2;

                    let x = x1 + dx;
                    let y = y1 + dy;
                    if (0..nx as i64).contains(&x) && (0..ny as i64).contains(&y) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unique()
        .count()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let nx = input.lines().next().map(|l| l.len()).unwrap();
    let ny = input.lines().count();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().flat_map(move |(x, c)| {
                if c != b'.' {
                    Some((c, (x as i64, y as i64)))
                } else {
                    None
                }
            })
        })
        .into_group_map();

    antennas
        .into_values()
        .flat_map(|vs| {
            vs.iter()
                .cartesian_product(vs.iter())
                .filter(|(i, j)| i != j)
                .flat_map(|((x1, y1), (x2, y2))| {
                    let mut dx = x1 - x2;
                    let mut dy = y1 - y2;
                    let d = dx.gcd(&dy);
                    dx /= d;
                    dy /= d;

                    std::iter::successors(Some((*x1, *y1)), move |(x, y)| Some((x + dx, y + dy)))
                        .take_while(|(x, y)| {
                            (0..nx as i64).contains(x) && (0..ny as i64).contains(y)
                        })
                })
                .collect::<Vec<_>>()
        })
        .unique()
        .count()
}

fn main() {
    let input = include_str!("../inputs/08.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
