const INPUT: &str = include_str!("inputs/day11.txt");

pub fn run() -> String {
    let mut monkeys: Vec<Monkey> = INPUT.split("\n\n").map(|input| input.into()).collect();
    let mut monkeys_2 = monkeys.clone();
    part_1(20, &mut monkeys);
    part_2(10_000, &mut monkeys_2);
    format!(
        "Part 1: {}\nPart 2: {}",
        max_count(&monkeys),
        max_count(&monkeys_2)
    )
}
fn max_count(monkeys: &Vec<Monkey>) -> usize {
    let mut max = 0;
    let mut smaller = 0;
    for monkey in monkeys {
        if monkey.count >= max {
            smaller = max;
            max = monkey.count;
        } else if monkey.count >= smaller {
            smaller = monkey.count;
        }
    }
    max * smaller
}
fn part_1(rounds: usize, monkeys: &mut Vec<Monkey>) {
    for _ in 0..rounds {
        for n in 0..monkeys.len() {
            monkeys[n].count += monkeys[n].queue.len();
            let mut new_vec = Vec::new();
            std::mem::swap(&mut new_vec, &mut monkeys[n].queue);
            for mut item in new_vec {
                use Operation::*;
                match monkeys[n].operation {
                    Mul(n) => item *= n,
                    MulSelf => item *= item,
                    Plus(n) => item += n,
                    PlusSelf => item += item,
                }
                item /= 3;
                if item % monkeys[n].modulus == 0 {
                    let idx = monkeys[n].true_throw;
                    monkeys[idx].queue.push(item);
                } else {
                    let idx = monkeys[n].false_throw;
                    monkeys[idx].queue.push(item);
                }
            }
        }
    }
}
fn part_2(rounds: usize, monkeys: &mut Vec<Monkey>) {
    let mut global_modulus = 1;
    for monkey in monkeys.iter() {
        global_modulus *= monkey.modulus;
    }
    for _ in 0..rounds {
        for n in 0..monkeys.len() {
            monkeys[n].count += monkeys[n].queue.len();
            let mut new_vec = Vec::new();
            std::mem::swap(&mut new_vec, &mut monkeys[n].queue);
            for mut item in new_vec {
                use Operation::*;
                match monkeys[n].operation {
                    Mul(v) => item *= v % global_modulus,
                    MulSelf => item *= item % global_modulus,
                    Plus(v) => item += v % global_modulus,
                    PlusSelf => item *= 2 % global_modulus,
                }
                item %= global_modulus;
                if item % monkeys[n].modulus == 0 {
                    let idx = monkeys[n].true_throw;
                    monkeys[idx].queue.push(item);
                } else {
                    let idx = monkeys[n].false_throw;
                    monkeys[idx].queue.push(item);
                }
            }
        }
    }
}
#[derive(Clone)]
struct Monkey {
    count: usize,
    queue: Vec<u64>,
    operation: Operation,
    modulus: u64,
    true_throw: usize,
    false_throw: usize,
}
#[derive(Clone)]
enum Operation {
    Plus(u64),
    PlusSelf,
    Mul(u64),
    MulSelf,
}
impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut splits = input.lines().skip(1).flat_map(|l| l.split(": ").skip(1));
        let queue = splits
            .next()
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        use Operation::*;
        let mut operator_parts = splits.next().unwrap().split_ascii_whitespace().skip(3);
        let operation = match (
            operator_parts.next().unwrap(),
            operator_parts.next().unwrap(),
        ) {
            ("*", "old") => MulSelf,
            ("*", n) => Mul(n.parse().unwrap()),
            ("+", "old") => PlusSelf,
            ("+", n) => Plus(n.parse().unwrap()),
            _ => panic!("bad operation input"),
        };
        let modulus = splits
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let true_throw = splits
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let false_throw = splits
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            count: 0,
            queue,
            operation,
            modulus,
            true_throw,
            false_throw,
        }
    }
}
const TEST: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
