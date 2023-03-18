use std::{
    array::from_fn,
    collections::{BinaryHeap, HashSet},
};
const INPUT: &str = include_str!("inputs/day24.txt");
pub fn run() -> String {
    //let test: Field<4, 6> = TEST.into();
    let field: Field<35, 100> = INPUT.into();
    //instead of writing another heuristic function for a*, it's easier to
    //simply flip the field vertically.
    let flipped = field.vert_flip();

    format!(
        "{}\n{}",
        field.find_path(0),
        field.find_path(flipped.find_path(field.find_path(0)))
    )
}

#[derive(Debug, Clone)]
struct Field<const ROWS: usize, const COLS: usize> {
    start_col: usize,
    end_col: usize,
    blizzards: [[Blizzard; COLS]; ROWS],
}
impl<const ROWS: usize, const COLS: usize> Field<ROWS, COLS> {
    fn vert_flip(&self) -> Self {
        let mut out = self.clone();
        std::mem::swap(&mut out.start_col, &mut out.end_col);
        out.blizzards.reverse();
        for blizzard in out.blizzards.iter_mut().flat_map(|r| r.iter_mut()) {
            match blizzard {
                Blizzard::Up => *blizzard = Blizzard::Down,
                Blizzard::Down => *blizzard = Blizzard::Up,
                _ => (),
            }
        }
        out
    }
    fn find_path(&self, time: usize) -> usize {
        let mut queue = BinaryHeap::new();
        let mut opened = HashSet::new();
        let mut searched = HashSet::new();
        queue.push(QueueNode {
            node: Node::Start { time },
            start_col: self.start_col,
            end_col: self.end_col,
        });
        while let Some(queue_node) = queue.pop() {
            if let Node::Traveling { row, col, time } = queue_node.node {
                if row == ROWS - 1 && col == self.end_col {
                    return time + 1;
                }
            }
            opened.remove(&queue_node.node);

            queue_node
                .node
                .create_children(self, &mut queue, &mut opened, &mut searched);

            searched.insert(queue_node.node);
        }
        unreachable!()
    }
}
impl<const ROWS: usize, const COLS: usize> From<&str> for Field<ROWS, COLS> {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        Self {
            start_col: lines.next().unwrap().find('.').unwrap() - 1,
            blizzards: from_fn(|_| {
                let mut chars = lines.next().unwrap().chars().skip(1);
                from_fn(|_| chars.next().unwrap().into())
            }),
            end_col: lines.next().unwrap().find('.').unwrap() - 1,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Node<const ROWS: usize, const COLS: usize> {
    Start { time: usize },
    Traveling { row: usize, col: usize, time: usize },
}

impl<const ROWS: usize, const COLS: usize> Node<ROWS, COLS> {
    fn cost(&self, start_col: usize, end_col: usize) -> usize {
        match self {
            Self::Start { time } => time + start_col.abs_diff(end_col) + COLS + 1,
            Node::Traveling { row, col, time } => {
                time + row.abs_diff(ROWS) + col.abs_diff(COLS) + 1
            }
        }
    }
    fn create_children(
        &self,
        field: &Field<ROWS, COLS>,
        queue: &mut BinaryHeap<QueueNode<ROWS, COLS>>,
        opened: &mut HashSet<Node<ROWS, COLS>>,
        searched: &mut HashSet<Node<ROWS, COLS>>,
    ) {
        match self {
            Self::Start { time } => {
                queue.push(QueueNode {
                    node: Self::Start { time: time + 1 },
                    start_col: field.start_col,
                    end_col: field.end_col,
                });
                opened.insert(Self::Start { time: time + 1 });
                let traveler = Self::Traveling {
                    row: 0,
                    col: field.start_col,
                    time: time + 1,
                };
                if let (false, false, false, false) = traveler.collisions(field) {
                    queue.push(QueueNode {
                        node: traveler.clone(),
                        start_col: field.start_col,
                        end_col: field.end_col,
                    });
                    opened.insert(traveler);
                }
            }
            Self::Traveling { row, col, time } => {
                let wait = Self::Traveling {
                    row: *row,
                    col: *col,
                    time: time + 1,
                };
                if !searched.contains(&wait) && !opened.contains(&wait) {
                    if let (false, false, false, false) = wait.collisions(field) {
                        queue.push(QueueNode {
                            node: wait.clone(),
                            start_col: field.start_col,
                            end_col: field.end_col,
                        });
                        opened.insert(wait);
                    }
                }
                if *row > 0 {
                    let up = Self::Traveling {
                        row: *row - 1,
                        col: *col,
                        time: time + 1,
                    };
                    if !searched.contains(&up) && !opened.contains(&up) {
                        if let (false, false, false, false) = up.collisions(field) {
                            queue.push(QueueNode {
                                node: up.clone(),
                                start_col: field.start_col,
                                end_col: field.end_col,
                            });
                            opened.insert(up);
                        }
                    }
                }
                if *row < ROWS - 1 {
                    let down = Self::Traveling {
                        row: *row + 1,
                        col: *col,
                        time: time + 1,
                    };
                    if !searched.contains(&down) && !opened.contains(&down) {
                        if let (false, false, false, false) = down.collisions(field) {
                            queue.push(QueueNode {
                                node: down.clone(),
                                start_col: field.start_col,
                                end_col: field.end_col,
                            });
                            opened.insert(down);
                        }
                    }
                }
                if *col > 0 {
                    let left = Self::Traveling {
                        row: *row,
                        col: *col - 1,
                        time: time + 1,
                    };
                    if !searched.contains(&left) && !opened.contains(&left) {
                        if let (false, false, false, false) = left.collisions(field) {
                            queue.push(QueueNode {
                                node: left.clone(),
                                start_col: field.start_col,
                                end_col: field.end_col,
                            });
                            opened.insert(left);
                        }
                    }
                }
                if *col < COLS - 1 {
                    let right = Self::Traveling {
                        row: *row,
                        col: *col + 1,
                        time: time + 1,
                    };
                    if !searched.contains(&right) && !opened.contains(&right) {
                        if let (false, false, false, false) = right.collisions(field) {
                            queue.push(QueueNode {
                                node: right.clone(),
                                start_col: field.start_col,
                                end_col: field.end_col,
                            });
                            opened.insert(right);
                        }
                    }
                }
            }
        }
    }
    fn collisions(&self, Field { blizzards, .. }: &Field<ROWS, COLS>) -> (bool, bool, bool, bool) {
        match self {
            Node::Start { .. } => (false, false, false, false),
            Node::Traveling { row, col, time } => (
                matches!(
                    blizzards[*row]
                        [(col + (-(*time as isize)).rem_euclid(COLS as isize) as usize) % COLS],
                    Blizzard::Right,
                ),
                matches!(blizzards[*row][(col + time) % COLS], Blizzard::Left),
                matches!(
                    blizzards
                        [(row + (-(*time as isize)).rem_euclid(ROWS as isize) as usize) % ROWS]
                        [*col],
                    Blizzard::Down
                ),
                matches!(blizzards[(row + time) % ROWS][*col], Blizzard::Up),
            ),
        }
    }
}
#[derive(Debug)]
struct QueueNode<const ROWS: usize, const COLS: usize> {
    node: Node<ROWS, COLS>,
    start_col: usize,
    end_col: usize,
}
impl<const ROWS: usize, const COLS: usize> Eq for QueueNode<ROWS, COLS> {}
impl<const ROWS: usize, const COLS: usize> PartialEq for QueueNode<ROWS, COLS> {
    fn eq(&self, other: &Self) -> bool {
        self.node.cost(self.start_col, self.end_col)
            == other.node.cost(other.start_col, other.end_col)
    }
}
impl<const ROWS: usize, const COLS: usize> Ord for QueueNode<ROWS, COLS> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //cmp in reverse, we need a min heap
        other
            .node
            .cost(other.start_col, other.end_col)
            .cmp(&self.node.cost(self.start_col, self.end_col))
    }
}

impl<const ROWS: usize, const COLS: usize> PartialOrd for QueueNode<ROWS, COLS> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// #[derive(PartialEq, Eq, Hash)]
// struct Blizzard {
//     row: usize,
//     col: usize,
//     dir: Direction,
// }

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
    None,
}
impl From<char> for Blizzard {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            '.' => Self::None,
            c => panic!("{c} is not a valid blizzard type"),
        }
    }
}

const TEST: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
