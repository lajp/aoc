fn part1(input: String) -> String {
    let mut total = 0;
    for line in input.lines() {
        let nums = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        total += nums[0] * 10 + nums.last().unwrap()
    }

    total.to_string()
}

fn part2(input: String) -> String {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut total = 0;
    for line in input.lines() {
        let mut first = 0;
        let mut second = 0;
        let mut seen = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                first = c.to_digit(10).unwrap();
                break;
            } else {
                seen += &c.to_string();
            }

            if let Some((i, _)) = nums
                .iter()
                .map(|w| seen.contains(w))
                .enumerate()
                .find(|(_, w)| *w)
            {
                first = (i + 1) as u32;
                break;
            }
        }

        seen.clear();
        for c in line.chars().rev() {
            if c.is_numeric() {
                second = c.to_digit(10).unwrap();
                break;
            } else {
                seen += &c.to_string();
            }

            if let Some((i, _)) = nums
                .iter()
                .map(|w| seen.chars().rev().collect::<String>().contains(w))
                .enumerate()
                .find(|(_, w)| *w)
            {
                second = (i + 1) as u32;
                break;
            }
        }

        total += first * 10 + second;
    }

    total.to_string()
}

fn main() {
    let input: String = include_str!("../input/01").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input.clone()));
}
