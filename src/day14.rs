use core::cmp::{max, min};
use std::collections::HashMap;
const INPUT: &str = include_str!("inputs/day14.txt");
pub fn run() -> String {
    todo!()
}

struct Cave {
    source: Point,
    min_bound: Point,
    max_bound: Point,
    field: HashMap<Point, Space>,
}
impl Cave {
    fn pour_sand(&mut self) -> usize {}
}
impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let mut field = HashMap::new();
        let mut min_bound = Point {
            x: usize::MAX,
            y: usize::MAX,
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
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
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
