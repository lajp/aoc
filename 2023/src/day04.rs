use std::fmt::Display;

fn part1(input: String) -> impl Display {
    let mut ans = 0;
    for line in input.lines() {
        let sp = line.split('|').collect::<Vec<_>>();
        let winning = sp[0].split(' ').map(|s| s.trim()).collect::<Vec<_>>();
        let mut value = 0;

        for num in sp[1].split(' ') {
            if !num.is_empty() && winning.contains(&num.trim()) {
                if value == 0 {
                    value = 1
                } else {
                    value *= 2;
                }
            }
        }
        ans += value;
    }
    ans
}

fn part2(input: String) -> impl Display {
    let mut card_amounts = vec![1; input.lines().count()];
    for (c, line) in input.lines().enumerate() {
        let l = line.split('|').collect::<Vec<_>>();
        let winning = l[0].split(' ').map(|s| s.trim()).collect::<Vec<_>>();
        let mut value = 0;

        for num in l[1].split(' ') {
            if !num.is_empty() && winning.contains(&num.trim()) {
                value += 1;
            }
        }

        for i in (c + 1)..=(c + value) {
            card_amounts[i] += card_amounts[c];
        }
    }
    card_amounts.iter().sum::<i32>()
}

fn main() {
    let input: String = include_str!("../input/04").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
