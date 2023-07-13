use std::io::stdout;
use std::io::Write;
use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

const INPUT: &str = include_str!("inputs/day12.txt");
pub fn run(graphical_mode: bool) -> String {
    let input: Topography<41, 70> = INPUT.into();
    let (a, b) = input.shortest_path(graphical_mode);
    format!("Part 1: {}\nPart 2: {}", a, b)
}
struct Topography<const ROWS: usize, const COLS: usize> {
    heights: [[u8; COLS]; ROWS],
    start: (usize, usize),
    end: (usize, usize),
}

impl<const ROWS: usize, const COLS: usize> Topography<ROWS, COLS> {
    fn shortest_path(&self, display: bool) -> (usize, usize) {
        let mut completed = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(SearchNode {
            pos: self.end,
            cost: 0,
            parent: None,
        });
        let mut part_a = None;
        let mut part_b = None;
        while let Some(node) = queue.pop_front() {
            if completed.contains(&node.pos) {
                continue;
            }
            if display {
                self.display(&node, &part_a, &part_b, &completed);
            }
            if node.pos == self.start {
                part_a = Some(node.clone());
            }
            if self.heights[node.pos.0][node.pos.1] == 1 && part_b.is_none() {
                part_b = Some(node.clone());
            }
            if let (Some(a), Some(b)) = (&part_a, &part_b) {
                return (a.cost, b.cost);
            }

            completed.insert(node.pos);
            if node.pos.0 < ROWS - 1
                && !completed.contains(&(node.pos.0 + 1, node.pos.1))
                && self.heights[node.pos.0 + 1][node.pos.1]
                    >= self.heights[node.pos.0][node.pos.1] - 1
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
                    >= self.heights[node.pos.0][node.pos.1] - 1
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
                    >= self.heights[node.pos.0][node.pos.1] - 1
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
                    >= self.heights[node.pos.0][node.pos.1] - 1
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
        path_a: &Option<SearchNode>,
        path_b: &Option<SearchNode>,
        found: &HashSet<(usize, usize)>,
    ) {
        use crossterm::{
            cursor::MoveTo,
            style::{Color, Print, SetForegroundColor},
            ExecutableCommand, QueueableCommand,
        };
        let mut path_set = HashSet::new();
        node.collect_path(&mut path_set);
        if let Some(a) = path_a {
            a.collect_path(&mut path_set);
        }
        if let Some(b) = path_b {
            b.collect_path(&mut path_set);
        }
        if let Some(par) = &node.parent {
            par.collect_path(&mut path_set);
        }
        let mut stdout = stdout();
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

#[derive(Clone)]
struct SearchNode {
    pos: (usize, usize),
    cost: usize,
    parent: Option<Rc<Parent>>,
}
impl SearchNode {
    fn collect_path(&self, set: &mut HashSet<(usize, usize)>) {
        set.insert(self.pos);
        if let Some(parent) = self.parent.clone() {
            parent.collect_path(set);
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
                        heights[row][col] = 1;
                    }
                    'E' => {
                        end = (row, col);
                        heights[row][col] = 26
                    }
                    c @ 'a'..='z' => heights[row][col] = c as u8 - 'a' as u8 + 1,
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
