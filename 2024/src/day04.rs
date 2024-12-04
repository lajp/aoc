fn part1(input: &str) -> impl std::fmt::Display {
    let map: Vec<&str> = input.lines().collect();

    let x = map
        .iter()
        .map(|s| s.matches("XMAS").count() + s.matches("SAMX").count())
        .sum::<usize>();

    let y = (0..map[0].len())
        .map(|x| {
            let s: String = (0..map.len()).map(|y| &map[y][x..x + 1]).collect();
            s.matches("XMAS").count() + s.matches("SAMX").count()
        })
        .sum::<usize>();

    let mut d = 0;
    for x in 3..map[0].len() {
        for y in 0..map.len() {
            if y >= 3 {
                let diag = map[y - 3][x - 3..x - 2].to_owned()
                    + &map[y - 2][x - 2..x - 1]
                    + &map[y - 1][x - 1..x]
                    + &map[y][x..x + 1];

                if matches!(diag.as_str(), "XMAS" | "SAMX") {
                    d += 1;
                }
            }

            if y + 3 < map.len() {
                let diag = map[y + 3][x - 3..x - 2].to_owned()
                    + &map[y + 2][x - 2..x - 1]
                    + &map[y + 1][x - 1..x]
                    + &map[y][x..x + 1];

                if matches!(diag.as_str(), "XMAS" | "SAMX") {
                    d += 1;
                }
            }
        }
    }

    x + y + d
}

fn part2(input: &str) -> impl std::fmt::Display {
    let map: Vec<&str> = input.lines().collect();

    let mut ans = 0;
    for y in 1..map.len() - 1 {
        for x in 1..map.len() - 1 {
            if map[y][x..x + 1] == *"A" {
                let d1 = map[y - 1][x - 1..x].to_owned() + &map[y + 1][x + 1..x + 2];
                let d2 = map[y - 1][x + 1..x + 2].to_owned() + &map[y + 1][x - 1..x];

                if matches!((d1.as_str(), d2.as_str()), ("MS" | "SM", "MS" | "SM")) {
                    ans += 1
                }
            }
        }
    }

    ans
}

fn main() {
    let input = include_str!("../inputs/04.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
