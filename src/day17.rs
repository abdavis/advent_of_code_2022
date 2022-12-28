use core::iter::Peekable;

use core::hash::Hash;
use std::fmt::Display;

const INPUT: &str = include_str!("inputs/day17.txt");
const EXAMPLE: &str = include_str!("example_inputs/day17.txt");
pub fn run() -> String {
    format!(
        "{}\n{}",
        Chamber::default().get_height(
            Shape::iter().peekable(),
            Draft::iter(INPUT).peekable(),
            2022
        ),
        Chamber::default().get_height(
            Shape::iter().peekable(),
            Draft::iter(INPUT).peekable(),
            1_000_000_000_000
        )
    )
}

struct Value {
    rocks_dropped: usize,
    height: usize,
}

#[derive(Hash, PartialEq, Eq)]
struct Key {
    chamber_state: Vec<u8>,
    shape: u8,
    draft: u16,
}
#[derive(Clone)]
struct Chamber {
    chamber_state: Vec<u8>,
    cut_height: usize,
}

impl Chamber {
    fn get_height(
        &mut self,
        mut shapes: Peekable<impl Iterator<Item = (u8, Shape)>>,
        mut drafts: Peekable<impl Iterator<Item = (u16, Draft)>>,
        num_rocks: usize,
    ) -> usize {
        use std::collections::HashMap;
        let mut map = HashMap::<Key, Value>::new();

        for n in 0..num_rocks {
            match map.entry(Key {
                chamber_state: self.chamber_state.clone(),
                shape: shapes.peek().unwrap().0,
                draft: drafts.peek().unwrap().0,
            }) {
                std::collections::hash_map::Entry::Occupied(occ) => {
                    let val = occ.get();
                    let cycle_length = n - val.rocks_dropped;
                    let rocks_left = num_rocks - n;
                    let height_gain = self.height() - val.height;
                    let cycles = rocks_left / cycle_length;
                    self.cut_height += cycles * height_gain;
                    return self.get_height(
                        shapes,
                        drafts,
                        num_rocks - n - (cycles * cycle_length),
                    );
                }
                std::collections::hash_map::Entry::Vacant(mut vac) => {
                    vac.insert(Value {
                        rocks_dropped: n,
                        height: self.height(),
                    });
                    self.drop_rock(shapes.next().unwrap().1, &mut drafts);
                }
            }
        }
        self.height()
    }
    fn drop_rock(&mut self, mut shape: Shape, drafts: &mut impl Iterator<Item = (u16, Draft)>) {
        let mut y_pos = self.chamber_state.len() + 3;
        loop {
            if let Some(new_shape) = shape.apply_draft(&drafts.next().unwrap().1) {
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
        self.reduce();
    }
    fn reduce(&mut self) {
        use std::collections::VecDeque;
        //breadth first search
        let mut searched = Vec::<u8>::new();
        let mut queue = VecDeque::new();
        for n in 0..7 {
            queue.push_back((n, 0));
        }
        while let Some((x, y)) = queue.pop_front() {
            if y < self.chamber_state.len()
                && self.chamber_state[self.chamber_state.len() - y - 1] & (1 << x) == 0
            {
                if searched.len() == y {
                    searched.push(0);
                }
                if searched[y] & (1 << x) == 0 {
                    searched[y] |= 1 << x;
                    queue.push_back((x, y + 1));
                    if x > 0 {
                        queue.push_back((x - 1, y));
                    }
                    if x < 6 {
                        queue.push_back((x + 1, y));
                    }
                }
            }
        }
        //collect result of search into new chamber.
        self.cut_height += self.chamber_state.len() - searched.len();
        self.chamber_state = searched.iter().map(|v| !v).rev().collect();
    }
    fn intersecting(&self, y_pos: usize, shape: &Shape) -> bool {
        for (i, row) in shape.0.iter().enumerate() {
            if let Some(casm_row) = self.chamber_state.get(i + y_pos) {
                if row & casm_row != 0 {
                    return true;
                }
            }
        }
        false
    }
    fn insert(&mut self, y_pos: usize, shape: &Shape) {
        for (i, row) in shape.0.iter().enumerate() {
            if let Some(casm_row) = self.chamber_state.get_mut(y_pos + i) {
                *casm_row |= *row;
            } else {
                self.chamber_state.push(*row);
            }
        }
    }
    fn height(&self) -> usize {
        self.cut_height + self.chamber_state.len()
    }
}
impl Default for Chamber {
    fn default() -> Self {
        Self {
            chamber_state: vec![],
            cut_height: 0,
        }
    }
}

#[derive(Clone)]
struct Shape(Vec<u8>);
impl Shape {
    //return a repeating iterator of shapes in the right order
    fn iter() -> impl Iterator<Item = (u8, Self)> {
        let shapes = [
            Self(vec![0b00011110]),
            Self(vec![0b00001000, 0b00011100, 0b00001000]),
            Self(vec![0b00011100, 0b00000100, 0b00000100]),
            Self(vec![0b00010000, 0b00010000, 0b00010000, 0b00010000]),
            Self(vec![0b00011000, 0b00011000]),
        ];
        (0..).flat_map(move |_| {
            shapes
                .clone()
                .into_iter()
                .enumerate()
                .map(|(n, foo)| (n as u8, foo))
        })
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
    fn iter(input: &str) -> impl Iterator<Item = (u16, Self)> + '_ {
        (0..).flat_map(|_| {
            input
                .chars()
                .map(|c| match c {
                    '<' => Self::Left,
                    '>' => Self::Right,
                    _ => panic!("bad input"),
                })
                .enumerate()
                .map(|(n, foo)| (n as u16, foo))
        })
    }
}
