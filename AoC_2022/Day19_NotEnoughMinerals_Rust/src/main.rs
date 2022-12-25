use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

#[derive(Debug, Clone)]
struct Blueprint {
    id: i128,
    ore_robot_cost: i128,              // ore
    clay_robot_cost: i128,             // ore
    obsidian_robot_cost: (i128, i128), // (ore, clay)
    geode_robot_cost: (i128, i128),    // (ore, obsidian)
}

impl Blueprint {
    fn max_needed_ore_robots(&self) -> i128 {
        let ore_costs = vec![
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost.0,
            self.geode_robot_cost.0,
        ];
        *ore_costs.iter().max().unwrap()
    }
    fn max_needed_clay_robots(&self) -> i128 {
        self.obsidian_robot_cost.1
    }
    fn max_needed_obsidian_robots(&self) -> i128 {
        self.geode_robot_cost.1
    }
}

struct TaskData {
    blueprints: Vec<Blueprint>,
}

#[derive(Debug)]
struct Simulation {
    blueprint: Blueprint,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    minute: i128,

    ore_robots: i128,
    clay_robots: i128,
    obisidian_robots: i128,
    geode_robots: i128,

    ore: i128,
    clay: i128,
    obsidian: i128,
    geode: i128,
}

impl State {
    fn start() -> Self {
        State {
            minute: 0,
            ore_robots: 1,
            clay_robots: 0,
            obisidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
    fn collect_minerals(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obisidian_robots;
        self.geode += self.geode_robots;
        self.minute += 1;
    }
    fn guaranteed_not_better(&self, other: &State) -> bool {
        let or = self.ore_robots <= other.ore_robots;
        let cr = self.clay_robots <= other.clay_robots;
        let obr = self.obisidian_robots <= other.obisidian_robots;
        let gr = self.geode_robots <= other.geode_robots;
        let o = self.ore <= other.ore;
        let c = self.clay <= other.clay;
        let ob = self.obsidian <= other.obsidian;
        let g = self.geode <= other.geode;
        let t = self.minute >= other.minute;
        or && cr && obr && gr && o && c && ob && g && t
    }
}

impl Simulation {
    fn new(blueprint: Blueprint) -> Self {
        Simulation { blueprint }
    }
    // greedy could be wrong so try brute force
    fn can_build_ore(&self, state: &State) -> bool {
        state.ore >= self.blueprint.ore_robot_cost
    }
    fn build_ore(&self, state: &State) -> State {
        let mut s = state.clone();
        s.ore -= self.blueprint.ore_robot_cost;
        s.ore_robots += 1;
        s
    }
    fn can_build_clay(&self, state: &State) -> bool {
        state.ore >= self.blueprint.clay_robot_cost
    }
    fn build_clay(&self, state: &State) -> State {
        let mut s = state.clone();
        s.ore -= self.blueprint.clay_robot_cost;
        s.clay_robots += 1;
        s
    }
    fn can_build_obsidian(&self, state: &State) -> bool {
        let (ore_cost, clay_cost) = self.blueprint.obsidian_robot_cost;
        state.ore >= ore_cost && state.clay >= clay_cost
    }
    fn build_obsidian(&self, state: &State) -> State {
        let mut s = state.clone();
        let (ore_cost, clay_cost) = self.blueprint.obsidian_robot_cost;
        s.ore -= ore_cost;
        s.clay -= clay_cost;
        s.obisidian_robots += 1;
        s
    }
    fn can_build_geode(&self, state: &State) -> bool {
        let (ore_cost, obsidian_cost) = self.blueprint.geode_robot_cost;
        state.ore >= ore_cost && state.obsidian >= obsidian_cost
    }
    fn build_geode(&self, state: &State) -> State {
        let mut s = state.clone();
        let (ore_cost, obsidian_cost) = self.blueprint.geode_robot_cost;
        s.ore -= ore_cost;
        s.obsidian -= obsidian_cost;
        s.geode_robots += 1;
        s
    }
    fn next_states(&self, state: &State) -> Vec<State> {
        let mut next_states = Vec::new();
        let mut ns = state.clone();
        ns.collect_minerals();
        let ore_saturated = state.ore_robots >= self.blueprint.max_needed_ore_robots();
        let clay_saturated = state.clay_robots >= self.blueprint.max_needed_clay_robots();
        let obsidian_saturated =
            state.obisidian_robots >= self.blueprint.max_needed_obsidian_robots();
        if !ore_saturated && self.can_build_ore(state) {
            next_states.push(self.build_ore(&ns));
        }
        if !clay_saturated && self.can_build_clay(state) {
            next_states.push(self.build_clay(&ns));
        }
        if !obsidian_saturated && self.can_build_obsidian(state) {
            next_states.push(self.build_obsidian(&ns));
        }

        if ore_saturated && clay_saturated && obsidian_saturated && self.can_build_geode(state) {
            next_states.push(self.build_geode(&ns));
        } else {
            if self.can_build_geode(state) {
                next_states.push(self.build_geode(&ns));
            }
            next_states.push(ns);
        }

        next_states
    }
    fn prune_state(&self, states_at_min: &mut HashMap<i128, Vec<State>>, state: &State) -> bool {
        let min = state.minute;
        for m in (0..=min).rev() {
            let check = states_at_min.get(&m).unwrap();
            for s in check {
                if state.guaranteed_not_better(&s) {
                    return true;
                }
            }
        }
        let v = states_at_min.get_mut(&min).unwrap();
        v.push(state.clone());
        false
    }
    fn simulate_all(&self, minutes: i128, start: &State) -> Vec<State> {
        let mut found = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut states_at_min: HashMap<i128, Vec<State>> = HashMap::new();
        for i in 0..=minutes {
            states_at_min.insert(i, Vec::new());
        }
        visited.insert(start.clone());
        queue.push_back(start.clone());
        while let Some(s) = queue.pop_front() {
            // println!("{:?}", s);
            if s.minute == minutes {
                found.push(s);
            } else {
                for ns in self.next_states(&s) {
                    if !visited.contains(&ns) && !self.prune_state(&mut states_at_min, &ns) {
                        visited.insert(ns.clone());
                        queue.push_back(ns);
                    }
                }
            }
        }
        found
    }
    fn max_cracked_geode(&self, start: State, minutes: i128) -> i128 {
        let simulated = self.simulate_all(minutes, &start);
        let max_cracked_geode = simulated.iter().map(|s| s.geode).max().unwrap();
        max_cracked_geode
    }
    fn quality_level(&self, start: State, minutes: i128) -> i128 {
        self.blueprint.id * self.max_cracked_geode(start, minutes)
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    }
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let id = i128::from_str_radix(&caps[1], 10).unwrap();
        let ore = i128::from_str_radix(&caps[2], 10).unwrap();
        let clay = i128::from_str_radix(&caps[3], 10).unwrap();
        let obsidian = (
            i128::from_str_radix(&caps[4], 10).unwrap(),
            i128::from_str_radix(&caps[5], 10).unwrap(),
        );
        let geode = (
            i128::from_str_radix(&caps[6], 10).unwrap(),
            i128::from_str_radix(&caps[7], 10).unwrap(),
        );
        blueprints.push(Blueprint {
            id,
            ore_robot_cost: ore,
            clay_robot_cost: clay,
            obsidian_robot_cost: obsidian,
            geode_robot_cost: geode,
        });
    }
    Ok(TaskData { blueprints })
}

fn part_one(input: &str) -> Result<i128> {
    let TaskData { blueprints } = parse_input(input)?;
    let start = State::start();
    let mut total = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        print!("blueprint {}: ", i + 1);
        let s = Simulation::new(blueprint.clone());
        let t = std::time::Instant::now();
        let q = s.quality_level(start.clone(), 24);
        let elapsed = t.elapsed();
        println!("{} ({}) in {:0.2?}", q, q / (i + 1) as i128, elapsed);
        total += q;
    }
    Ok(total)
}

fn part_two(input: &str) -> Result<i128> {
    let TaskData { blueprints } = parse_input(input)?;
    let start = State::start();
    let mut total = 1;
    for blueprint in blueprints.iter().take(3) {
        print!("blueprint {}: ", blueprint.id);
        let s = Simulation::new(blueprint.clone());
        let t = std::time::Instant::now();
        let cracked_geodes = s.max_cracked_geode(start.clone(), 32);
        let elapsed = t.elapsed();
        println!("cracked geodes {} in {:0.2?}", cracked_geodes, elapsed);
        total *= cracked_geodes;
    }
    Ok(total)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let t = std::time::Instant::now();
    let part_one = part_one(&input)?;
    let elapsed = t.elapsed();
    println!("Part one: {} in {:0.2?}", part_one, elapsed);
    let t = std::time::Instant::now();
    let part_two = part_two(&input)?;
    let elapsed = t.elapsed();
    println!("Part two: {} in {:0.2?}", part_two, elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref TEST: String = read_from_file("test.txt");
        static ref INPUT: String = read_from_file("input.txt");
    }

    fn read_from_file(filename: &str) -> String {
        fs::read_to_string(filename)
            .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg))
    }

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, 33);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 1009);
        Ok(())
    }

    // TODO:
    // Still needs optimization to run this test fast enough
    // #[test]
    // fn test_two() -> Result<()> {
    //     let answer = super::part_two(&TEST)?;
    //     assert_eq!(answer, 56 * 62);
    //     Ok(())
    // }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 18816);
        Ok(())
    }
}
