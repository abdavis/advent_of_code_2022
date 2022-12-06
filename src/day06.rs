const INPUT: &str = include_str!("inputs/day06.txt");
pub fn run() -> String {
    format!("{}\n{}", both_parts(INPUT, 4), both_parts(INPUT, 14))
}
fn both_parts(input: &str, length: usize) -> usize {
    input
        .as_bytes()
        .windows(length)
        .position(|slice| !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1])))
        .unwrap()
        + length
}
