use std::collections::HashSet;

fn part1(input: &str) -> impl std::fmt::Display {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mx = map[0].len();
    let my = map.len();

    fn ends(
        map: &Vec<Vec<u32>>,
        x: usize,
        y: usize,
        mx: usize,
        my: usize,
    ) -> HashSet<(usize, usize)> {
        let v = map[y][x];

        let mut set = HashSet::new();

        if v == 9 {
            set.insert((x, y));
            return set;
        }

        if x < mx - 1 && map[y][x + 1] == v + 1 {
            set.extend(ends(map, x + 1, y, mx, my));
        }

        if x > 0 && map[y][x - 1] == v + 1 {
            set.extend(ends(map, x - 1, y, mx, my));
        }

        if y < my - 1 && map[y + 1][x] == v + 1 {
            set.extend(ends(map, x, y + 1, mx, my));
        }

        if y > 0 && map[y - 1][x] == v + 1 {
            set.extend(ends(map, x, y - 1, mx, my));
        }

        set
    }

    (0..my)
        .map(|y| {
            (0..mx)
                .filter(|x| map[y][*x] == 0)
                .map(|x| ends(&map, x, y, mx, my).len())
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mx = map[0].len();
    let my = map.len();

    fn ends(map: &Vec<Vec<u32>>, x: usize, y: usize, mx: usize, my: usize) -> usize {
        let v = map[y][x];
        if v == 9 {
            return 1;
        }

        let mut sum = 0;

        if x < mx - 1 && map[y][x + 1] == v + 1 {
            sum += ends(map, x + 1, y, mx, my);
        }

        if x > 0 && map[y][x - 1] == v + 1 {
            sum += ends(map, x - 1, y, mx, my);
        }

        if y < my - 1 && map[y + 1][x] == v + 1 {
            sum += ends(map, x, y + 1, mx, my);
        }

        if y > 0 && map[y - 1][x] == v + 1 {
            sum += ends(map, x, y - 1, mx, my);
        }

        sum
    }

    (0..my)
        .map(|y| {
            (0..mx)
                .filter(|x| map[y][*x] == 0)
                .map(|x| ends(&map, x, y, mx, my))
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/10.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
