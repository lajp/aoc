use std::fmt::Display;

use cached::proc_macro::cached;

use regex::Regex;

fn is_legal(arr: &str, groups: &Vec<usize>) -> bool {
    let mut group = 0;
    let mut cur = 0;
    for c in arr.chars() {
        if c == '#' {
            cur += 1;
            continue;
        }

        if cur > 0 {
            if group >= groups.len() || cur != groups[group] {
                return false;
            }

            cur = 0;
            group += 1
        }
    }

    (cur == 0 && group == groups.len()) || (group == groups.len() - 1 && cur == groups[group])
}

fn part1(input: String) -> impl Display {
    let mut ans = 0;

    for line in input.lines() {
        let groups = Regex::new(r"\d+")
            .unwrap()
            .captures_iter(line)
            .map(|c| c[0].parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let record = line.split(' ').next().unwrap().to_string();
        let p = record.chars().filter(|c| *c == '?').count() as u32;
        let n = (0..2_usize.pow(p))
            .map(|n| {
                let mut nth = 0;
                let arr = record
                    .chars()
                    .map(|c| {
                        if c == '?' {
                            nth += 1;
                            if n & (1 << (nth - 1)) > 0 {
                                '#'
                            } else {
                                '.'
                            }
                        } else {
                            c
                        }
                    })
                    .collect::<String>();

                if is_legal(&arr, &groups) {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>();

        ans += n;
    }

    ans
}

#[cached]
fn recurse(record: Vec<char>, groups: Vec<usize>) -> usize {
    if groups.is_empty() {
        return if !record.contains(&'#') { 1 } else { 0 };
    }

    if record.is_empty() {
        return 0;
    }

    match record[0] {
        '.' => recurse(record.iter().copied().skip(1).collect(), groups.clone()),
        '#' => {
            if !record
                .iter()
                .take(groups[0])
                .copied()
                .all(|c| c == '#' || c == '?')
                || record.len() < groups[0]
            {
                0
            } else if record.len() == groups[0] {
                if groups.len() == 1 {
                    1
                } else {
                    0
                }
            } else {
                let char_after = *record.get(groups[0]).unwrap();
                if char_after == '?' || char_after == '.' {
                    recurse(
                        record.iter().copied().skip(groups[0] + 1).collect(),
                        groups.iter().copied().skip(1).collect(),
                    )
                } else {
                    0
                }
            }
        }
        '?' => {
            let with_dot = {
                let mut newrec = record.clone();
                newrec[0] = '.';
                recurse(newrec, groups.clone())
            };

            let with_hash = {
                let mut newrec = record.clone();
                newrec[0] = '#';
                recurse(newrec, groups.clone())
            };

            with_dot + with_hash
        }
        _ => unreachable!(),
    }
}

fn part2(input: String) -> impl Display {
    let mut ans = 0;

    for line in input.lines() {
        let groups = Regex::new(r"\d+")
            .unwrap()
            .captures_iter(line)
            .map(|c| c[0].parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .repeat(5);

        let record = [line.split(' ').next().unwrap()]
            .iter()
            .cycle()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join("?");

        ans += recurse(record.chars().collect(), groups)
    }

    ans
}

fn main() {
    let input: String = include_str!("../input/12").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
