use std::iter::Peekable;
const INPUT: &str = include_str!("inputs/day13.txt");
const EXAMPLE: &str = include_str!("example_inputs/day13.txt");

pub fn run() -> String {
    let mut packets: Vec<NestedList> = INPUT
        .lines()
        .filter_map(|l| match l {
            "" => None,
            l => Some(l.into()),
        })
        .collect();
    let part_1 = pairs_in_order(&packets);
    insert_divider_packets(&mut packets);
    packets.sort_unstable();
    let part_2 = find_divider_packets(&packets);
    format!("Part 1: {part_1}\nPart 2: {part_2}")
}
fn find_divider_packets(packets: &Vec<NestedList>) -> usize {
    let mut divider_packets = vec![];
    insert_divider_packets(&mut divider_packets);
    let mut out = 1;
    for (n, pack) in packets.iter().enumerate() {
        if *pack == divider_packets[0] {
            out *= n + 1;
        }
        if *pack == divider_packets[1] {
            out *= n + 1;
            break;
        }
    }
    out
}
fn insert_divider_packets(packets: &mut Vec<NestedList>) {
    packets.push(NestedList::List(vec![NestedList::List(vec![
        NestedList::Num(2),
    ])]));
    packets.push(NestedList::List(vec![NestedList::List(vec![
        NestedList::Num(6),
    ])]));
}
fn pairs_in_order(packets: &Vec<NestedList>) -> usize {
    let mut sum = 0;

    for (i, chunk) in packets.chunks(2).enumerate() {
        if chunk[0] < chunk[1] {
            sum += i + 1;
        }
    }

    sum
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum NestedList {
    Num(u32),
    List(Vec<NestedList>),
}
use std::cmp::{Ord, Ordering};
impl From<&str> for NestedList {
    fn from(value: &str) -> Self {
        (&mut value.chars().peekable()).into()
    }
}
impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        match (self, other) {
            (NestedList::Num(l), NestedList::Num(r)) => l.cmp(r),
            (NestedList::List(l), NestedList::List(r)) => {
                let (mut l_items, mut r_items) = (l.iter(), r.iter());
                loop {
                    match (l_items.next(), r_items.next()) {
                        (None, None) => break Ordering::Equal,
                        (Some(_), None) => break Ordering::Greater,
                        (None, Some(_)) => break Ordering::Less,
                        (Some(l_item), Some(r_item)) => match l_item.cmp(r_item) {
                            Ordering::Equal => (),
                            unequal_order => break unequal_order,
                        },
                    }
                }
            }
            (l_num @ NestedList::Num(_), r_list @ NestedList::List(_)) => {
                let l_list = Self::List(vec![l_num.clone()]);
                l_list.cmp(r_list)
            }
            (l_list @ NestedList::List(_), r_num @ NestedList::Num(_)) => {
                let r_list = Self::List(vec![r_num.clone()]);
                l_list.cmp(&r_list)
            }
        }
    }
}
impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<I: Iterator<Item = char>> From<&mut Peekable<I>> for NestedList {
    fn from(value: &mut Peekable<I>) -> Self {
        match value.next().unwrap() {
            c @ '0'..='9' => {
                let mut num = c.to_digit(10).unwrap();
                while let Some(c @ '0'..='9') = value.peek() {
                    num *= 10;
                    num += c.to_digit(10).unwrap();
                    value.next();
                }
                Self::Num(num)
            }
            '[' => {
                let mut list = Vec::new();
                while let Some(c) = value.peek() {
                    match c {
                        ']' => {
                            value.next();
                            break;
                        }
                        ',' => {
                            value.next();
                        }
                        '0'..='9' | '[' => list.push(value.into()),
                        c => panic!("parsing error! expected to peek ] , or num but found {c}"),
                    }
                }
                Self::List(list)
            }
            c => panic!("parsing error! expected next char to be num or [ but found {c}"),
        }
    }
}
