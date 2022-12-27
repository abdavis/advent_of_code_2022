use std::fmt::Display;

const INPUT: &str = include_str!("inputs/day16.txt");
const EXAMPLE: &str = include_str!("example_inputs/day16.txt");
pub fn run() -> String {
    //let test_graph: Graph<10> = EXAMPLE.into();
    let graph: Graph<59> = INPUT.into();
    format!("{}\n{}", graph.part_1(), graph.part_2())
}
#[derive(Clone)]
struct Worker {
    pos: usize,
    status: Status,
}
#[derive(Clone, PartialEq)]
enum Status {
    Ready,
    Working(usize),
}
struct Graph<'a, const N: usize> {
    key: GraphKey<'a, N>,
    costs: GraphCosts<N>,
}
impl<const N: usize> Graph<'_, N> {
    fn part_2(&self) -> usize {
        let pos = self.key.names.iter().position(|n| n == &"AA").unwrap();
        let start_state = Worker {
            pos,
            status: Status::Ready,
        };
        self.part_2_recurse(26, self.broken_valves(), start_state.clone(), start_state)
    }
    fn part_2_recurse(
        &self,
        time_left: usize,
        valves_opened: u64,
        me: Worker,
        elephant: Worker,
    ) -> usize {
        if time_left == 0 {
            return 0;
        }
        if valves_opened.trailing_ones() as usize == N
            && Status::Ready == me.status
            && Status::Ready == elephant.status
        {
            return 0;
        }
        match (&me.status, &elephant.status) {
            (Status::Ready, Status::Ready) => {
                let mut max_release = 0;
                let mut me_working = false;
                for m_j in 0..N {
                    if valves_opened & (1 << m_j) != 0 {
                        continue;
                    }
                    let me_spent_time = (self.costs.0[me.pos][m_j]) as usize + 1;
                    if me_spent_time >= time_left {
                        continue;
                    }
                    let mut new_me = me.clone();
                    new_me.pos = m_j;
                    new_me.status = Status::Working(me_spent_time);
                    let mut elephant_working = false;
                    for e_j in (0..m_j).chain(m_j + 1..N) {
                        if valves_opened & (1 << e_j) != 0 {
                            continue;
                        }
                        let eleph_spent_time = (self.costs.0[elephant.pos][e_j]) as usize + 1;
                        if eleph_spent_time >= time_left {
                            continue;
                        }
                        let mut new_elephant = elephant.clone();
                        new_elephant.pos = e_j;
                        new_elephant.status = Status::Working(eleph_spent_time);
                        max_release = max_release.max(self.part_2_recurse(
                            time_left,
                            valves_opened | (1 << m_j) | (1 << e_j),
                            new_me.clone(),
                            new_elephant,
                        ));
                        elephant_working = true;
                        me_working = true;
                    }
                    if !elephant_working {
                        //if the elephant can't do anything, go ahead with just me
                        max_release = max_release.max(self.part_2_recurse(
                            time_left,
                            valves_opened | (1 << m_j),
                            new_me,
                            elephant.clone(),
                        ));
                        me_working = true;
                    }
                }
                if !me_working {
                    //if me didn't do anything, check what elephant can do
                    for e_j in 0..N {
                        if valves_opened & (1 << e_j) != 0 {
                            continue;
                        }
                        let elephant_time_spent = self.costs.0[elephant.pos][e_j] as usize + 1;
                        if elephant_time_spent >= time_left {
                            continue;
                        }
                        let mut new_elephant = elephant.clone();
                        new_elephant.pos = e_j;
                        new_elephant.status = Status::Working(elephant_time_spent);
                        max_release = max_release.max(self.part_2_recurse(
                            time_left,
                            valves_opened | (1 << e_j),
                            me.clone(),
                            new_elephant,
                        ))
                    }
                }
                max_release
            }
            (Status::Ready, Status::Working(_)) => {
                let mut max_release = 0;
                let mut progress_made = false;
                for m_j in 0..N {
                    if valves_opened & (1 << m_j) != 0 {
                        continue;
                    }
                    let me_time_spent = self.costs.0[me.pos][m_j] as usize + 1;
                    if me_time_spent >= time_left {
                        continue;
                    }
                    let mut new_me = me.clone();
                    new_me.pos = m_j;
                    new_me.status = Status::Working(me_time_spent);
                    max_release = max_release.max(self.part_2_recurse(
                        time_left,
                        valves_opened | (1 << m_j),
                        new_me,
                        elephant.clone(),
                    ));
                    progress_made = true;
                }
                if !progress_made {
                    let mut new_elephant = elephant.clone();
                    if let Status::Working(t) = elephant.status {
                        new_elephant.status = Status::Ready;
                        let new_time_left = time_left - t;
                        return self.key.flow_rates[elephant.pos] as usize * new_time_left
                            + self.part_2_recurse(new_time_left, valves_opened, me, new_elephant);
                    }
                }
                max_release
            }
            (Status::Working(_), Status::Ready) => {
                let mut max_release = 0;
                let mut progress_made = false;
                for e_j in 0..N {
                    if valves_opened & (1 << e_j) != 0 {
                        continue;
                    }
                    let elephant_time_spent = self.costs.0[elephant.pos][e_j] as usize + 1;
                    if elephant_time_spent >= time_left {
                        continue;
                    }
                    let mut new_elephant = elephant.clone();
                    new_elephant.pos = e_j;
                    new_elephant.status = Status::Working(elephant_time_spent);
                    max_release = max_release.max(self.part_2_recurse(
                        time_left,
                        valves_opened | (1 << e_j),
                        me.clone(),
                        new_elephant,
                    ));
                    progress_made = true;
                }
                if !progress_made {
                    let mut new_me = me.clone();
                    if let Status::Working(t) = me.status {
                        new_me.status = Status::Ready;
                        let new_time_left = time_left - t;
                        return self.key.flow_rates[me.pos] as usize * new_time_left
                            + self.part_2_recurse(new_time_left, valves_opened, new_me, elephant);
                    }
                }
                max_release
            }
            (Status::Working(me_time), Status::Working(elephant_time)) => {
                let time_passed = me_time.min(elephant_time);
                let me_time_left = me_time - time_passed;
                let eleph_time_left = elephant_time - time_passed;
                let new_time_left = time_left - time_passed;
                let mut pressure_release = 0;
                let new_me = if me_time_left == 0 {
                    pressure_release += self.key.flow_rates[me.pos] as usize * new_time_left;
                    Worker {
                        pos: me.pos,
                        status: Status::Ready,
                    }
                } else {
                    Worker {
                        pos: me.pos,
                        status: Status::Working(me_time_left),
                    }
                };
                let new_eleph = if eleph_time_left == 0 {
                    pressure_release += self.key.flow_rates[elephant.pos] as usize * new_time_left;
                    Worker {
                        pos: elephant.pos,
                        status: Status::Ready,
                    }
                } else {
                    Worker {
                        pos: elephant.pos,
                        status: Status::Working(eleph_time_left),
                    }
                };
                pressure_release
                    + self.part_2_recurse(new_time_left, valves_opened, new_me, new_eleph)
            }
        }
    }
    fn part_1(&self) -> usize {
        let valves_opened = self.broken_valves();
        let node_index = self.key.names.iter().position(|n| *n == "AA").unwrap();
        self.part_1_recurse(30, valves_opened, node_index)
    }
    fn part_1_recurse(&self, time_left: usize, valves_opened: u64, node_index: usize) -> usize {
        //return 0 if all valves are opened
        if valves_opened.trailing_ones() == N as u32 {
            return 0;
        }
        if time_left == 0 {
            return 0;
        }
        let mut max_release = 0;
        for j in 0..N {
            if valves_opened & (1 << j) != 0 {
                continue;
            }
            let spent_time = (self.costs.0[node_index][j] + 1) as usize;
            if spent_time > time_left {
                //skip if valve can't be reached in time
                continue;
            }
            let new_time_left = time_left - spent_time;
            let pressure_release = new_time_left * self.key.flow_rates[j] as usize;
            max_release = max_release.max(
                pressure_release + self.part_1_recurse(new_time_left, valves_opened | (1 << j), j),
            );
        }
        max_release
    }
    fn broken_valves(&self) -> u64 {
        let mut out = 0;
        for (i, flow) in self.key.flow_rates.iter().enumerate() {
            if *flow == 0 {
                out |= 1 << i;
            }
        }
        out
    }
}
impl<'a, const N: usize> From<&'a str> for Graph<'a, N> {
    fn from(value: &'a str) -> Self {
        let key: GraphKey<N> = value.into();
        let costs = (&key).into();
        Self { key, costs }
    }
}
impl<const N: usize> Display for Graph<'_, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //header
        write!(f, "{:>3}", "")?;
        for node in self.key.names {
            write!(f, "{node:>3}")?;
        }

        write!(f, "\n")?;
        for (i, node) in self.key.names.iter().enumerate() {
            write!(f, "{node:>3}")?;
            for j in 0..N {
                write!(f, "{:>3}", self.costs.0[i][j])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}
#[derive(Debug)]
struct GraphCosts<const N: usize>([[u8; N]; N]);
impl<const N: usize> From<&GraphKey<'_, N>> for GraphCosts<N> {
    fn from(value: &GraphKey<N>) -> Self {
        let mut graph = [[255u8; N]; N];
        for (i, edges) in value.edges.iter().enumerate() {
            graph[i][i] = 0;
            for edge in edges {
                let j = value.names.iter().position(|v| v == edge).expect(
                    "couldn't find node {edge}, did you specify the right number of nodes?",
                );
                graph[i][j] = 1;
            }
        }
        for k in 0..N {
            for i in 0..N {
                for j in 0..N {
                    if graph[i][j] > graph[i][k].saturating_add(graph[k][j]) {
                        graph[i][j] = graph[i][k].saturating_add(graph[k][j]);
                    }
                }
            }
        }
        Self(graph)
    }
}

#[derive(Debug)]
struct GraphKey<'a, const N: usize> {
    flow_rates: [u8; N],
    names: [&'a str; N],
    edges: [Vec<&'a str>; N],
}
impl<'a, const N: usize> From<&'a str> for GraphKey<'a, N> {
    fn from(input: &'a str) -> Self {
        let mut graphkey = GraphKey {
            flow_rates: [0; N],
            names: [""; N],
            edges: std::array::from_fn(|_| Vec::new()),
        };
        for (i, line) in input.lines().enumerate() {
            let mut parts = line
                .split([' ', '=', ';', ','])
                .filter(|w| w.len() > 0)
                .skip(1);
            graphkey.names[i] = parts.next().unwrap();
            let mut parts = parts.skip(3);
            graphkey.flow_rates[i] = parts.next().unwrap().parse().unwrap();
            let parts = parts.skip(4);
            graphkey.edges[i] = parts.collect();
        }
        graphkey
    }
}
