use std::io::Write;
use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

const INPUT: &str = include_str!("inputs/day12.txt");
pub fn run() -> String {
    let mut input: Topography<41, 70> = INPUT.into();
    let a = input.shortest_path(None);
    std::thread::sleep(std::time::Duration::from_secs(3));
    let b = input.shortest_possible_path();
    format!("{}\n{}", a, b)
}
struct Topography<const ROWS: usize, const COLS: usize> {
    heights: [[u8; COLS]; ROWS],
    start: (usize, usize),
    end: (usize, usize),
}

impl<const ROWS: usize, const COLS: usize> Topography<ROWS, COLS> {
    fn shortest_possible_path(&mut self) -> usize {
        let mut starting_nodes = VecDeque::new();

        for row in 0..ROWS {
            for col in 0..COLS {
                if self.heights[row][col] == 0 {
                    starting_nodes.push_back(SearchNode {
                        pos: (row, col),
                        cost: 0,
                        parent: None,
                    })
                }
            }
        }
        self.shortest_path(Some(starting_nodes))
    }
    fn shortest_path(&self, starting_nodes: Option<VecDeque<SearchNode>>) -> usize {
        use crossterm::{
            cursor::DisableBlinking,
            terminal::{Clear, ClearType},
            ExecutableCommand,
        };
        use std::io::stdout;
        let mut stdout = stdout();
        stdout
            .execute(DisableBlinking)
            .unwrap()
            .execute(Clear(ClearType::All));
        let mut completed = HashSet::new();
        let mut queue = match starting_nodes {
            None => {
                let mut heap = VecDeque::new();
                heap.push_back(SearchNode {
                    pos: self.start,
                    cost: 0,
                    parent: None,
                });
                heap
            }
            Some(heap) => heap,
        };
        while let Some(node) = queue.pop_front() {
            if completed.contains(&node.pos) {
                continue;
            }
            self.display(&node, &completed, &mut stdout);
            if node.pos == self.end {
                return node.cost;
            }

            completed.insert(node.pos);
            if node.pos.0 < ROWS - 1
                && !completed.contains(&(node.pos.0 + 1, node.pos.1))
                && self.heights[node.pos.0 + 1][node.pos.1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push_back(SearchNode {
                    pos: (node.pos.0 + 1, node.pos.1),
                    cost: node.cost + 1,
                    parent: Some(Rc::new(Parent {
                        pos: node.pos,
                        parent: node.parent.clone(),
                    })),
                })
            }
            if node.pos.0 > 0
                && !completed.contains(&(node.pos.0 - 1, node.pos.1))
                && self.heights[node.pos.0 - 1][node.pos.1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push_back(SearchNode {
                    pos: (node.pos.0 - 1, node.pos.1),
                    cost: node.cost + 1,
                    parent: Some(Rc::new(Parent {
                        pos: node.pos,
                        parent: node.parent.clone(),
                    })),
                })
            }
            if node.pos.1 < COLS - 1
                && !completed.contains(&(node.pos.0, node.pos.1 + 1))
                && self.heights[node.pos.0][node.pos.1 + 1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push_back(SearchNode {
                    pos: (node.pos.0, node.pos.1 + 1),
                    cost: node.cost + 1,
                    parent: Some(Rc::new(Parent {
                        pos: node.pos,
                        parent: node.parent.clone(),
                    })),
                })
            }
            if node.pos.1 > 0
                && !completed.contains(&(node.pos.0, node.pos.1 - 1))
                && self.heights[node.pos.0][node.pos.1 - 1]
                    <= self.heights[node.pos.0][node.pos.1] + 1
            {
                queue.push_back(SearchNode {
                    pos: (node.pos.0, node.pos.1 - 1),
                    cost: node.cost + 1,
                    parent: Some(Rc::new(Parent {
                        pos: node.pos,
                        parent: node.parent.clone(),
                    })),
                })
            }
        }

        panic!("no path found")
    }
    fn display(
        &self,
        node: &SearchNode,
        found: &HashSet<(usize, usize)>,
        stdout: &mut std::io::Stdout,
    ) {
        use crossterm::{
            cursor::MoveTo,
            style::{Color, Print, SetForegroundColor},
            ExecutableCommand, QueueableCommand,
        };
        let mut path_set = HashSet::new();
        path_set.insert(node.pos);
        if let Some(par) = &node.parent {
            par.collect_path(&mut path_set);
        }

        stdout.execute(MoveTo(0, 0)).unwrap();

        for (r, row) in self.heights.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                let num = (val + 2).saturating_mul(10);
                stdout
                    .queue(SetForegroundColor(if path_set.contains(&(r, c)) {
                        Color::Rgb { r: 0, g: num, b: 0 }
                    } else if found.contains(&(r, c)) {
                        Color::Rgb { r: num, g: 0, b: 0 }
                    } else {
                        Color::Rgb {
                            r: num,
                            g: num,
                            b: num,
                        }
                    }))
                    .unwrap();
                stdout.queue(Print("██")).unwrap();
            }
            stdout.queue(Print('\n')).unwrap();
        }
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

struct SearchNode {
    pos: (usize, usize),
    cost: usize,
    parent: Option<Rc<Parent>>,
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
struct Parent {
    pos: (usize, usize),
    parent: Option<Rc<Parent>>,
}
impl Parent {
    fn collect_path(&self, set: &mut HashSet<(usize, usize)>) {
        if let Some(parent) = &self.parent {
            parent.collect_path(set);
        }
        set.insert(self.pos);
    }
}
