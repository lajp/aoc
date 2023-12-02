use std::fmt::Display;

fn part1(input: String) -> impl Display {
    let redmax = 12;
    let greenmax = 13;
    let bluemax = 14;
    let mut ans = 0;

    for line in input.lines() {
        let sp = line.split(' ').collect::<Vec<_>>();
        let colors = line.split(';').collect::<Vec<_>>();
        let id: i32 = sp[1]
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        let mut possible = true;

        for cs in colors {
            let cs = cs
                .strip_prefix(&format!("Game {}: ", id))
                .unwrap_or(cs)
                .split(',')
                .collect::<Vec<_>>();
            for c in cs {
                let c = c.trim().split(' ').collect::<Vec<_>>();
                let num = c[0].parse::<i32>().unwrap();
                match c[1] {
                    "blue" => {
                        if num > bluemax {
                            possible = false
                        }
                    }
                    "red" => {
                        if num > redmax {
                            possible = false
                        }
                    }
                    "green" => {
                        if num > greenmax {
                            possible = false
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        if possible {
            ans += id
        }
    }
    ans
}

fn part2(input: String) -> impl Display {
    let mut ans = 0;
    for line in input.lines() {
        let mut redmax = 0;
        let mut greenmax = 0;
        let mut bluemax = 0;

        let sp = line.split(' ').collect::<Vec<_>>();
        let colors = line.split(';').collect::<Vec<_>>();
        let id: i32 = sp[1]
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();

        for cs in colors {
            let cs = cs
                .strip_prefix(&format!("Game {}: ", id))
                .unwrap_or(cs)
                .split(',')
                .collect::<Vec<_>>();

            for c in cs {
                let c = c.trim().split(' ').collect::<Vec<_>>();
                let num = c[0].parse::<i32>().unwrap();
                match c[1] {
                    "blue" => {
                        if num > bluemax {
                            bluemax = num
                        }
                    }
                    "red" => {
                        if num > redmax {
                            redmax = num
                        }
                    }
                    "green" => {
                        if num > greenmax {
                            greenmax = num
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        ans += bluemax * greenmax * redmax;
    }
    ans
}

fn main() {
    let input: String = include_str!("../input/02").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
