use core::cmp::{max, min};
use std::collections::HashMap;
const INPUT: &str = include_str!("inputs/day14.txt");
const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
pub fn run() -> String {
    let mut cave: Cave = INPUT.into();
    let part_1 = cave.pour_sand(false);

    let part_2 = part_1 + cave.pour_sand(true);
    format!("Part 1: {part_1}\nPart 2: {part_2}")
}

struct Cave {
    source: Point,
    min_bound: Point,
    max_bound: Point,
    field: HashMap<Point, Space>,
}
impl Cave {
    fn pour_sand(&mut self, floor: bool) -> usize {
        let mut count = 0;
        'outer: loop {
            let mut grain = self.source;
            loop {
                if !floor && !self.check_bounds(grain) {
                    break 'outer;
                }
                if floor && grain.y > self.max_bound.y {
                    count += 1;
                    self.field.insert(grain, Space::Sand);
                    self.expand_width(grain);
                    break;
                }
                if !self.field.contains_key(&Point {
                    x: grain.x,
                    y: grain.y + 1,
                }) {
                    grain = Point {
                        x: grain.x,
                        y: grain.y + 1,
                    };
                } else if !self.field.contains_key(&Point {
                    x: grain.x - 1,
                    y: grain.y + 1,
                }) {
                    grain = Point {
                        x: grain.x - 1,
                        y: grain.y + 1,
                    };
                } else if !self.field.contains_key(&Point {
                    x: grain.x + 1,
                    y: grain.y + 1,
                }) {
                    grain = Point {
                        x: grain.x + 1,
                        y: grain.y + 1,
                    };
                } else {
                    count += 1;
                    self.field.insert(grain, Space::Sand);
                    if grain == self.source {
                        break 'outer;
                    }
                    break;
                }
            }
        }
        count
    }
    fn check_bounds(&self, point: Point) -> bool {
        point.x >= self.min_bound.x && point.x <= self.max_bound.x && point.y <= self.max_bound.y
    }
    fn expand_width(&mut self, point: Point) {
        self.min_bound.x = min(self.min_bound.x, point.x);
        self.max_bound.x = max(self.max_bound.x, point.x);
    }
}
impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_bound.y + 1 {
            for x in self.min_bound.x..=self.max_bound.x {
                write!(
                    f,
                    "{}",
                    match self.field.get(&Point { x, y }) {
                        None => ".",
                        Some(Space::Wall) => "#",
                        Some(Space::Sand) => "O",
                    }
                )?
            }
            write!(f, "\n")?
        }
        write!(f, "")
    }
}
impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let mut field = HashMap::new();
        let mut min_bound = Point {
            x: u16::MAX,
            y: u16::MAX,
        };
        let mut max_bound = Point { x: 0, y: 0 };
        for line in input.lines() {
            let mut points = line.split(" -> ").map(|val| Point::from(val));
            let mut last_point = points.next().unwrap();
            last_point.expand_bounds(&mut min_bound, &mut max_bound);
            for next_point in points {
                next_point.expand_bounds(&mut min_bound, &mut max_bound);
                for point in Point::range(last_point, next_point) {
                    field.insert(point, Space::Wall);
                }
                last_point = next_point;
            }
        }
        Self {
            source: Point { x: 500, y: 0 },
            min_bound,
            max_bound,
            field,
        }
    }
}
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u16,
    y: u16,
}
impl Point {
    fn expand_bounds(&self, minimum: &mut Self, maximum: &mut Self) {
        minimum.x = min(self.x, minimum.x);
        minimum.y = min(self.y, minimum.y);
        maximum.x = max(self.x, maximum.x);
        maximum.y = max(self.y, maximum.y);
    }
    fn range(a: Self, b: Self) -> Box<dyn Iterator<Item = Point>> {
        if a.x != b.x {
            Box::new((min(a.x, b.x)..=max(a.x, b.x)).map(move |x| Point { x, y: a.y }))
        } else {
            Box::new((min(a.y, b.y)..=max(a.y, b.y)).map(move |y| Point { x: a.x, y }))
        }
    }
}
impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}
enum Space {
    Wall,
    Sand,
}
