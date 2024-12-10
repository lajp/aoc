fn part1(input: &str) -> impl std::fmt::Display {
    let mut disk = Vec::new();

    input.trim().chars().enumerate().for_each(|(i, c)| {
        let d = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            disk.extend(vec![Some(i / 2); d]);
        } else {
            disk.extend(vec![None; d])
        }
    });

    let mut start = 0;
    while disk[start].is_some() {
        start += 1;
    }

    let mut end = disk.len() - 1;
    while disk[end].is_none() {
        end -= 1;
    }

    while start < end {
        disk.swap(start, end);
        while end > 0 && disk[end].is_none() {
            end -= 1;
        }

        while start < end && disk[start].is_some() {
            start += 1;
        }
    }

    disk.into_iter()
        .enumerate()
        .map(|(i, c)| if let Some(f) = c { i * f } else { 0 })
        .sum::<usize>()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let mut space = Vec::new();
    let mut files = Vec::new();

    let mut cur = 0;

    input.trim().chars().enumerate().for_each(|(i, c)| {
        let d = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            files.push((i / 2, cur, d));
        } else {
            space.push((cur, d));
        }

        cur += d;
    });

    for (_, fs, fl) in files.iter_mut().rev() {
        for (s, l) in space.iter_mut() {
            if s < fs && fl <= l {
                *fs = *s;
                *s += *fl;
                *l -= *fl;

                break;
            }
        }
    }

    files
        .into_iter()
        .map(|(id, fs, fl)| (fs..fs + fl).map(|i| i * id).sum::<usize>())
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../inputs/09.txt");
    println!("PART1: {}", part1(input));
    println!("PART2: {}", part2(input));
}
