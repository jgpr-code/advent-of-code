use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::io::{self, Read};

static TOTAL_TIME: i128 = 30;

#[derive(Debug)]
struct State {
    node: String,
    remaining_time: i128,
    opened: BTreeSet<String>,
}

#[derive(Debug)]
struct ElephantState {
    elephant_node: String,
    node: String,
    remaining_time: i128,
    opened: BTreeSet<String>,
}

#[derive(Debug)]
struct TaskData {
    nodes: HashMap<String, i128>,
    adjacency: HashMap<String, Vec<String>>,
    best_for_state: HashMap<State, i128>,
    best_for_elephant_state: HashMap<ElephantState, i128>,
    best_release: HashMap<(String, i128, BTreeSet<String>), i128>, // (node, time, opened) -> released
}

impl TaskData {
    fn open(flow: i128, remaining: i128) -> i128 {
        // open starts at_minute, takes one minute then releases until end
        let released = flow * (remaining - 1);
        released
    }
    fn next_states(&self, state: State) -> Vec<State> {
        todo!()
    }
    fn next_elephant_states(&self, state: ElephantState) -> Vec<ElephantState> {
        todo!()
    }
    fn release_alone(&mut self, start: String) -> i128 {
        todo!()
    }
    fn release_together(&mut self, start: String) -> i128 {
        todo!()
    }
    fn can_be_pruned(&self, state: State) {
        todo!()
    }
    fn release_the_most(&mut self, start: String) -> i128 {
        self.bfs_with_prune(start);
        self.best_release
            .iter()
            .map(|(_, released)| *released)
            .max()
            .unwrap()
    }
    fn do_prune(
        &self,
        node: String,
        remaining_time: i128,
        opened: BTreeSet<String>,
        released: i128,
    ) -> bool {
        let key = (node, remaining_time, opened);
        if self.best_release.contains_key(&key) {
            self.best_release[&key] >= released
        } else {
            false
        }
        //false
    }
    fn update_best(
        &mut self,
        node: String,
        remaining: i128,
        opened: BTreeSet<String>,
        released: i128,
    ) {
        let key = (node, remaining, opened);
        if self.best_release.contains_key(&key) {
            if self.best_release[&key] < released {
                self.best_release.insert(key, released);
            }
        } else {
            self.best_release.insert(key, released);
        }
    }
    fn bfs_with_prune(&mut self, start: String) {
        let mut queue: VecDeque<QueueElem> = VecDeque::new(); // at_min, released, remaining
        queue.push_back(QueueElem {
            node: start,
            remaining: 30,
            released: 0,
            opened: BTreeSet::new(),
        }); // node, remaining, released, opened, opened_str
        while let Some(elem) = queue.pop_front() {
            if elem.remaining <= 0 {
                continue;
            } // no time left => nothing to do
              // try open
            let flow = self.nodes[&elem.node];
            if !elem.opened.contains(&elem.node) && flow > 0 {
                let new_remaining = elem.remaining - 1; // open costs 1
                let new_released = elem.released + open(flow, elem.remaining);
                let mut new_opened = elem.opened.clone();
                new_opened.insert(elem.node.clone());
                if !self.do_prune(
                    elem.node.clone(),
                    new_remaining,
                    new_opened.clone(),
                    new_released,
                ) {
                    self.update_best(
                        elem.node.clone(),
                        new_remaining,
                        new_opened.clone(),
                        new_released,
                    );
                    queue.push_back(QueueElem {
                        node: elem.node.clone(),
                        remaining: new_remaining,
                        released: new_released,
                        opened: new_opened,
                    });
                }
            }
            // insert neighbors
            let neighs: Vec<String> = self.adjacency[&elem.node].clone();
            for neigh in neighs {
                let new_remaining = elem.remaining - 1; // moving costs 1
                let new_released = elem.released;
                let new_opened = elem.opened.clone();

                if !self.do_prune(
                    neigh.clone(),
                    new_remaining,
                    new_opened.clone(),
                    new_released,
                ) {
                    self.update_best(
                        neigh.clone(),
                        new_remaining,
                        new_opened.clone(),
                        new_released,
                    );
                    queue.push_back(QueueElem {
                        node: neigh.clone(),
                        remaining: new_remaining,
                        released: new_released,
                        opened: new_opened.clone(),
                    });
                }
            }
        }
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
    let mut best_release = HashMap::new();
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
        best_release.insert((name.clone(), 30, BTreeSet::new()), 0);
    }
    Ok(TaskData {
        nodes,
        adjacency,
        best_for_state,
        best_for_elephant_state,
        best_release,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    //println!("{:?}", data);
    let answer = data.release_the_most(String::from("AA"));
    //println!("");
    //println!("{:?}", data);
    Ok(answer)
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    let answer = data.realse_the_most_together("AA");
    Ok(-1)
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
