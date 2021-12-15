use lazy_static::lazy_static;
use multiset::HashMultiSet;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin
        .read_to_string(&mut buffer)
        .expect("failed to read file");
    let mut input_part_one = parse_buffer(&buffer);
    let mut input_part_two = input_part_one.clone();
    println!("{:?}", input_part_one);
    println!("Part 1: {}", part_one(&mut input_part_one));
    println!("Part 2: {}", part_two(&mut input_part_two));
}

#[derive(Debug, Clone)]
struct CaveGraph {
    adjacency: HashMap<String, HashMultiSet<String>>,
    start_label: String,
    end_label: String,
}

impl CaveGraph {
    // this assumes that there are no two big_caves next to each other as this would basically result in an infinite amount of paths
    fn eliminate_big_caves(&mut self, allow_twice: bool) {
        let big_caves: Vec<String> = self
            .adjacency
            .keys()
            .filter(|k| CaveGraph::big_cave(k))
            .map(|c| c.clone()) // don't know if this is needed but just do it for safety for now
            .collect();
        for big_cave in big_caves.iter() {
            let neighbors = self.adjacency[big_cave].clone();
            for (i, neighbor) in neighbors.iter().enumerate() {
                // remove connection to big_cave
                let neighbor_neighbors = self.adjacency.get_mut(neighbor).expect("neighbor");
                neighbor_neighbors.remove(big_cave);
                // add connections to all other caves connected to big_cave
                for (j, other_neighbor) in neighbors.iter().enumerate() {
                    if !allow_twice && i == j {
                        continue;
                    }
                    neighbor_neighbors.insert(other_neighbor.clone());
                }
            }
        }
    }
    fn big_cave(cave: &str) -> bool {
        cave.chars().all(|c| c.is_uppercase())
    }

    fn count_paths(&self, allow_twice: bool) -> i64 {
        let mut path_count = 0;
        let mut visited: HashMap<String, i64> = HashMap::new();
        self.dfs(
            &mut path_count,
            self.start_label.clone(),
            self.end_label.clone(),
            &mut visited,
            allow_twice,
        );
        path_count
    }

    fn dfs(
        &self,
        count: &mut i64,
        from: String,
        to: String,
        visited: &HashMap<String, i64>,
        allow_twice: bool,
    ) {
        if from == to {
            *count += 1;
            return;
        }
        let mut next_visited = visited.clone();
        let elem = next_visited.entry(from.clone()).or_insert(0);
        *elem += 1;
        for neighbor in self.adjacency.get(&from).unwrap().iter() {
            let amount_encountered = next_visited.entry(neighbor.clone()).or_insert(0);
            if *amount_encountered == 0 {
                self.dfs(
                    count,
                    neighbor.clone(),
                    to.clone(),
                    &next_visited,
                    allow_twice,
                );
            } else if *amount_encountered == 1
                && allow_twice
                && *neighbor != self.start_label
                && *neighbor != self.end_label
            {
                self.dfs(count, neighbor.clone(), to.clone(), &next_visited, false);
            }
        }
    }
}

fn parse_buffer(buffer: &str) -> CaveGraph {
    lazy_static! {
        static ref RE: Regex = Regex::new("^(?P<A>.*)-(?P<B>.*)$").unwrap();
    }

    let mut cave_graph = CaveGraph {
        adjacency: HashMap::new(),
        start_label: String::from("start"),
        end_label: String::from("end"),
    };
    for line in buffer.lines() {
        let caps = RE.captures(line).unwrap();
        let node_a = caps.name("A").unwrap().as_str().to_string();
        let node_b = caps.name("B").unwrap().as_str().to_string();

        let adjacent_node_a = cave_graph
            .adjacency
            .entry(node_a.clone())
            .or_insert(HashMultiSet::new());
        adjacent_node_a.insert(node_b.clone());

        let adjacent_node_b = cave_graph
            .adjacency
            .entry(node_b.clone())
            .or_insert(HashMultiSet::new());
        adjacent_node_b.insert(node_a.clone());
    }
    cave_graph
}

fn part_one(cave_graph: &mut CaveGraph) -> i64 {
    let allow_twice = false;
    cave_graph.eliminate_big_caves(allow_twice);
    println!("{:?}", cave_graph);

    cave_graph.count_paths(allow_twice)
}

fn part_two(cave_graph: &mut CaveGraph) -> i64 {
    let allow_twice = true;
    cave_graph.eliminate_big_caves(allow_twice);
    cave_graph.count_paths(allow_twice)
}
// build graph
// replace big Nodes by connecting all its neigbours with each other
// run dfs from start and count finish encounters
