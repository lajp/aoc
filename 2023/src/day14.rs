use std::fmt::Display;

fn part1(input: String) -> impl Display {
    let mut ans = 0;
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut newmap = map.clone();

    for (y, l) in map.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != 'O' {
                continue;
            }
            let mut cury = y;

            while cury > 0 && newmap[cury - 1][x] == '.' {
                newmap[cury - 1][x] = 'O';
                newmap[cury][x] = '.';
                cury -= 1;
            }

            ans += map.len() - cury;
        }
    }

    ans
}

fn cycle(map: &mut Vec<Vec<char>>) {
    let mut newmap = map.clone();

    for (y, l) in map.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != 'O' {
                continue;
            }
            let mut cury = y;

            while cury > 0 && newmap[cury - 1][x] == '.' {
                newmap[cury - 1][x] = 'O';
                newmap[cury][x] = '.';
                cury -= 1;
            }
        }
    }

    *map = newmap.clone();

    for (y, l) in map.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != 'O' {
                continue;
            }
            let mut curx = x;

            while curx > 0 && newmap[y][curx - 1] == '.' {
                newmap[y][curx - 1] = 'O';
                newmap[y][curx] = '.';
                curx -= 1;
            }
        }
    }

    *map = newmap.clone();

    for (y, l) in map.iter().enumerate().rev() {
        for (x, c) in l.iter().enumerate().rev() {
            if *c != 'O' {
                continue;
            }
            let mut cury = y;

            while cury < map.len() - 1 && newmap[cury + 1][x] == '.' {
                newmap[cury + 1][x] = 'O';
                newmap[cury][x] = '.';
                cury += 1;
            }
        }
    }

    *map = newmap.clone();

    for (y, l) in map.iter().enumerate().rev() {
        for (x, c) in l.iter().enumerate().rev() {
            if *c != 'O' {
                continue;
            }
            let mut curx = x;

            while curx < map.len() - 1 && newmap[y][curx + 1] == '.' {
                newmap[y][curx + 1] = 'O';
                newmap[y][curx] = '.';
                curx += 1;
            }
        }
    }

    *map = newmap.clone();
}

fn part2(input: String) -> impl Display {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut states = vec![map.clone()];

    let mut i = 0;
    let mut skip = false;
    while i < 1000000000 {
        cycle(&mut map);
        i += 1;
        if !skip {
            if let Some((j, _)) = states.iter().enumerate().find(|(_, s)| **s == map) {
                skip = true;
                i += ((1000000000 - i) / (i - j)) * (i - j);
            }
        }

        states.push(map.clone());
    }

    map.iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .map(|c| if *c == 'O' { map.len() - y } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let input: String = include_str!("../input/14").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
