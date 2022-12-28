const INPUT: &str = include_str!("inputs/day17.txt");
const EXAMPLE: &str = include_str!("example_inputs/day17.txt");
pub fn run() -> String {
    let shapes = Shape::iter();
    let drafts = Draft::iter(EXAMPLE);
    let mut chamber = Chamber::default();
    format!("foo")
}

struct Chamber(Vec<u8>);
impl Chamber {
    fn drop_rocks(
        &mut self,
        num_rocks: usize,
        mut shapes: impl Iterator<Item = Shape>,
        mut drafts: impl Iterator<Item = Draft>,
    ) -> usize {
        for _ in 0..num_rocks {
            let mut y_pos = self.0.len() + 3;
            let mut shape = shapes.next().unwrap();
            loop {
                //apply draft
            }
        }
        self.0.len()
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
