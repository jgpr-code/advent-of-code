use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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
struct Grid {
    content: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn at_pos(&self, pos: &(i32, i32)) -> i32 {
        self.content[pos.0 as usize][pos.1 as usize]
    }

    fn is_valid_pos(&self, pos: &(i32, i32)) -> bool {
        let row = pos.0;
        let col = pos.1;
        0 <= row && row < self.rows as i32 && 0 <= col && col < self.cols as i32
    }

    fn four_neighbors(&self, pos: &(i32, i32)) -> Vec<(i32, i32)> {
        let delta_row: Vec<i32> = vec![-1, 0, 1, 0];
        let delta_col: Vec<i32> = vec![0, 1, 0, -1];
        self.neighbors_from_deltas(&pos, &delta_row, &delta_col)
    }

    fn eight_neighbors(&self, pos: &(i32, i32)) -> Vec<(i32, i32)> {
        let delta_row: Vec<i32> = vec![-1, -1, 0, 1, 1, 1, 0, -1];
        let delta_col: Vec<i32> = vec![0, 1, 1, 1, 0, -1, -1, -1];
        self.neighbors_from_deltas(&pos, &delta_row, &delta_col)
    }

    fn neighbors_from_deltas(
        &self,
        pos: &(i32, i32),
        delta_row: &Vec<i32>,
        delta_col: &Vec<i32>,
    ) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        for delta in delta_row.iter().zip(delta_col.iter()) {
            let neighbor = (pos.0 + delta.0, pos.1 + delta.1);
            if self.is_valid_pos(&neighbor) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }

    fn incremented_grid(&self, increment: i32) -> Grid {
        let mut content = vec![vec![0; self.cols]; self.rows];
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut incremented = self.content[row][col];
                for inc in 0..increment {
                    incremented += 1;
                    if incremented == 10 {
                        incremented = 1;
                    }
                }
                content[row][col] = incremented;
            }
        }
        Grid {
            content,
            rows: self.rows,
            cols: self.cols,
        }
    }

    // subgrid access would be useful in the future

    fn enlarged_grid(&self, times: usize) -> Grid {
        let mut content = vec![vec![0; times * self.cols]; times * self.rows];
        for tile_row in 0..times {
            for tile_col in 0..times {
                let increment = tile_row + tile_col;
                let incremented_grid = self.incremented_grid(increment as i32);
                let offset_row = tile_row * self.rows;
                let offset_col = tile_col * self.cols;
                for row in 0..self.rows {
                    for col in 0..self.cols {
                        content[offset_row + row][offset_col + col] =
                            incremented_grid.content[row][col];
                    }
                }
            }
        }
        Grid {
            content,
            rows: times * self.rows,
            cols: times * self.cols,
        }
    }
}

fn parse_buffer(buffer: &str) -> Grid {
    let content: Vec<Vec<i32>> = buffer
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();
    let rows = content.len();
    let cols = content[0].len();
    Grid {
        content,
        rows,
        cols,
    }
}

#[derive(Debug, Eq)]
struct Node {
    pos: (i32, i32),
    distance: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            Ordering::Greater
        } else if self.distance > other.distance {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

fn part_one(grid: &Grid) -> i32 {
    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();
    let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();

    dist.insert((0, 0), grid.at_pos(&(0, 0)));

    let target = ((grid.rows - 1) as i32, (grid.cols - 1) as i32);

    priority_queue.push(Node {
        pos: (0, 0),
        distance: 0, // start node doesn't count towards cost
    });

    while let Some(node) = priority_queue.pop() {
        //println!("{:?}", node);
        if node.pos == target {
            return node.distance;
        }
        let old_dist = dist.entry(node.pos).or_insert(i32::MAX);
        if node.distance > *old_dist {
            continue;
        }
        for neighbor_pos in grid.four_neighbors(&node.pos).iter() {
            let next_distance = node.distance + grid.at_pos(neighbor_pos);
            let next_node = Node {
                pos: *neighbor_pos,
                distance: next_distance,
            };
            let next_dist = dist.entry(next_node.pos).or_insert(i32::MAX);
            if next_distance < *next_dist {
                priority_queue.push(next_node);
                *next_dist = next_distance;
            }
        }
    }
    *dist.get(&target).unwrap()
}

fn part_two(grid: &Grid) -> i32 {
    let enlarged_grid = grid.enlarged_grid(5);
    part_one(&enlarged_grid)
}
