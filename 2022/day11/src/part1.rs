use std::io;

#[derive(Copy, Clone, Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Multiply,
            '/' => Self::Divide,
            _ => unreachable!(),
        }
    }
}

impl Operator {
    fn apply(self, n1: i64, n2: i64) -> i64 {
        match self {
            Operator::Plus => n1 + n2,
            Operator::Minus => n1 - n2,
            Operator::Divide => n1 / n2,
            Operator::Multiply => n1 * n2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum NumOrOld {
    Num(i64),
    Old,
}

impl From<&str> for NumOrOld {
    fn from(value: &str) -> Self {
        match value {
            "old" => Self::Old,
            _ => Self::Num(value.parse().unwrap()),
        }
    }
}

impl NumOrOld {
    fn value(self, old: i64) -> i64 {
        match self {
            NumOrOld::Num(a) => a,
            NumOrOld::Old => old,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: (NumOrOld, Operator, NumOrOld),
    testn: i64,
    throw_to_true: i64,
    throw_to_false: i64,
    inspected: i64,
}

impl Monkey {
    fn new(
        items: Vec<i64>,
        operation: (NumOrOld, Operator, NumOrOld),
        testn: i64,
        throw_to_true: i64,
        throw_to_false: i64,
    ) -> Self {
        Self {
            items,
            operation,
            testn,
            throw_to_true,
            throw_to_false,
            inspected: 0,
        }
    }
}

pub fn run() {
    let mut monkeys = Vec::<Monkey>::new();
    let lines = io::stdin().lines();

    let mut items = Vec::<i64>::new();
    let mut operation = (NumOrOld::Num(0), Operator::Plus, NumOrOld::Num(0));
    let mut testn = 0;
    let mut throw_to_false = -1;
    let mut throw_to_true = -1;

    for line in lines.flatten() {
        if line.is_empty() {
            monkeys.push(Monkey::new(
                items.clone(),
                operation,
                testn,
                throw_to_true,
                throw_to_false,
            ));
            items.clear();
            continue;
        }

        if line.trim().starts_with("Starting items:") {
            items = line.trim()[16..]
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
        } else if line.trim().starts_with("Operation: ") {
            let ss: Vec<&str> = line.trim()[17..].split(' ').collect();
            operation = (
                NumOrOld::from(ss[0]),
                Operator::from(ss[1].chars().next().unwrap()),
                NumOrOld::from(ss[2]),
            );
        } else if line.trim().starts_with("Test: divisible by ") {
            testn = line.trim()[19..].parse().unwrap();
        } else if line.trim().starts_with("If true: throw to monkey ") {
            throw_to_true = line.trim()[25..].parse().unwrap();
        } else if line.trim().starts_with("If false: throw to monkey ") {
            throw_to_false = line.trim()[26..].parse().unwrap();
        }
    }
    monkeys.push(Monkey::new(
        items.clone(),
        operation,
        testn,
        throw_to_true,
        throw_to_false,
    ));

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let mut worry = monkeys[i].operation.1.apply(
                    monkeys[i].operation.0.value(monkeys[i].items[0]),
                    monkeys[i].operation.2.value(monkeys[i].items[0]),
                );
                worry /= 3;

                let throw_to = if worry % monkeys[i].testn == 0 {
                    monkeys[i].throw_to_true as usize
                } else {
                    monkeys[i].throw_to_false as usize
                };

                monkeys[i].inspected += 1;
                monkeys[i].items.remove(0);
                monkeys[throw_to].items.push(worry);
            }
        }
    }

    monkeys.sort_by_key(|m| -m.inspected);
    println!("{}", monkeys[0].inspected * monkeys[1].inspected);
}
