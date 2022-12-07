use std::collections::BinaryHeap;
use std::rc::Rc;

const INPUT: &str = include_str!("../inputs/day05.txt");
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
pub fn run() -> String {
    todo!()
}

#[derive(Clone)]
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
    fn queue_children(
        &self,
        target: &CrateStacks9001<N>,
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
                    priority_queue.push(Self {
                        distance,
                        heuristic: child_stack.heuristic(&target),
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
    fn first_node(stacks: CrateStacks9001<N>, target: &CrateStacks9001<N>) -> Self {
        Self {
            heuristic: stacks.heuristic(target),
            stacks,
            distance: 0,
            parent: None,
        }
    }
}

#[derive(Clone)]
struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}
#[derive(Clone)]
struct Parent {
    instruction: Instruction,
    parent: Option<Rc<Parent>>,
}
