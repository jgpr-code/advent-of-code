use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::io::{self, Read};

static TOTAL_TIME: i128 = 30;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    remaining_time: i128,
    node: String,
    opened: BTreeSet<String>,
}
#[derive(Debug)]
struct Elem {
    released: i128,
    state: State,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ElephantState {
    remaining_time: i128,
    nodes: Vec<String>,
    opened: BTreeSet<String>,
}
#[derive(Debug)]
struct ElephantElem {
    released: i128,
    state: ElephantState,
}

#[derive(Debug)]
struct TaskData {
    node_flows: HashMap<String, i128>,
    adjacency: HashMap<String, Vec<String>>,
    best_for_state: HashMap<State, i128>,
    best_for_elephant_state: HashMap<ElephantState, i128>,
    // best_release: HashMap<(String, i128, BTreeSet<String>), i128>, // (node, time, opened) -> released
}

impl TaskData {
    fn open(flow: i128, remaining: i128) -> i128 {
        // open starts at_minute, takes one minute then releases until end
        let released = flow * (remaining - 1);
        released
    }
    fn next_elems(&self, elem: Elem) -> Vec<Elem> {
        let mut elems = Vec::new();

        let old_released = elem.released;
        let old_state = elem.state;
        let flow = self.node_flows[&old_state.node];
        // try open
        if !old_state.opened.contains(&old_state.node) && flow > 0 {
            let released = old_released + Self::open(flow, old_state.remaining_time);
            let remaining_time = old_state.remaining_time - 1;
            let node = old_state.node.clone();
            let mut opened = old_state.opened.clone();
            opened.insert(old_state.node.clone());
            let next_state = State {
                remaining_time,
                node,
                opened,
            };
            elems.push(Elem {
                released,
                state: next_state,
            });
        }
        // try neighbors
        for neigh in self.adjacency[&old_state.node].iter() {
            let released = old_released;
            let remaining_time = old_state.remaining_time - 1;
            let node = neigh.clone();
            let opened = old_state.opened.clone();
            let next_state = State {
                remaining_time,
                node,
                opened,
            };
            elems.push(Elem {
                released,
                state: next_state,
            });
        }
        elems
    }
    fn prune(&mut self, elem: &Elem) -> bool {
        if !self.best_for_state.contains_key(&elem.state) {
            self.best_for_state.insert(elem.state.clone(), 0);
            return false;
        }
        let best = self.best_for_state[&elem.state];
        best >= elem.released
    }
    fn update_best(&mut self, elem: &Elem) {
        self.best_for_state
            .insert(elem.state.clone(), elem.released);
    }
    fn bfs(&mut self, start: String) {
        let mut queue: VecDeque<Elem> = VecDeque::new();
        let start = Elem {
            released: 0,
            state: State {
                remaining_time: 30,
                node: start,
                opened: BTreeSet::new(),
            },
        };
        queue.push_back(start);
        while let Some(elem) = queue.pop_front() {
            if elem.state.remaining_time <= 0 {
                continue;
            }
            for next_elem in self.next_elems(elem).into_iter() {
                // println!("{:?}", next_elem);
                if !self.prune(&next_elem) {
                    self.update_best(&next_elem);
                    queue.push_back(next_elem);
                }
            }
        }
    }
    fn release(&mut self, start: String) -> i128 {
        self.bfs(start);
        // println!("{:?}", self.best_for_state);
        self.best_for_state
            .iter()
            .map(|(_, released)| *released)
            .max()
            .unwrap()
    }
    fn next_elephant_states(&self, state: ElephantState) -> Vec<ElephantState> {
        todo!()
    }
    fn release_together(&mut self, start: String) -> i128 {
        todo!()
    }
}

//           BB
//           |
//      CC---AA---DD
//           |
//           EE
//
// graph with costs between edges
// each node has open(time) -> released
// somehow avoid repeating states e.g. nodes concat + time
// same node but with less time and less released => prune
// never open valves with flow 0 -> just treat as opened from the start

fn parse_input(input: &str) -> Result<TaskData> {
    lazy_static! {
        static ref FLOW: Regex = Regex::new(r"flow rate=(\d+)").unwrap();
        static ref NODES: Regex = Regex::new(r"([A-Z][A-Z])").unwrap();
    }
    let mut nodes = HashMap::new();
    let mut adjacency = HashMap::new();
    let mut best_for_state = HashMap::new();
    let mut best_for_elephant_state = HashMap::new();
    for line in input.lines() {
        let flow = i128::from_str_radix(&FLOW.captures(line).unwrap()[1], 10).unwrap();
        let nodes_cap: Vec<String> = NODES
            .captures_iter(line)
            .map(|c| String::from(&c[0]))
            .collect();
        let name = nodes_cap[0].clone();
        nodes.insert(name.clone(), flow);
        let neighs: Vec<String> = nodes_cap.iter().skip(1).map(|s| s.clone()).collect();
        adjacency.insert(name.clone(), neighs);
    }
    Ok(TaskData {
        node_flows: nodes,
        adjacency,
        best_for_state,
        best_for_elephant_state,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    //println!("{:?}", data);
    let answer = data.release(String::from("AA"));
    //println!("");
    //println!("{:?}", data);
    Ok(answer)
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    let answer = data.release_together(String::from("AA"));
    Ok(answer)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part one: {}", part_one(&input)?);
    println!("Part two: {}", part_two(&input)?);
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
        assert_eq!(answer, 1651);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 2029);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 1707);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 0);
        Ok(())
    }
}
