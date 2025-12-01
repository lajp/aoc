fn part1(input: &str) -> impl std::fmt::Display {
    input
        .trim()
        .split(",")
        .map(|range| {
            let mut ans = 0;

            let (start, end) = range.split_once('-').unwrap();
            let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());

            for id in start..=end {
                let lg = id.ilog10();
                if lg == 0 || lg % 2 != 1 {
                    continue;
                }

                let fst = id / 10usize.pow(lg.div_ceil(2));
                let snd = id % 10usize.pow(lg.div_ceil(2));

                if fst == snd {
                    ans += id;
                }
            }

            ans
        })
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    input
        .trim()
        .split(",")
        .map(|range| {
            let mut ans = 0;

            let (start, end) = range.split_once('-').unwrap();

            let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());

            'outer: for id in start..=end {
                let lg = id.ilog10();
                if lg == 0 {
                    continue;
                }

                for n in 1..=(lg.div_ceil(2)) {
                    if (lg + 1) % n != 0 || n * 2 > (lg + 1) {
                        continue;
                    }

                    let orig = id % 10usize.pow(n);
                    let mut cand = orig;

                    for _ in 0..(lg / n) {
                        cand *= 10usize.pow(n);
                        cand += orig;
                    }

                    if cand == id {
                        ans += id;
                        continue 'outer;
                    }
                }
            }

            ans
        })
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/02.txt");
    //let input = include_str!("../test.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
