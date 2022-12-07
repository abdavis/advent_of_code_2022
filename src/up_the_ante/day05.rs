use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

const INPUT: &str = include_str!("../inputs/day05.txt");

pub fn run() -> String {
    let (stacks_str, instruc_str) = INPUT.split_once("\n\n").unwrap();
    let (stacks, instructions) = (
        CrateStacks9001::<9>::from(stacks_str),
        InstructionList::from(instruc_str),
    );
    let target = stacks.process_instructions(&instructions);
    let node = SearchNode::path_find(&stacks, &target);
    assert_eq!(target, node.stacks);
    format!("{}", node.parent.unwrap())
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct CrateStacks9001<const N: usize>([Vec<u8>; N]);

impl<const N: usize> From<&str> for CrateStacks9001<N> {
    fn from(input: &str) -> Self {
        let mut out = std::array::from_fn(|_| Vec::new());
        for line in input.lines().rev().skip(1) {
            for (n, c) in line.as_bytes().iter().skip(1).step_by(4).enumerate() {
                if (b'A'..=b'Z').contains(c) {
                    out[n].push(*c);
                }
            }
        }
        Self(out)
    }
}

impl<const N: usize> CrateStacks9001<N> {
    fn process_instructions(&self, list: &InstructionList) -> Self {
        let mut out = self.clone();
        for ins in &list.0 {
            out.make_move(ins);
        }
        out
    }
    fn make_move(&mut self, ins: &Instruction) {
        let mut moving = self.0[ins.from as usize]
            .split_off(self.0[ins.from as usize].len() - ins.amount as usize);
        self.0[ins.to as usize].append(&mut moving);
        self.0[ins.from as usize].shrink_to_fit();
        self.0[ins.to as usize].shrink_to_fit();
    }
    fn heuristic(&self, target: &Self) -> u32 {
        (self.0.iter().map(|v| v.len()).sum::<usize>()
            - self
                .0
                .iter()
                .zip(target.0.iter())
                .map(|(stack, other)| {
                    stack
                        .iter()
                        .zip(other.iter())
                        .take_while(|(a, b)| a == b)
                        .count()
                })
                .sum::<usize>()) as u32
    }
    fn top_crates(&self) -> String {
        self.0
            .iter()
            .map(|v| char::from(*v.last().unwrap()))
            .collect()
    }
}

struct InstructionList(Vec<Instruction>);
impl From<&str> for InstructionList {
    fn from(input: &str) -> Self {
        let mut out = Vec::new();
        let mut nums = input
            .split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .map(|n| n.parse().unwrap());
        while let (Some(amount), Some(from), Some(to)) = (nums.next(), nums.next(), nums.next()) {
            out.push(Instruction {
                amount,
                from: from - 1,
                to: to - 1,
            });
        }
        Self(out)
    }
}

struct SearchNode<const N: usize> {
    stacks: CrateStacks9001<N>,
    distance: u32,
    heuristic: u32,
    parent: Option<Parent>,
}

impl<const N: usize> SearchNode<N> {
    fn path_find(start: &CrateStacks9001<N>, finish: &CrateStacks9001<N>) -> SearchNode<N> {
        const OVERHEAD: u32 = 3;
        let mut heap = BinaryHeap::new();
        let mut set: HashSet<CrateStacks9001<N>> = HashSet::new();
        let mut max_cost = start.heuristic(finish) + OVERHEAD;
        heap.push(Self::first_node(start.clone(), finish));
        while let Some(node) = heap.pop() {
            if node.stacks == *finish {
                return node;
            }
            if !set.contains(&node.stacks) {
                max_cost = std::cmp::min(max_cost, node.distance + node.heuristic + OVERHEAD);
                Self::queue_children(&node, finish, max_cost, &mut heap);
                set.insert(node.stacks);
            }
        }
        panic!("no path found")
    }
    fn queue_children(
        &self,
        target: &CrateStacks9001<N>,
        max_cost: u32,
        priority_queue: &mut BinaryHeap<SearchNode<N>>,
    ) {
        let distance = self.distance + 1;
        let parent_pointer = match &self.parent {
            None => None,
            Some(val) => Some(Rc::new(val.clone())),
        };
        for from in 0..self.stacks.0.len() {
            for to in (0..from).chain(from + 1..self.stacks.0.len()) {
                for amount in (1..=self.stacks.0[from].len()) {
                    let mut child_stack = self.stacks.clone();
                    let instruction = Instruction {
                        from: from as u8,
                        to: to as u8,
                        amount: amount as u8,
                    };
                    child_stack.make_move(&instruction);
                    let heuristic = child_stack.heuristic(target);
                    if distance + heuristic < max_cost {
                        priority_queue.push(Self {
                            distance,
                            heuristic,
                            stacks: child_stack,
                            parent: Some(Parent {
                                instruction,
                                parent: parent_pointer.clone(),
                            }),
                        })
                    }
                }
            }
        }
    }
    fn first_node(stacks: CrateStacks9001<N>, target: &CrateStacks9001<N>) -> Self {
        Self {
            heuristic: stacks.heuristic(target),
            stacks,
            distance: 0,
            parent: None,
        }
    }
}
impl<const N: usize> PartialEq for SearchNode<N> {
    fn eq(&self, other: &Self) -> bool {
        self.distance + self.heuristic == other.distance + other.heuristic
    }
}
impl<const N: usize> Eq for SearchNode<N> {}
impl<const N: usize> PartialOrd for SearchNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((other.distance + other.heuristic).cmp(&(self.distance + self.heuristic)))
    }
}
impl<const N: usize> Ord for SearchNode<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.distance + other.heuristic).cmp(&(self.distance + self.heuristic))
    }
}
#[derive(Clone)]
struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.amount,
            self.from + 1,
            self.to + 1
        )
    }
}
#[derive(Clone)]
struct Parent {
    instruction: Instruction,
    parent: Option<Rc<Parent>>,
}
impl std::fmt::Display for Parent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(par) = &self.parent {
            write!(f, "{par}\n")?
        }
        write!(f, "{}", self.instruction)
    }
}
#[cfg(test)]
mod tests {
    use crate::up_the_ante::day05::{CrateStacks9001, InstructionList, SearchNode};

    #[test]
    fn verify_test() {
        const TEST: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let (stacks_str, instruc_str) = TEST.split_once("\n\n").unwrap();
        let (stack, instructions) = (
            CrateStacks9001::<3>::from(stacks_str),
            InstructionList::from(instruc_str),
        );
        let target = stack.process_instructions(&instructions);
        let node = SearchNode::path_find(&stack, &target);
        assert_eq!(target, node.stacks);
        println!("{}", node.parent.unwrap());
    }
}
