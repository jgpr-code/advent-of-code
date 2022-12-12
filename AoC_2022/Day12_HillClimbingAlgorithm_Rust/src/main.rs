use anyhow::Result;
use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

struct TaskData {
    field: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl TaskData {
    fn step_possible_char(from: char, to: char) -> bool {
        let f = from as u8;
        let t = to as u8;
        f >= t || t == f + 1
    }
    fn step_possible(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let mut from_field = self.field[from.0][from.1];
        let mut to_field = self.field[to.0][to.1];
        if from_field == 'S' {
            from_field = 'a';
        }
        if to_field == 'E' {
            to_field = 'z';
        }

        let ans = Self::step_possible_char(from_field, to_field);
        // println!("{} -> {}: {}", from_field, to_field, ans);
        ans
    }
    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let dr = vec![-1, 1, 0, 0];
        let dc = vec![0, 0, -1, 1];
        let mut neighs = Vec::new();
        for i in 0..4 {
            let nr = pos.0 as i32 + dr[i];
            let nc = pos.1 as i32 + dc[i];
            // println!("({:?}, {:?})", nc, nr);
            if nr >= 0 && nc >= 0 && nr < self.rows as i32 && nc < self.cols as i32 {
                let to = (nr as usize, nc as usize);
                if self.step_possible(pos, to) {
                    // println!("{:?} -> {:?}", pos, to);
                    neighs.push(to);
                }
            }
        }
        neighs
    }
    fn min_steps_to_best(&self) -> i128 {
        // (row, col, cost)
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut pos_queue: VecDeque<(usize, usize, i128)> = VecDeque::new();
        let (sr, sc) = self.start;
        pos_queue.push_back((sr, sc, 0));
        visited.insert((sr, sc));
        while let Some((row, col, cost)) = pos_queue.pop_front() {
            let neighbors = self.neighbors((row, col));
            // println!("{:?}", neighbors);
            for neigh in neighbors {
                if visited.contains(&neigh) {
                    continue;
                }
                if neigh == self.end {
                    return cost + 1;
                }
                pos_queue.push_back((neigh.0, neigh.1, cost + 1));
                visited.insert(neigh);
            }
        }
        100000
    }
    fn min_best_from_AorS(&mut self) -> i128 {
        let mut global_min = 10000;
        let (sr, sc) = self.start;
        self.field[sr][sc] = 'a';
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.field[r][c] == 'a' {
                    self.start = (r, c);
                    println!("{:?} {:?}", self.start, global_min);
                    global_min = cmp::min(global_min, self.min_steps_to_best());
                    println!("{:?} {:?}", self.start, global_min);
                }
            }
        }
        global_min
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let field: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let rows = field.len();
    let cols = field[0].len();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for r in 0..rows {
        for c in 0..cols {
            if field[r][c] == 'S' {
                start = (r, c);
            }
            if field[r][c] == 'E' {
                end = (r, c);
            }
        }
    }
    println!("{:?}, {:?}", start, end);
    Ok(TaskData {
        field,
        rows,
        cols,
        start,
        end,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    Ok(data.min_steps_to_best())
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;

    Ok(data.min_best_from_AorS())
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
        assert_eq!(answer, 31);
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
