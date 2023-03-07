use std::collections::HashMap;
const INPUT: &str = include_str!("inputs/day19.txt");
const EXAMPLE: &str = include_str!("example_inputs/day19.txt");
pub fn run() -> String {
    let blueprints: Vec<Blueprint> = INPUT.lines().map(|l| l.into()).collect();
    format!(
        "{}\n{}",
        blueprints
            .iter()
            .enumerate()
            .map(|(n, blueprint)| (n + 1)
                * Factory::default().maximize_geodes(blueprint, 24, &mut HashMap::new()))
            .sum::<usize>(),
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
    ore: usize,
    ore_bots: usize,
    clay: usize,
    clay_bots: usize,
    obsidian: usize,
    obsidian_bots: usize,
    geodes: usize,
    geode_bots: usize,
}
impl Factory {
    fn maximize_geodes(
        &self,
        blueprint: &Blueprint,
        time_left: usize,
        memoizer: &mut HashMap<(usize, Self), usize>,
    ) -> usize {
        if let Some(val) = memoizer.get(&(time_left, *self)) {
            return *val;
        }
        if time_left == 0 {
            return self.geodes;
        }
        let mut max = 0;
        let stepped = self.mine();
        max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer));
        if self.ore >= blueprint.ore {
            let mut stepped = stepped.clone();
            stepped.ore -= blueprint.ore;
            stepped.ore_bots += 1;
            max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer));
        }
        if self.ore >= blueprint.clay {
            let mut stepped = stepped.clone();
            stepped.ore -= blueprint.clay;
            stepped.clay_bots += 1;
            max = max.max(stepped.maximize_geodes(blueprint, time_left - 1, memoizer))
        }
        if self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1 {
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
    fn mine(&self) -> Self {
        Self {
            ore: self.ore + self.ore_bots,
            clay: self.clay + self.clay_bots,
            obsidian: self.obsidian + self.obsidian_bots,
            geodes: self.geodes + self.geode_bots,
            ..*self
        }
    }
}
impl Default for Factory {
    fn default() -> Self {
        Self {
            ore: 0,
            ore_bots: 1,
            clay: 0,
            clay_bots: 0,
            obsidian: 0,
            obsidian_bots: 0,
            geodes: 0,
            geode_bots: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
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
