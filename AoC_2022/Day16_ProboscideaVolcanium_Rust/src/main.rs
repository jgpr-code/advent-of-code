use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
// use std::cmp;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    second: bool,
    remaining_time: i128,
    node: String,
    opened: BTreeSet<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Elem {
    released: i128,
    state: State,
}

// impl Ord for Elem {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         self.released.cmp(&other.released)
//     }
// }

// impl PartialOrd for Elem {
//     fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
//         Some(self.cmp(&other))
//     }
// }

#[derive(Debug)]
struct TaskData {
    node_flows: HashMap<String, i128>,
    adjacency: HashMap<String, Vec<String>>,
    best_for_state: HashMap<State, i128>,
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
                second: old_state.second,
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
                second: old_state.second,
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
        let key = &elem.state;
        // also update all nodes with less time and same pos, opened
        if self.best_for_state.contains_key(key) {
            let best = self.best_for_state[key];
            if elem.released > best {
                self.best_for_state.insert(key.clone(), elem.released);
                if elem.state.second {
                    for t in 0..=key.remaining_time {
                        let mut ck = key.clone();
                        ck.remaining_time = t;
                        if !self.best_for_state.contains_key(&ck)
                            || self.best_for_state[&ck] < elem.released
                        {
                            self.best_for_state.insert(ck.clone(), elem.released);
                        }
                    }
                }
                return false;
            } else {
                return true;
            }
        } else {
            let best = elem.released;
            self.best_for_state.insert(key.clone(), best);
            if elem.state.second {
                for t in 0..=key.remaining_time {
                    let mut ck = key.clone();
                    ck.remaining_time = t;
                    if !self.best_for_state.contains_key(&ck) || self.best_for_state[&ck] < best {
                        self.best_for_state.insert(ck.clone(), best);
                    }
                }
            }
            return false;
        };
    }

    fn bfs(&mut self, start_node: String, initial_time: i128, use_second: bool) {
        let mut queue: VecDeque<Elem> = VecDeque::new();
        // let mut prio_queue: BinaryHeap<Elem> = BinaryHeap::new();
        let start = Elem {
            released: 0,
            state: State {
                second: false,
                remaining_time: initial_time,
                node: start_node.clone(),
                opened: BTreeSet::new(),
            },
        };
        //prio_queue.push(start);
        queue.push_back(start);
        //while let Some(elem) = prio_queue.pop() {
        while let Some(elem) = queue.pop_front() {
            if elem.state.remaining_time <= 0 {
                if !use_second || elem.state.second {
                    continue;
                }
                let mut next_elem = elem.clone();
                next_elem.state.second = true;
                next_elem.state.remaining_time = initial_time;
                next_elem.state.node = start_node.clone();
                if !self.prune(&next_elem) {
                    // println!("{:?}", next_elem);
                    queue.push_back(next_elem);
                    // prio_queue.push(next_elem);
                }
            } else {
                for next_elem in self.next_elems(elem).into_iter() {
                    // println!("{:?}", next_elem);
                    if !self.prune(&next_elem) {
                        queue.push_back(next_elem);
                        // prio_queue.push(next_elem);
                    }
                }
            }
        }
    }
    fn release(&mut self, start: String, initial_time: i128, use_second: bool) -> i128 {
        self.bfs(start, initial_time, use_second);
        // println!("{:?}", self.best_for_state);
        self.best_for_state
            .iter()
            .map(|(_, released)| *released)
            .max()
            .unwrap()
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
        best_for_state: HashMap::new(),
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    //println!("{:?}", data);
    let answer = data.release(String::from("AA"), 30, false);
    //println!("");
    //println!("{:?}", data);
    Ok(answer)
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    let answer = data.release(String::from("AA"), 26, true);
    Ok(answer)
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
        assert_eq!(answer, 2723);
        Ok(())
    }
}
