use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};
const INPUT: &str = include_str!("inputs/day23.txt");

pub fn run() -> String {
    let mut field: Field = INPUT.into();
    let (first, second) = field.solve();
    format!("Part 1: {first}\n Part 2: {second}")
}

struct Field {
    field: HashSet<Spot>,
    direction: Direction,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xmin, ymin, xmax, ymax) = self.bounding_box();
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if self.field.contains(&Spot { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Field {
    fn solve(&mut self) -> (usize, usize) {
        for _ in 0..10 {
            self.step();
        }
        (self.count_empty(), {
            let mut c = 11;
            while !self.step() {
                c += 1
            }
            c
        })
    }

    fn count_empty(&self) -> usize {
        let (xmin, ymin, xmax, ymax) = self.bounding_box();
        ((xmax - xmin + 1) * (ymax - ymin + 1)) as usize - self.field.len()
    }
    fn bounding_box(&self) -> (isize, isize, isize, isize) {
        self.field.iter().fold(
            (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
            |(xmin, ymin, xmax, ymax), s| {
                (xmin.min(s.x), ymin.min(s.y), xmax.max(s.x), ymax.max(s.y))
            },
        )
    }
    fn step(&mut self) -> bool {
        let mut target_spots: HashMap<Spot, Vec<Spot>> = HashMap::new();

        for spot in self.field.iter() {
            match (
                &self.direction,
                self.north_occupied(spot),
                self.south_occupied(spot),
                self.west_occupied(spot),
                self.east_occupied(spot),
            ) {
                (_, false, false, false, false) | (_, true, true, true, true) => {
                    target_spots.insert(*spot, vec![]);
                }
                // When north is first
                (Direction::N, false, _, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::N, _, false, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::N, _, _, false, _) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::N, _, _, _, false) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                //when south is first
                (Direction::S, _, false, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::S, _, _, false, _) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::S, _, _, _, false) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::S, false, _, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                // when west is first
                (Direction::W, _, _, false, _) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::W, _, _, _, false) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::W, false, _, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::W, _, false, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                //when east is first
                (Direction::E, _, _, _, false) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::E, false, _, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::E, _, false, _, _) => {
                    target_spots
                        .entry(Spot {
                            y: spot.y + 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
                (Direction::E, _, _, false, _) => {
                    target_spots
                        .entry(Spot {
                            x: spot.x - 1,
                            ..*spot
                        })
                        .and_modify(|v| v.push(*spot))
                        .or_insert(vec![*spot]);
                }
            }
        }

        self.direction.next();

        let mut new_field = HashSet::new();
        for (k, v) in target_spots {
            if v.len() <= 1 {
                new_field.insert(k);
            } else {
                for v in v {
                    new_field.insert(v);
                }
            }
        }
        let complete = self.field == new_field;
        self.field = new_field;

        complete
        // println!("{self}");
    }
    fn north_occupied(&self, &Spot { x, y }: &Spot) -> bool {
        self.field.contains(&Spot { x: x - 1, y: y - 1 })
            || self.field.contains(&Spot { x, y: y - 1 })
            || self.field.contains(&Spot { x: x + 1, y: y - 1 })
    }
    fn south_occupied(&self, &Spot { x, y }: &Spot) -> bool {
        self.field.contains(&Spot { x: x - 1, y: y + 1 })
            || self.field.contains(&Spot { x, y: y + 1 })
            || self.field.contains(&Spot { x: x + 1, y: y + 1 })
    }
    fn west_occupied(&self, &Spot { x, y }: &Spot) -> bool {
        self.field.contains(&Spot { x: x - 1, y: y - 1 })
            || self.field.contains(&Spot { x: x - 1, y })
            || self.field.contains(&Spot { x: x - 1, y: y + 1 })
    }
    fn east_occupied(&self, &Spot { x, y }: &Spot) -> bool {
        self.field.contains(&Spot { x: x + 1, y: y - 1 })
            || self.field.contains(&Spot { x: x + 1, y })
            || self.field.contains(&Spot { x: x + 1, y: y + 1 })
    }
}

enum Direction {
    N,
    S,
    W,
    E,
}
impl Default for Direction {
    fn default() -> Self {
        Self::N
    }
}
impl Direction {
    fn next(&mut self) {
        match self {
            Self::N => *self = Self::S,
            Self::S => *self = Self::W,
            Self::W => *self = Self::E,
            Self::E => *self = Self::N,
        }
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let spots = value.lines().enumerate().flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| Spot {
                    x: col as isize,
                    y: row as isize,
                })
        });

        Self {
            field: spots.collect(),
            direction: Default::default(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Spot {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    const TEST: &str = include_str!("example_inputs/day23.txt");
    use super::*;

    #[test]
    fn run() {
        let mut test: Field = TEST.into();
        assert_eq!((110, 20), test.solve())
    }
}
