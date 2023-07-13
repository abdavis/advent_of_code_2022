use std::collections::HashMap;
use std::ops::{Add, SubAssign};
const INPUT: &str = include_str!("inputs/day19.txt");
const EXAMPLE: &str = include_str!("example_inputs/day19.txt");
pub fn run() -> String {
    let blueprints: Vec<Blueprint> = INPUT.lines().map(|l| l.into()).collect();
    format!(
        "Part 1: {}\nPart 2: {}",
        blueprints
            .iter()
            .enumerate()
            .map(|(n, blueprint)| (n + 1) as u16
                * Factory::default().maximize_geodes(blueprint, 24, &mut HashMap::new()))
            .sum::<u16>(),
        blueprints
            .iter()
            .take(3)
            .map(|b| Factory::default().maximize_geodes(b, 32, &mut HashMap::new()))
            .reduce(|acc, n| acc * n)
            .unwrap()
    )
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Factory {
    ore: Resource,
    ore_bots: u16,
    clay: Resource,
    clay_bots: u16,
    obsidian: Resource,
    obsidian_bots: u16,
    geodes: u16,
    geode_bots: u16,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Resource {
    Limited(u16),
    Infinite,
}
impl Add<u16> for Resource {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        match self {
            Self::Infinite => Self::Infinite,
            Self::Limited(n) => Self::Limited(n + rhs),
        }
    }
}
impl SubAssign<u16> for Resource {
    fn sub_assign(&mut self, rhs: u16) {
        match self {
            Resource::Limited(ref mut n) => *n -= rhs,
            Resource::Infinite => (),
        }
    }
}
impl PartialEq<u16> for Resource {
    fn eq(&self, other: &u16) -> bool {
        if let Self::Limited(n) = self {
            n == other
        } else {
            false
        }
    }
}
impl PartialOrd<u16> for Resource {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        match self {
            Self::Limited(n) => Some(n.cmp(other)),
            Self::Infinite => Some(std::cmp::Ordering::Greater),
        }
    }
}
impl Factory {
    fn maximize_geodes(
        &self,
        blueprint: &Blueprint,
        time_left: u16,
        memoizer: &mut HashMap<(u16, Self), u16>,
    ) -> u16 {
        if let Some(val) = memoizer.get(&(time_left, *self)) {
            return *val;
        }
        if time_left == 0 {
            return self.geodes;
        }
        let mut max = 0;
        let stepped = self.mine(blueprint);
        max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer));
        if self.ore >= blueprint.ore && self.ore_bots < blueprint.max_ore_cost() {
            let mut stepped = stepped.clone();
            stepped.ore -= blueprint.ore;
            stepped.ore_bots += 1;
            max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer));
        }
        if self.ore >= blueprint.clay && self.clay_bots < blueprint.clay_cost() {
            let mut stepped = stepped.clone();
            stepped.ore -= blueprint.clay;
            stepped.clay_bots += 1;
            max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer))
        }
        if self.ore >= blueprint.obsidian.0
            && self.clay >= blueprint.obsidian.1
            && self.obsidian_bots < blueprint.obsidian_cost()
        {
            let mut stepped = stepped;
            stepped.ore -= blueprint.obsidian.0;
            stepped.clay -= blueprint.obsidian.1;
            stepped.obsidian_bots += 1;
            max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer));
        }
        if self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1 {
            let mut stepped = stepped;
            stepped.ore -= blueprint.geode.0;
            stepped.obsidian -= blueprint.geode.1;
            stepped.geode_bots += 1;
            max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer));
        }
        memoizer.insert((time_left, *self), max);

        max
    }
    fn mine(&self, blueprint: &Blueprint) -> Self {
        let mut out = Self {
            ore: self.ore + self.ore_bots,
            clay: self.clay + self.clay_bots,
            obsidian: self.obsidian + self.obsidian_bots,
            geodes: self.geodes + self.geode_bots,
            ..*self
        };
        if out.ore >= blueprint.max_ore_cost() && out.ore_bots >= blueprint.max_ore_cost() {
            out.ore = Resource::Infinite;
        }
        if out.clay >= blueprint.clay_cost() && out.clay_bots >= blueprint.clay_cost() {
            out.clay = Resource::Infinite;
        }
        if out.obsidian >= blueprint.obsidian_cost()
            && out.obsidian_bots >= blueprint.obsidian_cost()
        {
            out.obsidian = Resource::Infinite;
        }
        out
    }
}
impl Default for Factory {
    fn default() -> Self {
        Self {
            ore: Resource::Limited(0),
            ore_bots: 1,
            clay: Resource::Limited(0),
            clay_bots: 0,
            obsidian: Resource::Limited(0),
            obsidian_bots: 0,
            geodes: 0,
            geode_bots: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: u16,
    clay: u16,
    obsidian: (u16, u16),
    geode: (u16, u16),
}
impl Blueprint {
    fn max_ore_cost(&self) -> u16 {
        self.ore
            .max(self.clay.max(self.obsidian.0.max(self.geode.0)))
    }
    fn clay_cost(&self) -> u16 {
        self.obsidian.1
    }
    fn obsidian_cost(&self) -> u16 {
        self.geode.1
    }
}
impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let mut nums = value.split_whitespace().filter_map(|w| w.parse().ok());
        Self {
            ore: nums.next().unwrap(),
            clay: nums.next().unwrap(),
            obsidian: (nums.next().unwrap(), nums.next().unwrap()),
            geode: (nums.next().unwrap(), nums.next().unwrap()),
        }
    }
}
