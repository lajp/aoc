use itertools::Itertools;

fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let nums = l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let fst = nums[..nums.len() - 1]
                .iter()
                .enumerate()
                .max_by_key(|(i, x)| (**x, -(*i as i32)))
                .unwrap()
                .0;
            let snd = nums[fst + 1..].iter().position_max().unwrap() + fst + 1;

            (nums[fst] * 10 + nums[snd]) as usize
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .map(|l| {
            let nums = l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();

            let mut prev: i32 = -1;
            let mut num = 0usize;
            for i in (0..=11).rev() {
                prev = nums[(prev + 1) as usize..nums.len() - i]
                    .iter()
                    .enumerate()
                    .max_by_key(|(i, x)| (**x, -(*i as i32)))
                    .unwrap()
                    .0 as i32
                    + prev
                    + 1;

                num *= 10;
                num += nums[prev as usize] as usize;
            }

            num
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/03.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
