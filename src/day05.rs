use std::collections::VecDeque;

const INPUT: &str = include_str!("inputs/day05.txt");
pub fn run() -> String {
    let (crates, instructions) = parse_input::<9>(INPUT);
    part1(crates.clone(), &instructions) + "\n" + &part2(crates, &instructions)
}
fn parse_input<const NUM_STACKS: usize>(
    input: &str,
) -> ([VecDeque<char>; NUM_STACKS], Vec<(u8, u8, u8)>) {
    let (crates_str, instructions_str) = input.split_once("\n\n").unwrap();
    let mut crates = std::array::from_fn(|_| VecDeque::new());
    for line in crates_str.lines() {
        for (n, c) in line.chars().skip(1).step_by(4).enumerate() {
            match c {
                c @ 'A'..='Z' => crates[n].push_back(c),
                _ => (),
            }
        }
    }
    let mut nums = instructions_str
        .split_ascii_whitespace()
        .skip(1)
        .step_by(2)
        .map(|num| num.parse().unwrap());
    let mut instructions = Vec::new();
    while let (Some(a), Some(b), Some(c)) = (nums.next(), nums.next(), nums.next()) {
        instructions.push((a, b, c));
    }
    (crates, instructions)
}
fn part1<const NUM_STACKS: usize>(
    mut crates: [VecDeque<char>; NUM_STACKS],
    instructions: &Vec<(u8, u8, u8)>,
) -> String {
    for (moves, from, to) in instructions {
        for _ in 0..*moves {
            let val = crates[*from as usize - 1].pop_front().unwrap();
            crates[*to as usize - 1].push_front(val);
        }
    }
    crates
        .iter_mut()
        .map(|que| que.pop_front().unwrap())
        .collect()
}
fn part2<const NUM_STACKS: usize>(
    mut crates: [VecDeque<char>; NUM_STACKS],
    instructions: &Vec<(u8, u8, u8)>,
) -> String {
    for (moves, from, to) in instructions {
        let (moves, from, to) = (*moves as usize, *from as usize - 1, *to as usize - 1);
        let mut other = crates[from].split_off(moves);
        std::mem::swap(&mut other, &mut crates[from]);
        for container in other.iter().rev() {
            crates[to].push_front(*container);
        }
    }
    crates
        .iter_mut()
        .map(|que| que.pop_front().unwrap())
        .collect()
}
