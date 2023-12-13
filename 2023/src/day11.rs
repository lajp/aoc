use std::fmt::Display;

fn part1(input: String) -> impl Display {
    let mut rows_without_galaxy = vec![true; input.lines().count()];
    let mut columns_without_galaxy = vec![true; input.lines().next().unwrap().len()];
    let mut galaxies = vec![];

    for (row, l) in input.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            if c == '#' {
                rows_without_galaxy[row] = false;
                columns_without_galaxy[col] = false;
                galaxies.push((row, col));
            }
        }
    }

    let mut ans = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let max_y = std::cmp::max(galaxies[i].0, galaxies[j].0);
            let min_y = std::cmp::min(galaxies[i].0, galaxies[j].0);

            let max_x = std::cmp::max(galaxies[i].1, galaxies[j].1);
            let min_x = std::cmp::min(galaxies[i].1, galaxies[j].1);

            let yd = max_y - min_y
                + rows_without_galaxy
                    .iter()
                    .skip(min_y)
                    .take(max_y - min_y)
                    .filter(|v| **v)
                    .count();
            let xd = max_x - min_x
                + columns_without_galaxy
                    .iter()
                    .skip(min_x)
                    .take(max_x - min_x)
                    .filter(|v| **v)
                    .count();

            ans += xd + yd;
        }
    }

    ans
}

fn part2(input: String) -> impl Display {
    let mut rows_without_galaxy = vec![true; input.lines().count()];
    let mut columns_without_galaxy = vec![true; input.lines().next().unwrap().len()];
    let mut galaxies = vec![];

    for (row, l) in input.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            if c == '#' {
                rows_without_galaxy[row] = false;
                columns_without_galaxy[col] = false;
                galaxies.push((row, col));
            }
        }
    }

    let mut ans = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let max_y = std::cmp::max(galaxies[i].0, galaxies[j].0);
            let min_y = std::cmp::min(galaxies[i].0, galaxies[j].0);

            let max_x = std::cmp::max(galaxies[i].1, galaxies[j].1);
            let min_x = std::cmp::min(galaxies[i].1, galaxies[j].1);

            let yd = max_y - min_y
                + rows_without_galaxy
                    .iter()
                    .skip(min_y)
                    .take(max_y - min_y)
                    .filter(|v| **v)
                    .count()
                    * 999999;
            let xd = max_x - min_x
                + columns_without_galaxy
                    .iter()
                    .skip(min_x)
                    .take(max_x - min_x)
                    .filter(|v| **v)
                    .count()
                    * 999999;

            ans += xd + yd;
        }
    }

    ans
}

fn main() {
    let input: String = include_str!("../input/11").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
