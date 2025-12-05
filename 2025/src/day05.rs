use std::ops::RangeInclusive;
fn part1(input: &str) -> impl std::fmt::Display {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<_> = ranges
        .lines()
        .map(|l| {
            let (s, e) = l.split_once('-').unwrap();

            s.parse::<usize>().unwrap()..=e.parse().unwrap()
        })
        .collect();

    ids.lines()
        .map(|l| {
            let id = l.parse().unwrap();

            if ranges.iter().any(|r| r.contains(&id)) {
                1
            } else {
                0
            }
        })
        .sum::<usize>()
}

struct RangeSet {
    inner: Vec<RangeInclusive<usize>>,
}

impl RangeSet {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn insert(&mut self, range: RangeInclusive<usize>) {
        let (start, end) = range.clone().into_inner();

        let overlapping = self
            .inner
            .iter()
            .filter(|r| {
                r.contains(&start)
                    || r.contains(&end)
                    || range.contains(r.start())
                    || range.contains(r.end())
            })
            .cloned()
            .collect::<Vec<_>>();

        self.inner.retain(|r| {
            !(r.contains(&start)
                || r.contains(&end)
                || range.contains(r.start())
                || range.contains(r.end()))
        });

        let start = overlapping
            .iter()
            .map(|r| r.start())
            .copied()
            .chain([start])
            .min()
            .unwrap();
        let end = overlapping
            .iter()
            .map(|r| r.end())
            .copied()
            .chain([end])
            .max()
            .unwrap();

        self.inner.push(start..=end);
    }

    pub fn size(&self) -> usize {
        self.inner.iter().map(|r| r.clone().count()).sum()
    }
}

fn part2(input: &str) -> impl std::fmt::Display {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let set = ranges
        .lines()
        .map(|l| {
            let (s, e) = l.split_once('-').unwrap();

            s.parse::<usize>().unwrap()..=e.parse().unwrap()
        })
        .fold(RangeSet::new(), |mut acc, r| {
            acc.insert(r);
            acc
        });

    set.size()
}

fn main() {
    let input = include_str!("../inputs/05.txt");
    //let input = include_str!("../test.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
