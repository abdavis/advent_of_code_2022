const INPUT: &str = include_str!("inputs/day03.txt");
pub fn run() -> String {
    format!("Part 1: {}\n", part1(INPUT)) + &format!("Part 2: {}", part2(INPUT))
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
struct Bitmap(u64);
impl FromIterator<char> for Bitmap {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut acc = 0;
        for c in iter {
            acc |= 1 << c.num();
        }
        Self(acc)
    }
}
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let left: Bitmap = line.chars().take(line.len() / 2).collect();
            let right: Bitmap = line.chars().rev().take(line.len() / 2).collect();
            (left.0 & right.0).trailing_zeros()
        })
        .sum()
}
fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut lines = input.lines();
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let a: Bitmap = a.chars().collect();
        let b: Bitmap = b.chars().collect();
        let c: Bitmap = c.chars().collect();
        sum += (a.0 & b.0 & c.0).trailing_zeros()
    }
    sum
}
