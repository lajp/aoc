fn part1(input: &str) -> impl std::fmt::Display {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (_, [n1, n2]) = caps.extract();

            n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap()
        })
        .sum::<i64>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(input)
        .fold((0, true), |(sum, enable), caps| match (&caps[0], enable) {
            ("do()", _) => (sum, true),
            ("don't()", _) => (sum, false),
            (_, true) => (
                sum + caps[1].parse::<i64>().unwrap() * caps[2].parse::<i64>().unwrap(),
                enable,
            ),
            (_, false) => (sum, enable),
        })
        .0
}

fn main() {
    let input = include_str!("../inputs/03.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
