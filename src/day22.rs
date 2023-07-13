use core::panic;

pub fn run() -> String {
    let val = Instructions::from(INPUT.lines().last().unwrap());
    //    format!("{val:?}")
    "Day 22 unfinished".into()
}

const INPUT: &str = include_str!("inputs/day22.txt");
const TEST: &str = include_str!("example_inputs/day22.txt");

struct Square<const N: usize>([[Space; N]; N]);

enum Space {
    Empty,
    Wall,
}

#[derive(Debug)]
enum Instruc {
    R,
    L,
    Move(u32),
}

#[derive(Debug)]
struct Instructions(Vec<Instruc>);

impl From<&str> for Instructions {
    fn from(value: &str) -> Self {
        let mut out = vec![];
        for val in value.split_inclusive(['L', 'R']) {
            let mut count = 0;
            let mut rotation = None;
            for c in val.chars() {
                match c {
                    c @ '0'..='9' => {
                        count *= 10;
                        count += c.to_digit(10).unwrap();
                    }
                    'L' => rotation = Some(Instruc::L),
                    'R' => rotation = Some(Instruc::R),
                    _ => panic!("{c} is not a valid instruction"),
                }
            }
            out.push(Instruc::Move(count));
            match rotation {
                None => {}
                Some(n) => out.push(n),
            }
        }
        Self(out)
    }
}
