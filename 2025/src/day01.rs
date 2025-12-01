fn part1(input: &str) -> impl std::fmt::Display {
    let mut dial = 50;
    input
        .lines()
        .map(|l| {
            let (c, n) = l.split_at(1);
            let n: i32 = n.parse().unwrap();

            match c {
                "L" => dial -= n,
                "R" => dial += n,
                _ => unreachable!(),
            }

            dial = dial.rem_euclid(100);
            if dial == 0 { 1 } else { 0 }
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut dial = 50;
    input
        .lines()
        .map(|l| {
            let (c, n) = l.split_at(1);
            let mut n: i64 = n.parse().unwrap();

            let mut ans = (n / 100) as usize;

            if c == "L" {
                n = -n
            }

            let m = n % 100;
            if dial != 0 && (dial + m <= 0 || dial + m > 99) {
                ans += 1;
            }

            dial = dial.wrapping_add(n).rem_euclid(100);

            ans
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/01.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
