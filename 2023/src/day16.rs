use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;

fn part1(input: String) -> impl Display {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut beams: Vec<((i64, i64), (i8, i8))> = vec![((-1, 0), (1, 0))];
    let mut visited = HashSet::<((i64, i64), (i8, i8))>::new();

    while let Some(((x, y), (xv, yv))) = beams.pop() {
        let nx = x + xv as i64;
        let ny = y + yv as i64;
        if nx < 0 || nx > (map[0].len() - 1) as i64 || ny < 0 || ny > (map.len() - 1) as i64 {
            continue;
        }

        if visited.contains(&((nx, ny), (xv, yv))) {
            continue;
        }

        visited.insert(((nx, ny), (xv, yv)));

        match map[ny as usize][nx as usize] {
            '|' => {
                if xv != 0 {
                    beams.push(((nx, ny), (0, -1)));
                    beams.push(((nx, ny), (0, 1)));
                } else {
                    beams.push(((nx, ny), (xv, yv)));
                }
            }
            '-' => {
                if yv != 0 {
                    beams.push(((nx, ny), (-1, 0)));
                    beams.push(((nx, ny), (1, 0)));
                } else {
                    beams.push(((nx, ny), (xv, yv)));
                }
            }
            '/' => beams.push(((nx, ny), (-yv, -xv))),
            '\\' => beams.push(((nx, ny), (yv, xv))),
            _ => beams.push(((nx, ny), (xv, yv))),
        }
    }

    visited.iter().map(|(v, _)| v).unique().count()
}

fn part2(input: String) -> impl Display {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (0..map[0].len())
        .flat_map(|x| {
            vec![
                ((x as i64, -1), (0, 1)),
                ((x as i64, map.len() as i64), (0, -1)),
            ]
        })
        .chain((0..map.len()).flat_map(|y| {
            vec![
                ((-1, y as i64), (1, 0)),
                ((map[0].len() as i64, y as i64), (-1, 0)),
            ]
        }))
        .map(|starter| {
            let mut beams: Vec<((i64, i64), (i8, i8))> = vec![starter];
            let mut visited = HashSet::<((i64, i64), (i8, i8))>::new();

            while let Some(((x, y), (xv, yv))) = beams.pop() {
                let nx = x + xv as i64;
                let ny = y + yv as i64;
                if nx < 0 || nx > (map[0].len() - 1) as i64 || ny < 0 || ny > (map.len() - 1) as i64
                {
                    continue;
                }

                if visited.contains(&((nx, ny), (xv, yv))) {
                    continue;
                }

                visited.insert(((nx, ny), (xv, yv)));

                match map[ny as usize][nx as usize] {
                    '|' => {
                        if xv != 0 {
                            beams.push(((nx, ny), (0, -1)));
                            beams.push(((nx, ny), (0, 1)));
                        } else {
                            beams.push(((nx, ny), (xv, yv)));
                        }
                    }
                    '-' => {
                        if yv != 0 {
                            beams.push(((nx, ny), (-1, 0)));
                            beams.push(((nx, ny), (1, 0)));
                        } else {
                            beams.push(((nx, ny), (xv, yv)));
                        }
                    }
                    '/' => beams.push(((nx, ny), (-yv, -xv))),
                    '\\' => beams.push(((nx, ny), (yv, xv))),
                    _ => beams.push(((nx, ny), (xv, yv))),
                }
            }

            visited.iter().map(|(v, _)| v).unique().count()
        })
        .max()
        .unwrap()
}

fn main() {
    let input: String = include_str!("../input/16").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
