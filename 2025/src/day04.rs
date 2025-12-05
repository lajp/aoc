fn part1(input: &str) -> impl std::fmt::Display {
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut ans = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != '@' {
                continue;
            }

            let diffs: [(i32, i32); 8] = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 0),
                (-1, 1),
            ];

            let mut adj = 0;

            for (dx, dy) in diffs {
                let x = x as i32 + dx;
                let y = y as i32 + dy;

                if x >= 0
                    && x < map[0].len() as i32
                    && y >= 0
                    && y < map.len() as i32
                    && map[y as usize][x as usize] == '@'
                {
                    adj += 1;
                }
            }

            if adj < 4 {
                ans += 1;
            }
        }
    }

    ans
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut ans = 0;
    loop {
        let prev = ans;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] != '@' {
                    continue;
                }

                let diffs: [(i32, i32); 8] = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 0),
                    (-1, 1),
                ];

                let mut adj = 0;

                for (dx, dy) in diffs {
                    let x = x as i32 + dx;
                    let y = y as i32 + dy;

                    if x >= 0
                        && x < map[0].len() as i32
                        && y >= 0
                        && y < map.len() as i32
                        && map[y as usize][x as usize] == '@'
                    {
                        adj += 1;
                    }
                }

                if adj < 4 {
                    map[y][x] = '.';
                    ans += 1;
                }
            }
        }
        if ans == prev {
            break;
        }
    }

    ans
}

fn main() {
    let input = include_str!("../inputs/04.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
