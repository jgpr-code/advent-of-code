use anyhow::Result;
use std::cmp::Reverse; // to make a min heap push elems wrapped in Reverse
use std::collections::{BinaryHeap, HashMap, HashSet}; // max heap
use std::io::{self, Read};
use std::ops::{Add, Neg, Sub};

// implement simulation + a*

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2d {
    x: i128,
    y: i128,
}
impl Add for Vec2d {
    type Output = Vec2d;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Neg for Vec2d {
    type Output = Vec2d;
    fn neg(self) -> Self::Output {
        Vec2d {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl Sub for Vec2d {
    type Output = Vec2d;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
impl Vec2d {
    fn new(x: i128, y: i128) -> Self {
        Vec2d { x, y }
    }
    fn manhattan(&self, other: &Vec2d) -> i128 {
        let delta = *self - *other;
        delta.x.abs() + delta.y.abs()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct State {
    time: usize,
    manhattan_to_target: i128,
    pos: Vec2d,
}

struct TaskData {
    directions: Vec<Vec2d>, // 0 = >, 1 = v, 2 = <, 3 = ^
    blizzards_modulus: usize,
    blizzards: Vec<HashMap<Vec2d, Vec<usize>>>, // blizzard at, directions_index
    start: Vec2d,
    target: Vec2d,
    rows: i128,
    cols: i128,
}
impl TaskData {
    fn print_time(&self, time: usize) {
        let time = time % self.blizzards_modulus;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = Vec2d::new(row, col);
                if pos == self.start {
                    print!("S");
                } else if pos == self.target {
                    print!("T");
                } else if row == 0 || row == self.rows - 1 || col == 0 || col == self.cols - 1 {
                    print!("#");
                } else {
                    if let Some(blizzards) =
                        self.blizzards[time].get(&Vec2d::new(row as i128, col as i128))
                    {
                        let n = blizzards.len();
                        if n > 1 {
                            print!("{}", blizzards.len())
                        } else if n == 1 {
                            match blizzards[0] {
                                0 => print!(">"),
                                1 => print!("v"),
                                2 => print!("<"),
                                3 => print!("^"),
                                _ => panic!("unknown blizzard"),
                            }
                        }
                    } else {
                        print!(".");
                    }
                }
            }
            println!("");
        }
    }
    fn simulate_blizzard_states(&mut self) {
        loop {
            let current_blizzards = self.blizzards.last().unwrap().clone();
            let mut new_blizzards: HashMap<Vec2d, Vec<usize>> = HashMap::new();
            for (pos, blizzards) in current_blizzards {
                for blizzard in blizzards {
                    let mut new_pos = pos + self.directions[blizzard];
                    self.wrap_pos(&mut new_pos);
                    let vec = new_blizzards.entry(new_pos).or_insert(Vec::new());
                    vec.push(blizzard);
                }
            }
            if new_blizzards == self.blizzards[0] {
                return;
            } else {
                self.blizzards_modulus += 1;
                self.blizzards.push(new_blizzards);
            }
        }
    }
    fn wrap_pos(&self, pos: &mut Vec2d) {
        if pos.x == 0 {
            pos.x = self.rows - 2;
        } else if pos.x == self.rows - 1 {
            pos.x = 1;
        }
        if pos.y == 0 {
            pos.y = self.cols - 2;
        } else if pos.y == self.cols - 1 {
            pos.y = 1;
        }
    }
    fn is_possible(&self, pos: &Vec2d, time: usize) -> bool {
        if pos.x == 0 || pos.x == self.rows - 1 || pos.y == 0 || pos.y == self.cols - 1 {
            return *pos == self.start || *pos == self.target;
        }
        let blizzard_time = time % self.blizzards_modulus;
        if self.blizzards[blizzard_time].contains_key(pos) {
            return false;
        }
        true
    }
    // assumes simulate_blizzard_states was run before!
    fn find_path(&self, initial_time: usize, from: &Vec2d, to: &Vec2d) -> usize {
        let mut prio_queue = BinaryHeap::new();
        let mut visited = HashSet::new();
        visited.insert((initial_time % self.blizzards_modulus, *from));
        prio_queue.push(Reverse(State {
            time: initial_time,
            manhattan_to_target: from.manhattan(&to),
            pos: *from,
        }));
        while let Some(Reverse(state)) = prio_queue.pop() {
            let ntime = state.time + 1;
            //println!("time: {}", ntime);
            for dir in self.directions.iter() {
                let npos = state.pos + *dir;
                if npos == *to {
                    return ntime; // best found
                }
                let check = (ntime % self.blizzards_modulus, npos);
                if !visited.contains(&check) && self.is_possible(&npos, ntime) {
                    prio_queue.push(Reverse(State {
                        time: ntime,
                        manhattan_to_target: npos.manhattan(to),
                        pos: npos,
                    }));
                    visited.insert(check);
                }
            }
            let npos_wait = state.pos;
            let check = (ntime % self.blizzards_modulus, npos_wait);
            if !visited.contains(&check) && self.is_possible(&npos_wait, ntime) {
                prio_queue.push(Reverse(State {
                    time: ntime,
                    manhattan_to_target: npos_wait.manhattan(to),
                    pos: npos_wait,
                }));
                visited.insert(check);
            }
        }
        panic!("no target found");
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let directions = vec![
        Vec2d::new(0, 1),  // right
        Vec2d::new(1, 0),  // down
        Vec2d::new(0, -1), // left
        Vec2d::new(-1, 0), // up
    ];
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut start_col = -1;
    let mut target_col = -1;
    for col in 0..cols {
        let first_row = grid[0][col];
        let last_row = grid[rows - 1][col];
        if first_row == '.' {
            start_col = col as i128;
        }
        if last_row == '.' {
            target_col = col as i128;
        }
    }
    if start_col == -1 || target_col == -1 {
        panic!("invalid input grid, no start or target");
    }
    let start = Vec2d::new(0, start_col);
    let target = Vec2d::new(rows as i128 - 1, target_col);
    let mut initial_blizzards = HashMap::new();
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let pos = Vec2d::new(row as i128, col as i128);
            let c = grid[row][col];
            if c != '.' {
                let vec = initial_blizzards.entry(pos).or_insert(Vec::new());
                vec.push(match c {
                    '>' => 0,
                    'v' => 1,
                    '<' => 2,
                    '^' => 3,
                    _ => panic!("invalid char in input"),
                });
            }
        }
    }
    let mut blizzards = Vec::new();
    blizzards.push(initial_blizzards);

    Ok(TaskData {
        directions,
        blizzards_modulus: 1,
        blizzards,
        start,
        target,
        rows: rows as i128,
        cols: cols as i128,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.simulate_blizzard_states();
    Ok(data.find_path(0, &data.start, &data.target) as i128)
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.simulate_blizzard_states();
    let s_to_t = data.find_path(0, &data.start, &data.target);
    let t_to_s = data.find_path(s_to_t, &data.target, &data.start);
    Ok(data.find_path(t_to_s, &data.start, &data.target) as i128)
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
        assert_eq!(answer, 0);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 0);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 0);
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
