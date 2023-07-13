const INPUT: &str = include_str!("inputs/day10.txt");
const TEST: &str = include_str!("example_inputs/day10.txt");
pub fn run() -> String {
    let (part_1, part_2) = both_parts(INPUT);
    format!("Part 1: {part_1}\nPart 2:\n{}", to_string(part_2))
}
fn to_string(screen: [[bool; 40]; 6]) -> String {
    let mut out = String::new();
    for r in screen {
        for c in r {
            out += match c {
                true => "█",
                false => " ",
            }
        }
        out += "\n";
    }
    out
}
fn both_parts(input: &str) -> (isize, [[bool; 40]; 6]) {
    let important_cycles = [20, 60, 100, 140, 180, 220];
    let mut lines = input.lines().peekable();
    let mut x: isize = 1;
    let mut cycle: usize = 0;
    let mut sum = 0;
    let mut screen = [[false; 40]; 6];
    let mut wait_time: usize = match lines.peek().unwrap() {
        &"noop" => 1,
        _ => 2,
    };
    loop {
        let hor = cycle % 40;
        if hor as isize >= x - 1 && hor as isize <= x + 1 {
            screen[cycle / 40][hor] = true;
        }

        cycle += 1;
        wait_time -= 1;

        if important_cycles.contains(&cycle) {
            sum += cycle as isize * x;
        } //println!("{cycle} {wait_time} {x}");

        if wait_time == 0 {
            match lines.next() {
                None => break,
                Some("noop") => (),
                Some(line) => {
                    x += line
                        .split_whitespace()
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse::<isize>()
                        .unwrap()
                }
            }
            match lines.peek() {
                None => break,
                Some(&"noop") => wait_time = 1,
                Some(_) => wait_time = 2,
            }
        }
    }

    (sum, screen)
}
