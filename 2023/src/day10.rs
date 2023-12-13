use std::fmt::Display;

use itertools::Itertools;

fn neighbours(pos: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];

    if pos.0 > 0 {
        ret.push((pos.0 - 1, pos.1));
    }
    if pos.0 < height - 1 {
        ret.push((pos.0 + 1, pos.1));
    }

    if pos.1 > 0 {
        ret.push((pos.0, pos.1 - 1));
    }
    if pos.1 < width - 1 {
        ret.push((pos.0, pos.1 + 1));
    }

    ret
}

fn legal(from: (usize, usize), to: (usize, usize), tochar: char) -> bool {
    if tochar == 'S' {
        return true;
    }

    if from.1 == to.1 {
        if to.0 > from.0 {
            "|JL".contains(tochar)
        } else {
            "|7F".contains(tochar)
        }
    } else if to.1 > from.1 {
        "-7J".contains(tochar)
    } else {
        "-LF".contains(tochar)
    }
}

fn part1(input: String) -> impl Display {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            if let Some((x, _)) = l.chars().enumerate().find(|(_, c)| *c == 'S') {
                Some((y, x))
            } else {
                None
            }
        })
        .unwrap();

    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map[0].len();
    let height = map.len();

    let mut q = Vec::new();
    let ns: Vec<_> = neighbours(start, height, width)
        .iter()
        .filter(|p| legal(start, **p, map[p.0][p.1]))
        .cloned()
        .collect();

    q.push((start, ns.into_iter().peekable()));

    while let Some((pos, mut it)) = q.pop() {
        let tile = map[pos.0][pos.1];

        if it.peek().is_none() {
            continue;
        }

        if let Some(new) =
            match tile {
                'S' => {
                    if q.iter().any(|(p, _)| *p == start) {
                        break;
                    }

                    it.next()
                }
                '|' => it.find(|p| p.1 == pos.1),
                '-' => it.find(|p| p.0 == pos.0),
                'F' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 + 1 || p.1 == pos.1 && p.0 == pos.0 + 1),
                '7' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 - 1 || p.1 == pos.1 && p.0 == pos.0 + 1),
                'L' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 + 1 || p.1 == pos.1 && p.0 == pos.0 - 1),
                'J' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 - 1 || p.1 == pos.1 && p.0 == pos.0 - 1),
                _ => None,
            }
        {
            let ns: Vec<_> = neighbours(new, height, width)
                .iter()
                .filter(|p| **p != pos && legal(new, **p, map[p.0][p.1]))
                .cloned()
                .collect();

            q.push((pos, it));
            q.push((new, ns.into_iter().peekable()));
            continue;
        }
    }

    q.len() / 2
}

fn part2(input: String) -> impl Display {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            if let Some((x, _)) = l.chars().enumerate().find(|(_, c)| *c == 'S') {
                Some((y, x))
            } else {
                None
            }
        })
        .unwrap();

    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map[0].len();
    let height = map.len();

    let mut q = Vec::new();
    let ns: Vec<_> = neighbours(start, height, width)
        .iter()
        .filter(|p| legal(start, **p, map[p.0][p.1]))
        .cloned()
        .collect();

    q.push((start, ns.into_iter().peekable()));

    while let Some((pos, mut it)) = q.pop() {
        let tile = map[pos.0][pos.1];

        if it.peek().is_none() {
            continue;
        }

        if let Some(new) =
            match tile {
                'S' => {
                    if q.iter().any(|(p, _)| *p == start) {
                        break;
                    }

                    it.next()
                }
                '|' => it.find(|p| p.1 == pos.1),
                '-' => it.find(|p| p.0 == pos.0),
                'F' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 + 1 || p.1 == pos.1 && p.0 == pos.0 + 1),
                '7' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 - 1 || p.1 == pos.1 && p.0 == pos.0 + 1),
                'L' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 + 1 || p.1 == pos.1 && p.0 == pos.0 - 1),
                'J' => it
                    .find(|p| p.0 == pos.0 && p.1 == pos.1 - 1 || p.1 == pos.1 && p.0 == pos.0 - 1),
                _ => None,
            }
        {
            let ns: Vec<_> = neighbours(new, height, width)
                .iter()
                .filter(|p| **p != pos && legal(new, **p, map[p.0][p.1]))
                .cloned()
                .collect();

            q.push((pos, it));
            q.push((new, ns.into_iter().peekable()));
            continue;
        }
    }

    let mut path = q.iter().map(|(p, _)| p).collect::<Vec<_>>();
    path.push(path.first().unwrap());

    // https://en.wikipedia.org/wiki/Shoelace_formula and
    // https://en.wikipedia.org/wiki/Pick's_theorem
    let area = path
        .iter()
        .map(|(x, y)| (*x as f64, *y as f64))
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<f64>()
        / 2.0;

    (area.abs() - (path.len() as f64 / 2.0) + 1.0).round()
}

fn main() {
    let input: String = include_str!("../input/10").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
