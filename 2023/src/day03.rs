use std::{collections::HashMap, fmt::Display};

fn part1(input: String) -> impl Display {
    let mut parts: u64 = 0;
    let matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, line) in matrix.iter().enumerate() {
        let mut curnum: u64 = 0;
        let mut addcur = false;

        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                curnum *= 10;
                curnum += c.to_digit(10).unwrap() as u64;
                let x = x as i32;
                let y = y as i32;
                let coords = [
                    (y - 1, x - 1),
                    (y - 1, x + 1),
                    (y - 1, x),
                    (y + 1, x - 1),
                    (y + 1, x + 1),
                    (y + 1, x),
                    (y, x - 1),
                    (y, x + 1),
                ];

                addcur = addcur
                    || coords
                        .iter()
                        .filter_map(|(y, x)| {
                            if *x >= 0 && *y >= 0 {
                                let x = *x as usize;
                                let y = *y as usize;
                                if x < line.len() && y < matrix.len() {
                                    let s = matrix[y][x];
                                    Some(s != '.' && !s.is_alphanumeric())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .any(|v| v);
                continue;
            }
            if addcur {
                parts += curnum;
                addcur = false;
            }

            curnum = 0;
        }

        if addcur {
            parts += curnum;
        }
    }

    parts
}

fn part2(input: String) -> impl Display {
    let matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut gears = matrix
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if *c == '*' {
                        Some(((y, x), vec![]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(usize, usize), Vec<u64>>>();

    for (y, line) in matrix.iter().enumerate() {
        let mut curnum = 0;
        let mut numgears = Vec::<(usize, usize)>::new();

        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                curnum *= 10;
                curnum += c.to_digit(10).unwrap() as u64;
                let x = x as i32;
                let y = y as i32;
                let coords = [
                    (y - 1, x - 1),
                    (y - 1, x + 1),
                    (y - 1, x),
                    (y + 1, x - 1),
                    (y + 1, x + 1),
                    (y + 1, x),
                    (y, x - 1),
                    (y, x + 1),
                ];

                coords.iter().for_each(|(y, x)| {
                    if *x >= 0 && *y >= 0 {
                        let x = *x as usize;
                        let y = *y as usize;
                        if x < line.len() && y < matrix.len() {
                            let s = matrix[y][x];
                            if s == '*' {
                                numgears.push((y, x));
                            }
                        }
                    }
                });
                continue;
            }

            if curnum > 0 {
                numgears.dedup();
                numgears
                    .iter()
                    .for_each(|gc| gears.get_mut(gc).unwrap().push(curnum));
            }
            curnum = 0;
            numgears.clear();
        }

        numgears.dedup();
        numgears
            .iter()
            .for_each(|gc| gears.get_mut(gc).unwrap().push(curnum));
    }

    gears
        .iter()
        .flat_map(|(_, v)| {
            if v.len() == 2 {
                Some(v.iter().product::<u64>())
            } else {
                None
            }
        })
        .sum::<u64>()
}

fn main() {
    let input: String = include_str!("../input/03").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
