use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("inputs/day12.txt");
pub fn run() -> String {
    let mut input: Topography<41, 70> = INPUT.into();
    format!(
        "{}\n{}",
        input.shortest_path(None),
        input.shortest_possible_path()
    )
}
struct Topography<const ROWS: usize, const COLS: usize> {
    heights: [[u8; COLS]; ROWS],
    start: (usize, usize),
    end: (usize, usize),
}

impl<const ROWS: usize, const COLS: usize> Topography<ROWS, COLS> {
    fn shortest_possible_path(&mut self) -> usize {
        let mut starting_nodes = BinaryHeap::new();

        for row in 0..ROWS {
            for col in 0..COLS {
                if self.heights[row][col] == 0 {
                    starting_nodes.push(SearchNode {
                        pos: (row, col),
                        cost: 0,
                    })
                }
            }
        }
        self.shortest_path(Some(starting_nodes))
    }
    fn shortest_path(&self, starting_nodes: Option<BinaryHeap<SearchNode>>) -> usize {
        let mut completed = HashSet::new();
        let mut queue = match starting_nodes {
            None => {
                let mut heap = BinaryHeap::new();
                heap.push(SearchNode {
                    pos: self.start,
                    cost: 0,
                });
                heap
            }
            Some(heap) => heap,
        };
        while let Some(node) = queue.pop() {
            if node.pos == self.end {
                return node.cost;
            }
            if completed.contains(&node.pos) {
                continue;
            }

            completed.insert(node.pos);
            if node.pos.0 < ROWS - 1
                && !completed.contains(&(node.pos.0 + 1, node.pos.1))
                && self.heights[node.pos.0 + 1][node.pos.1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push(SearchNode {
                    pos: (node.pos.0 + 1, node.pos.1),
                    cost: node.cost + 1,
                })
            }
            if node.pos.0 > 0
                && !completed.contains(&(node.pos.0 - 1, node.pos.1))
                && self.heights[node.pos.0 - 1][node.pos.1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push(SearchNode {
                    pos: (node.pos.0 - 1, node.pos.1),
                    cost: node.cost + 1,
                })
            }
            if node.pos.1 < COLS - 1
                && !completed.contains(&(node.pos.0, node.pos.1 + 1))
                && self.heights[node.pos.0][node.pos.1 + 1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push(SearchNode {
                    pos: (node.pos.0, node.pos.1 + 1),
                    cost: node.cost + 1,
                })
            }
            if node.pos.1 > 0
                && !completed.contains(&(node.pos.0, node.pos.1 - 1))
                && self.heights[node.pos.0][node.pos.1 - 1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push(SearchNode {
                    pos: (node.pos.0, node.pos.1 - 1),
                    cost: node.cost + 1,
                })
            }
        }

        panic!("no path found")
    }
}

struct SearchNode {
    pos: (usize, usize),
    cost: usize,
}
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}
impl Eq for SearchNode {}
impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl<const ROWS: usize, const COLS: usize> From<&str> for Topography<ROWS, COLS> {
    fn from(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut chars = input.chars().filter(|&c| c != '\n');
        let mut heights = [[0; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                match chars.next().unwrap() {
                    'S' => {
                        start = (row, col);
                        heights[row][col] = 0;
                    }
                    'E' => {
                        end = (row, col);
                        heights[row][col] = 25
                    }
                    c @ 'a'..='z' => heights[row][col] = c as u8 - 'a' as u8,
                    _ => panic!("Bad input while parsing"),
                }
            }
        }

        Self {
            start,
            end,
            heights,
        }
    }
}
