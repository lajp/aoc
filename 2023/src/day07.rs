use std::{cmp::Ordering, fmt::Display};

use itertools::Itertools;

#[derive(Eq, PartialEq)]
struct Hand {
    pub cards: Vec<u32>,
    pub bid: u64,
}

fn cmp_eq(this: &Hand, other: &Hand) -> std::cmp::Ordering {
    this.cards
        .iter()
        .zip(other.cards.iter())
        .find_map(|(s, o)| if s != o { Some(s.cmp(o)) } else { None })
        .expect("impossible")
}

fn cmp_pt1(this: &Hand, other: &Hand) -> std::cmp::Ordering {
    let this_counts = this.cards.iter().counts();
    let other_counts = other.cards.iter().counts();
    let this_max_count = *this_counts.values().max().unwrap();
    let other_max_count = *other_counts.values().max().unwrap();

    if other_max_count == 5 {
        if this_max_count == 5 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count == 5 {
        return Ordering::Greater;
    }

    if other_max_count == 4 {
        if this_max_count == 4 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count == 4 {
        return Ordering::Greater;
    }

    if other_counts.keys().count() == 2 {
        if this_counts.keys().count() == 2 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_counts.keys().count() == 2 {
        return Ordering::Greater;
    }

    if other_max_count == 3 {
        if this_max_count == 3 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count == 3 {
        return Ordering::Greater;
    }

    if other_counts.keys().count() == 3 {
        if this_counts.keys().count() == 3 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_counts.keys().count() == 3 {
        return Ordering::Greater;
    }

    if other_max_count == 2 {
        if this_max_count == 2 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count == 2 {
        return Ordering::Greater;
    }

    cmp_eq(this, other)
}

fn cmp_pt2(this: &Hand, other: &Hand) -> std::cmp::Ordering {
    if this.cards == other.cards {
        println!("impossible");
        return Ordering::Equal;
    }

    let this_jokers = this.cards.iter().filter(|c| **c == 1).count();
    let other_jokers = other.cards.iter().filter(|c| **c == 1).count();
    let this_counts = this.cards.iter().filter(|c| **c != 1).counts();
    let other_counts = other.cards.iter().filter(|c| **c != 1).counts();
    let this_max_count = *this_counts.values().max().unwrap_or(&0);
    let other_max_count = *other_counts.values().max().unwrap_or(&0);

    if other_max_count + other_jokers == 5 {
        if this_max_count + this_jokers == 5 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count + this_jokers == 5 {
        return Ordering::Greater;
    }

    if other_max_count + other_jokers == 4 {
        if this_max_count + this_jokers == 4 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count + this_jokers == 4 {
        return Ordering::Greater;
    }

    if other_counts.keys().count() == 2 {
        if this_counts.keys().count() == 2 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_counts.keys().count() == 2 {
        return Ordering::Greater;
    }

    if other_max_count + other_jokers == 3 {
        if this_max_count + this_jokers == 3 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count + this_jokers == 3 {
        return Ordering::Greater;
    }

    if other_counts.keys().count() == 3 {
        if this_counts.keys().count() == 3 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_counts.keys().count() == 3 {
        return Ordering::Greater;
    }

    if other_max_count + other_jokers == 2 {
        if this_max_count + this_jokers == 2 {
            return cmp_eq(this, other);
        }
        return Ordering::Less;
    } else if this_max_count + this_jokers == 2 {
        return Ordering::Greater;
    }

    cmp_eq(this, other)
}

fn part1(input: String) -> impl Display {
    let mut hands = input
        .lines()
        .map(|l| {
            let sp = l.split(' ').collect::<Vec<_>>();
            Hand {
                cards: sp[0]
                    .chars()
                    .map(|c| {
                        if c.is_numeric() {
                            c.to_digit(10).unwrap()
                        } else {
                            match c {
                                'T' => 10,
                                'J' => 11,
                                'Q' => 12,
                                'K' => 13,
                                'A' => 14,
                                _ => unreachable!(),
                            }
                        }
                    })
                    .collect::<Vec<_>>(),
                bid: sp[1].parse::<u64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    hands.sort_by(cmp_pt1);
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum::<u64>()
}

fn part2(input: String) -> impl Display {
    let mut hands = input
        .lines()
        .map(|l| {
            let sp = l.split(' ').collect::<Vec<_>>();
            Hand {
                cards: sp[0]
                    .chars()
                    .map(|c| {
                        if c.is_numeric() {
                            c.to_digit(10).unwrap()
                        } else {
                            match c {
                                'T' => 10,
                                'J' => 1,
                                'Q' => 12,
                                'K' => 13,
                                'A' => 14,
                                _ => unreachable!(),
                            }
                        }
                    })
                    .collect::<Vec<_>>(),
                bid: sp[1].parse::<u64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    hands.sort_by(cmp_pt2);
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum::<u64>()
}

fn main() {
    let input: String = include_str!("../input/07").to_string();
    println!("PART1: {}", part1(input.clone()));
    println!("PART2: {}", part2(input));
}
