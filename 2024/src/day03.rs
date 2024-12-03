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
    // NOTE: ()() is a hack in order to make the amount of capture groups equal
    // so that Captures::extract can be used
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|()()do\(\)|()()don't\(\)").unwrap();

    re.captures_iter(input)
        .fold((0, true), |(sum, enable), caps| {
            let (s, [n1, n2]) = caps.extract();

            match (s, enable) {
                ("do()", _) => (sum, true),
                ("don't()", _) => (sum, false),
                (_, true) => (
                    sum + n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap(),
                    enable,
                ),
                (_, false) => (sum, enable),
            }
        })
        .0
}

fn main() {
    let input = include_str!("../inputs/03.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
