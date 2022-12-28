use std::fmt::Display;

const INPUT: &str = include_str!("inputs/day17.txt");
const EXAMPLE: &str = include_str!("example_inputs/day17.txt");
pub fn run() -> String {
    format!(
        "{}",
        Chamber::get_height(INPUT, 2022) //chamber.drop_rocks(1_000_000_000_000 - 2022, &mut shapes, &mut drafts)
    )
}

struct Value {
    iterations: usize,
    height: usize,
}

#[derive(Hash, PartialEq, Eq)]
struct Key {
    max_heights: Chamber,
    shape: u8,
    draft: u16,
}
#[derive(Hash, PartialEq, Eq, Clone)]
struct Chamber(Vec<u8>);
impl Chamber {
    fn get_height(input: &str, num_rocks: usize) -> usize {
        let mut drafts = Draft::iter(input);
        let mut shapes = Shape::iter();
        let mut chamber = Chamber::default();
        for _ in 0..num_rocks {
            chamber.drop_rock(shapes.next().unwrap(), &mut drafts);
        }
        chamber.0.len()
    }
    fn drop_rock(&mut self, mut shape: Shape, drafts: &mut impl Iterator<Item = Draft>) {
        let mut y_pos = self.0.len() + 3;
        loop {
            if let Some(new_shape) = shape.apply_draft(&drafts.next().unwrap()) {
                if !self.intersecting(y_pos, &new_shape) {
                    shape = new_shape;
                }
            }

            if y_pos > 0 && !self.intersecting(y_pos - 1, &shape) {
                y_pos -= 1;
            } else {
                self.insert(y_pos, &shape);
                break;
            }
        }
    }
    fn intersecting(&self, y_pos: usize, shape: &Shape) -> bool {
        for (i, row) in shape.0.iter().enumerate() {
            if let Some(casm_row) = self.0.get(i + y_pos) {
                if row & casm_row != 0 {
                    return true;
                }
            }
        }
        false
    }
    fn insert(&mut self, y_pos: usize, shape: &Shape) {
        for (i, row) in shape.0.iter().enumerate() {
            if let Some(casm_row) = self.0.get_mut(y_pos + i) {
                *casm_row |= *row;
            } else {
                self.0.push(*row);
            }
        }
    }
}
impl Default for Chamber {
    fn default() -> Self {
        Self(vec![])
    }
}

#[derive(Clone)]
struct Shape(Vec<u8>);
impl Shape {
    //return a repeating iterator of shapes in the right order
    fn iter() -> impl Iterator<Item = Self> {
        let shapes = [
            Self(vec![0b00011110]),
            Self(vec![0b00001000, 0b00011100, 0b00001000]),
            Self(vec![0b00011100, 0b00000100, 0b00000100]),
            Self(vec![0b00010000, 0b00010000, 0b00010000, 0b00010000]),
            Self(vec![0b00011000, 0b00011000]),
        ];
        (0..).flat_map(move |_| shapes.clone().into_iter())
    }
    fn apply_draft(&self, draft: &Draft) -> Option<Self> {
        let mut moved = self.clone();
        for row in &mut moved.0 {
            match draft {
                Draft::Left => {
                    if *row >= 1 << 6 {
                        return None;
                    }
                    *row = *row << 1;
                }
                Draft::Right => {
                    if *row % 2 == 1 {
                        return None;
                    }
                    *row = *row >> 1;
                }
            }
        }
        Some(moved)
    }
}
impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter().rev() {
            writeln!(f, "{row:07b}")?;
        }
        write!(f, "")
    }
}
enum Draft {
    Left,
    Right,
}
impl Draft {
    fn iter(input: &str) -> impl Iterator<Item = Self> + '_ {
        (0..).flat_map(|_| {
            input.chars().map(|c| match c {
                '<' => Self::Left,
                '>' => Self::Right,
                _ => panic!("bad input"),
            })
        })
    }
}
