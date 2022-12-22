use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
const INPUT: &str = include_str!("inputs/day16.txt");
pub fn run() -> String {
    todo!();
}

#[derive(Clone)]
struct Graph<'a>(HashMap<&'a str, Node<'a>>);
impl<'a> From<&'a str> for Graph<'a> {
    fn from(value: &'a str) -> Self {
        let mut out = Self(value.lines().map(|l| l.into()).collect());
        out.0.shrink_to_fit();
        out
    }
}
impl<'a> Graph<'a> {
    fn recurse(
        self,
        release_rate: usize,
        time_left: usize,
        closed_valves: HashMap<&'a str, usize>,
    ) -> usize {
        if time_left == 0 {
            return 0;
        }
        todo!()
    }
    fn get_closed_valves(&self, start: &str) -> Vec<(&'a str, usize)> {
        let mut closed_valves = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some(val) = queue.pop_front() {
            if !visited.contains(&val.0) {}
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
        let key = node_parts.next().unwrap();
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

        Self {
            key,
            flow_rate,
            edges,
        }
    }
}
