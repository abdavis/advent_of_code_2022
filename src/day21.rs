use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn run() -> String {
    let monkeys: AllMonkeys = INPUT.into();

    format!(
        "{}\n{}",
        monkeys.calculate("root").0,
        monkeys.calc_expected()
    )
}
#[derive(Debug)]
struct AllMonkeys<'a>(HashMap<&'a str, RefCell<Monkey<'a>>>);
impl<'a> AllMonkeys<'a> {
    fn calculate(&self, name: &str) -> (i64, bool) {
        let mut monkey = self.0[name].borrow_mut();
        match *monkey {
            Monkey::Val(num) => (num, if name == "humn" { true } else { false }),
            Monkey::Job {
                ref mut left,
                ref mut right,
                ref mut calculated,
                ref operand,
            } => {
                if let Some(val) = calculated {
                    return (*val, false);
                }
                let left_val = self.calculate(left.0);
                let right_val = self.calculate(right.0);
                let val = match operand {
                    Operand::Plus => left_val.0 + right_val.0,
                    Operand::Minus => left_val.0 - right_val.0,
                    Operand::Mult => left_val.0 * right_val.0,
                    Operand::Div => left_val.0 / right_val.0,
                };
                *calculated = Some(val);
                left.1 = left_val.1;
                right.1 = right_val.1;
                (val, left.1 || right.1)
            }
        }
    }
    fn calc_expected(&self) -> i64 {
        self.calculate("root");

        match &*self.0.get("root").unwrap().borrow() {
            Monkey::Job {
                left: (humn_name, true),
                right: (monkey_name, false),
                ..
            }
            | Monkey::Job {
                left: (monkey_name, false),
                right: (humn_name, true),
                ..
            } => {
                if let Monkey::Job {
                    calculated: Some(val),
                    ..
                }
                | Monkey::Val(val) = *self.0.get(monkey_name).unwrap().borrow()
                {
                    self.expected_recurse(humn_name, val)
                } else {
                    panic!("couldn't get value for monkey child")
                }
            }
            _ => {
                panic!("root node has no humn child")
            }
        }
    }
    fn expected_recurse(&self, name: &str, expected: i64) -> i64 {
        match &*self.0[name].borrow() {
            Monkey::Val(_) => {
                if name == "humn" {
                    expected
                } else {
                    panic!("bottom node is not humn")
                }
            }
            Monkey::Job {
                left: (humn_name, true),
                right: (monk_name, false),
                operand,
                ..
            } => match operand {
                Operand::Plus => {
                    self.expected_recurse(humn_name, expected - self.calculate(monk_name).0)
                }
                Operand::Minus => {
                    self.expected_recurse(humn_name, expected + self.calculate(monk_name).0)
                }
                Operand::Mult => {
                    self.expected_recurse(humn_name, expected / self.calculate(monk_name).0)
                }
                Operand::Div => {
                    self.expected_recurse(humn_name, expected * self.calculate(monk_name).0)
                }
            },

            Monkey::Job {
                left: (monk_name, false),
                right: (humn_name, true),
                operand,
                ..
            } => match operand {
                Operand::Plus => {
                    self.expected_recurse(humn_name, expected - self.calculate(monk_name).0)
                }
                Operand::Minus => {
                    self.expected_recurse(humn_name, self.calculate(monk_name).0 - expected)
                }
                Operand::Mult => {
                    self.expected_recurse(humn_name, expected / self.calculate(monk_name).0)
                }
                Operand::Div => {
                    self.expected_recurse(humn_name, self.calculate(monk_name).0 / expected)
                }
            },
            _ => panic!(),
        }
    }
}
impl<'a> From<&'a str> for AllMonkeys<'a> {
    fn from(value: &'a str) -> Self {
        Self(
            value
                .lines()
                .map(|l| {
                    let (name, content) = l.split_once(": ").unwrap();
                    (name, RefCell::new(content.into()))
                })
                .collect(),
        )
    }
}
#[derive(Debug)]
enum Monkey<'a> {
    Val(i64),
    Job {
        left: (&'a str, bool),
        right: (&'a str, bool),
        calculated: Option<i64>,
        operand: Operand,
    },
}
impl<'a> From<&'a str> for Monkey<'a> {
    fn from(value: &'a str) -> Self {
        match value.parse() {
            Ok(n) => Self::Val(n),
            Err(_) => {
                let mut parts = value.split_whitespace();
                Self::Job {
                    left: (parts.next().unwrap(), false),
                    operand: parts.next().unwrap().into(),
                    right: (parts.next().unwrap(), false),
                    calculated: None,
                }
            }
        }
    }
}
#[derive(Debug)]
enum Operand {
    Plus,
    Minus,
    Mult,
    Div,
}
impl From<&str> for Operand {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mult,
            "/" => Self::Div,
            _ => panic!("bad operand: {value}"),
        }
    }
}

const INPUT: &str = include_str!("inputs/day21.txt");
