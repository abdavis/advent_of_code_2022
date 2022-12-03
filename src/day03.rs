const INPUT: &str = include_str!("inputs/day03.txt");
use std::collections::HashSet;
pub fn run() -> String {
    format!("Part 1: {}\n", part1(INPUT)) + &format!("Part2: {}", part2(INPUT))
}
trait ToNum {
    fn num(&self) -> u32;
}
impl ToNum for char {
    fn num(&self) -> u32 {
        match self {
            c @ 'a'..='z' => c.to_digit(36).unwrap() - 9,
            c @ 'A'..='Z' => c.to_digit(36).unwrap() - 9 + 26,
            c => panic!("\"{c}\" can't be turned into a number"),
        }
    }
}
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut leftset = HashSet::new();
            let mut rightset = HashSet::new();
            for (lc, rc) in line.chars().take(line.len() / 2).zip(line.chars().rev()) {
                leftset.insert(lc);
                rightset.insert(rc);
            }
            leftset.intersection(&rightset).next().unwrap().num()
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut lines = input.lines();
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        sum += a
            .chars()
            .collect::<HashSet<_>>()
            .intersection(
                &b.chars()
                    .collect::<HashSet<_>>()
                    .intersection(&c.chars().collect())
                    .cloned()
                    .collect(),
            )
            .next()
            .unwrap()
            .num()
    }
    sum
}
