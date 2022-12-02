const INPUT: &str = include_str!("inputs/day02.txt");

pub fn run() -> String {
    format!("Part one: {}\nPart two: {}", part1(INPUT), part2(INPUT))
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| match l {
            "A X" => 1 + 3,
            "B X" => 1,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2,
            "A Z" => 3,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => panic!("\"{l}\" is not a valid input, check inputs/day02.txt"),
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| match l {
            "A X" => 3,
            "B X" => 1,
            "C X" => 2,
            "A Y" => 3 + 1,
            "B Y" => 3 + 2,
            "C Y" => 3 + 3,
            "A Z" => 6 + 2,
            "B Z" => 6 + 3,
            "C Z" => 6 + 1,
            _ => panic!("\"{l}\" is not a valid input, check inputs/day02.txt"),
        })
        .sum()
}
