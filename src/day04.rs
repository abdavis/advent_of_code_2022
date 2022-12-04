const INPUT: &str = include_str!("inputs/day04.txt");
pub fn run() -> String {
    let input = parse_input(INPUT);
    format!("Part 1: {}\nPart 2: {}", part1(&input), part2(&input))
}
fn parse_input(input: &str) -> Vec<(isize, isize, isize, isize)> {
    let mut nums = input
        .split_terminator(&['-', ',', '\n'])
        .map(|s| s.parse().unwrap());
    let mut out = Vec::with_capacity(1000);
    while let (Some(a), Some(b), Some(c), Some(d)) =
        (nums.next(), nums.next(), nums.next(), nums.next())
    {
        out.push((a, b, c, d));
    }
    out
}
fn part1(input: &Vec<(isize, isize, isize, isize)>) -> usize {
    input
        .iter()
        .filter(|(a, b, c, d)| (a <= c && b >= d) || (c <= a && d >= b))
        .count()
}
fn part2(input: &Vec<(isize, isize, isize, isize)>) -> usize {
    input.iter().filter(|(a, b, c, d)| a <= d && c <= b).count()
}
