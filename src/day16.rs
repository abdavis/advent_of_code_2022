use std::collections::{HashMap, HashSet, VecDeque};
const INPUT: &str = include_str!("inputs/day16.txt");
const EXAMPLE: &str = include_str!("example_inputs/day16.txt");
pub fn run() -> String {
    let graph: Graph = INPUT.into();
    let part_1 = graph.part_1();
    format!("{part_1}")
}

#[derive(Clone)]
struct Graph<'a>(HashMap<&'a str, Node<'a>>);
impl<'a> From<&'a str> for Graph<'a> {
    fn from(value: &'a str) -> Self {
        let mut out = Self(
            value
                .lines()
                .map(|l| (l.split_whitespace().skip(1).next().unwrap(), l.into()))
                .collect(),
        );
        out.0.shrink_to_fit();
        out
    }
}
impl<'a> Graph<'a> {
    fn part_1(&self) -> usize {
        self.clone()
            .recurse(0, 30, self.get_closed_valves(&"AA"), &mut HashMap::new())
    }
    fn recurse(
        self,
        release_rate: usize,
        time_left: usize,
        closed_valves: Vec<(&'a str, usize)>,
        memoizer: &mut HashMap<(usize, usize, Vec<(&'a str, usize)>), usize>,
    ) -> usize {
        if time_left == 0 {
            return 0;
        }
        if closed_valves.len() == 0 {
            return release_rate * time_left;
        }
        if let Some(val) = memoizer.get(&(release_rate, time_left, closed_valves.clone())) {
            return *val;
        }
        let mut maximum = 0;
        for (key, cost) in &closed_valves {
            let mut new_self = self.clone();
            let new_release_rate = new_self.0[key].flow_rate + release_rate;
            new_self.0.get_mut(key).unwrap().flow_rate = 0;
            let mut pressure_release = release_rate * (cost + 1);
            let mut closed_valves = new_self.get_closed_valves(key);
            closed_valves.shrink_to_fit();
            pressure_release += new_self.recurse(
                new_release_rate,
                time_left - cost - 1,
                closed_valves,
                memoizer,
            );
            if pressure_release > maximum {
                maximum = pressure_release;
            }
        }
        memoizer.insert((release_rate, time_left, closed_valves), maximum);
        maximum
    }
    fn get_closed_valves(&self, start: &'a str) -> Vec<(&'a str, usize)> {
        let mut closed_valves = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some(val) = queue.pop_front() {
            if visited.contains(&val.0) {
                continue;
            }
            visited.insert(val.0);
            if self.0[&val.0].flow_rate > 0 {
                closed_valves.push(val);
            }
            for edge in &self.0[&val.0].edges {
                queue.push_back((edge, val.1 + 1));
            }
        }
        closed_valves
    }
}

#[derive(Clone)]
struct Node<'a> {
    flow_rate: usize,
    edges: Vec<&'a str>,
}
impl<'a> From<&'a str> for Node<'a> {
    fn from(value: &'a str) -> Self {
        let (node_str, edges_str) = value.split_once(';').unwrap();
        let mut node_parts = node_str.split_ascii_whitespace().skip(1).step_by(3);
        let _key = node_parts.next().unwrap();
        let flow_rate = node_parts
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();

        let mut edges: Vec<&str> = edges_str
            .split_ascii_whitespace()
            .skip(4)
            .map(|s| s.trim_end_matches(','))
            .collect();

        edges.shrink_to_fit();

        Self { flow_rate, edges }
    }
}
