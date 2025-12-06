fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let ops = lines.pop().unwrap();

    let mut lines: Vec<Vec<_>> = lines
        .into_iter()
        .map(|l| l.into_iter().map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();

    for (c, op) in ops.iter().enumerate() {
        for r in 1..lines.len() {
            match *op {
                "*" => lines[0][c] *= lines[r][c],
                "+" => lines[0][c] += lines[r][c],
                _ => unreachable!(),
            }
        }
    }

    lines[0].iter().sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut lines: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let ops = lines.pop().unwrap();

    let mut ans = 0;

    let mut nums = vec![];
    let mut skip = false;
    for c in (0..ops.len()).rev() {
        if skip {
            skip = false;
            continue;
        }

        let mut num = 0usize;
        for l in &lines {
            if let Some(n) = l[c].to_digit(10) {
                num *= 10;
                num += n as usize;
            }
        }

        nums.push(num);

        match ops[c] {
            '*' => ans += nums.drain(..).product::<usize>(),
            '+' => ans += nums.drain(..).sum::<usize>(),
            ' ' => continue,
            _ => unreachable!(),
        }

        skip = true;
    }

    ans
}

fn main() {
    let input = include_str!("../inputs/06.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
