use anyhow::Result;
use std::cmp;
use std::collections::VecDeque;
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

struct TaskData {
    blocks: Vec<(i128, i128, i128)>,
    covered_map: HashSet<(i128, i128, i128)>,
    visited: HashSet<(i128, i128, i128)>,
}

impl TaskData {
    fn count_surface_stupid(&self) -> i128 {
        let dx = vec![-1, 1, 0, 0, 0, 0];
        let dy = vec![0, 0, -1, 1, 0, 0];
        let dz = vec![0, 0, 0, 0, -1, 1];
        let mut surface = 0;
        for block in self.blocks.iter() {
            for i in 0..6 {
                let nx = block.0 + dx[i];
                let ny = block.1 + dy[i];
                let nz = block.2 + dz[i];
                if !self.covered_map.contains(&(nx, ny, nz)) {
                    surface += 1;
                }
            }
        }
        surface
    }

    fn get_ranges(&self) -> ((i128, i128), (i128, i128), (i128, i128)) {
        let mut min_x = i128::MAX;
        let mut max_x = i128::MIN;
        let mut min_y = i128::MAX;
        let mut max_y = i128::MIN;
        let mut min_z = i128::MAX;
        let mut max_z = i128::MIN;
        for block in self.blocks.iter() {
            min_x = cmp::min(min_x, block.0);
            max_x = cmp::max(max_x, block.0);
            min_y = cmp::min(min_y, block.1);
            max_y = cmp::max(max_y, block.1);
            min_z = cmp::min(min_z, block.2);
            max_z = cmp::max(max_z, block.2);
        }
        (
            (min_x - 1, max_x + 1),
            (min_y - 1, max_y + 1),
            (min_z - 1, max_z + 1),
        )
    }
    fn count_surface_without_interior(&mut self) -> i128 {
        let dx = vec![-1, 1, 0, 0, 0, 0];
        let dy = vec![0, 0, -1, 1, 0, 0];
        let dz = vec![0, 0, 0, 0, -1, 1];
        let mut surface = 0;
        self.bfs();
        for block in self.blocks.iter() {
            for i in 0..6 {
                let nx = block.0 + dx[i];
                let ny = block.1 + dy[i];
                let nz = block.2 + dz[i];
                let npos = (nx, ny, nz);
                if self.covered_map.contains(&npos) {
                    continue;
                }
                if !self.visited.contains(&npos) {
                    println!("check {:?} from block {:?} failed", npos, block);
                    continue;
                }
                surface += 1;
            }
        }
        surface
    }
    fn bfs(&mut self) {
        let dx = vec![-1, 1, 0, 0, 0, 0];
        let dy = vec![0, 0, -1, 1, 0, 0];
        let dz = vec![0, 0, 0, 0, -1, 1];
        let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = self.get_ranges();
        println!("x: {} - {}", min_x, max_x);
        println!("y: {} - {}", min_y, max_y);
        println!("z: {} - {}", min_z, max_z);
        let mut queue: VecDeque<(i128, i128, i128)> = VecDeque::new();
        // start from all eight corners
        // 0 0 0
        let start = (min_x, min_y, min_z);
        self.visited.insert(start);
        queue.push_back(start);
        // 0 0 1
        let start = (min_x, min_y, max_z);
        self.visited.insert(start);
        queue.push_back(start);
        // 0 1 0
        let start = (min_x, max_y, min_z);
        self.visited.insert(start);
        queue.push_back(start);
        // 0 1 1
        let start = (min_x, max_y, max_z);
        self.visited.insert(start);
        queue.push_back(start);
        // 1 0 0
        let start = (max_x, min_y, min_z);
        self.visited.insert(start);
        queue.push_back(start);
        // 1 0 1
        let start = (max_x, min_y, max_z);
        self.visited.insert(start);
        queue.push_back(start);
        // 1 1 0
        let start = (max_x, max_y, min_z);
        self.visited.insert(start);
        queue.push_back(start);
        // 1 1 1
        let start = (max_x, max_y, max_z);
        self.visited.insert(start);
        queue.push_back(start);

        while let Some(block) = queue.pop_front() {
            for i in 0..6 {
                let nx = block.0 + dx[i];
                let ny = block.1 + dy[i];
                let nz = block.2 + dz[i];
                let npos = (nx, ny, nz);
                if nx < min_x || nx > max_x || ny < min_y || ny > max_y || nz < min_z || nz > max_z
                {
                    continue;
                }
                if self.visited.contains(&npos) || self.covered_map.contains(&npos) {
                    continue;
                }
                queue.push_back(npos);
                self.visited.insert(npos);
            }
        }
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let mut blocks = Vec::new();
    let mut covered_map = HashSet::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(",").collect();
        let pos = (
            i128::from_str_radix(split[0], 10).unwrap(),
            i128::from_str_radix(split[1], 10).unwrap(),
            i128::from_str_radix(split[2], 10).unwrap(),
        );
        blocks.push(pos);
        covered_map.insert(pos);
    }
    Ok(TaskData {
        blocks,
        covered_map,
        visited: HashSet::new(),
    })
}

fn part_one(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    Ok(data.count_surface_stupid())
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    Ok(data.count_surface_without_interior())
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
