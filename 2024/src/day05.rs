use itertools::Itertools;

fn part1(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();

    let before = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (b, a) = l.split_once("|").unwrap();
            (b.parse::<usize>().unwrap(), a.parse::<usize>().unwrap())
        })
        .into_group_map();

    lines
        .flat_map(|p| {
            let nums: Vec<usize> = p.split(",").map(|n| n.parse().unwrap()).collect();

            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    if before
                        .get(&nums[j])
                        .map(|bs| bs.contains(&nums[i]))
                        .unwrap_or(false)
                    {
                        return None;
                    }
                }
            }

            Some(nums[nums.len() / 2])
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut lines = input.lines();

    let before = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (b, a) = l.split_once("|").unwrap();
            (b.parse::<usize>().unwrap(), a.parse::<usize>().unwrap())
        })
        .into_group_map();

    lines
        .flat_map(|p| {
            let mut nums: Vec<usize> = p.split(",").map(|n| n.parse().unwrap()).collect();

            let mut swapped = false;

            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    if before
                        .get(&nums[j])
                        .map(|bs| bs.contains(&nums[i]))
                        .unwrap_or(false)
                    {
                        swapped = true;
                        nums.swap(i, j);
                    }
                }
            }

            if swapped {
                Some(nums[nums.len() / 2])
            } else {
                None
            }
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/05.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
