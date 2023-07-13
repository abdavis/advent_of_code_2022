use std::collections::HashSet;
const INPUT: &str = include_str!("inputs/day09.txt");
pub fn run() -> String {
    let instructions = INPUT.lines().map(|l| l.into()).collect();

    let mut rope = Rope::new(1);
    let mut long_rope = Rope::new(9);

    format!(
        "Part 1: {}\nPart 2: {}",
        rope.process_steps(&instructions),
        long_rope.process_steps(&instructions)
    )
}
struct Rope {
    head: (isize, isize),
    tail: Tail,
}
struct Tail {
    pos: (isize, isize),
    tail: Option<Box<Tail>>,
}
impl Rope {
    fn process_steps(&mut self, instructions: &Vec<Instruction>) -> usize {
        let mut set = HashSet::new();
        set.insert((0, 0));

        for ins in instructions {
            for _ in 0..ins.steps {
                set.insert(self.move_rope(&ins.direction));
            }
        }

        set.len()
    }
    fn move_rope(&mut self, direction: &Direction) -> (isize, isize) {
        use Direction::*;
        match direction {
            Up => self.head.1 += 1,
            Down => self.head.1 -= 1,
            Right => self.head.0 += 1,
            Left => self.head.0 -= 1,
        }
        self.tail.move_tail(self.head)
    }
    fn new(length: usize) -> Self {
        Self {
            head: (0, 0),
            tail: *Tail::new(length).unwrap(),
        }
    }
}
impl Tail {
    fn new(length: usize) -> Option<Box<Tail>> {
        if length == 0 {
            None
        } else {
            Some(Box::new(Tail {
                pos: (0, 0),
                tail: Self::new(length - 1),
            }))
        }
    }
    fn move_tail(&mut self, pos: (isize, isize)) -> (isize, isize) {
        if pos.0.abs_diff(self.pos.0) > 1 || pos.1.abs_diff(self.pos.1) > 1 {
            self.pos.0 += (pos.0 - self.pos.0).clamp(-1, 1);
            self.pos.1 += (pos.1 - self.pos.1).clamp(-1, 1);
        }
        match &mut self.tail {
            None => self.pos,
            Some(tail) => tail.move_tail(self.pos),
        }
    }
}

struct Instruction {
    direction: Direction,
    steps: isize,
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        use Direction::*;
        let mut parts = input.split_ascii_whitespace();
        match (
            parts.next().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ) {
            ("D", steps) => Self {
                steps,
                direction: Down,
            },
            ("U", steps) => Self {
                steps,
                direction: Up,
            },
            ("L", steps) => Self {
                steps,
                direction: Left,
            },
            ("R", steps) => Self {
                steps,
                direction: Right,
            },
            _ => panic!("bad input"),
        }
    }
}
