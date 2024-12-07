use rayon::prelude::*;
use std::collections::HashSet;

fn newdir(dir: (i8, i8)) -> (i8, i8) {
    match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> impl std::fmt::Display {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.as_bytes().iter().collect())
        .collect();

    let cols = map[0].len();
    let rows = map.len();

    let mut pos = (0..rows)
        .flat_map(|y| (0..cols).map(move |x| (x, y)))
        .find(|(x, y)| map[*y][*x] == &b'^')
        .map(|(x, y)| (x as i64, y as i64))
        .unwrap();

    let mut dir: (i8, i8) = (0, -1);
    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);
        let nx = pos.0 + dir.0 as i64;
        let ny = pos.1 + dir.1 as i64;

        if nx < 0 || ny < 0 || nx >= map[0].len() as i64 || ny >= map.len() as i64 {
            break;
        }

        if map[ny as usize][nx as usize] == &b'#' {
            dir = newdir(dir)
        } else {
            pos = (nx, ny)
        }
    }

    visited.len()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.as_bytes().iter().collect())
        .collect();

    let cols = map[0].len();
    let rows = map.len();

    let mut pos = (0..rows)
        .flat_map(|y| (0..cols).map(move |x| (x, y)))
        .find(|(x, y)| map[*y][*x] == &b'^')
        .map(|(x, y)| (x as i64, y as i64))
        .unwrap();

    let start = pos;
    let mut dir: (i8, i8) = (0, -1);
    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);
        let nx = pos.0 + dir.0 as i64;
        let ny = pos.1 + dir.1 as i64;

        if nx < 0 || ny < 0 || nx >= map[0].len() as i64 || ny >= map.len() as i64 {
            break;
        }

        if map[ny as usize][nx as usize] == &b'#' {
            dir = newdir(dir)
        } else {
            pos = (nx, ny)
        }
    }

    fn dir_to_num(dir: (i8, i8)) -> usize {
        match dir {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => unreachable!(),
        }
    }

    visited
        .into_iter()
        .par_bridge()
        .filter(|obst| obst != &start)
        .filter(|obst| {
            let mut pos = start;
            let mut dir = (0, -1);
            let mut visited = vec![false; rows * cols * 4];

            loop {
                let ind = cols * pos.1 as usize * 4 + pos.0 as usize * 4 + dir_to_num(dir);
                if visited[ind] {
                    return true;
                }
                visited[ind] = true;

                let nx = pos.0 + dir.0 as i64;
                let ny = pos.1 + dir.1 as i64;

                if nx < 0 || ny < 0 || nx >= map[0].len() as i64 || ny >= map.len() as i64 {
                    return false;
                }

                if &(nx, ny) == obst || map[ny as usize][nx as usize] == &b'#' {
                    dir = newdir(dir);
                } else {
                    pos = (nx, ny);
                }
            }
        })
        .count()
}

fn main() {
    let input = include_str!("../inputs/06.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
