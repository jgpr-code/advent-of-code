use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Debug)]
struct TaskData {
    stacks: Vec<VecDeque<char>>,
    operations: Vec<Operation>,
}

impl TaskData {
    fn execute_operations(&mut self) {
        for op in self.operations.iter() {
            op.execute(&mut self.stacks);
        }
    }
    fn execute_operations_9001(&mut self) {
        for op in self.operations.iter() {
            op.execute_9001(&mut self.stacks);
        }
    }
    fn stack_top_str(&self) -> String {
        let mut tops = String::new();
        for stack in self.stacks.iter() {
            let &c = stack.front().unwrap_or(&' ');
            tops.push(c);
        }
        tops
    }
}

#[derive(Debug)]
struct Operation {
    amount: usize,
    from: usize,
    to: usize,
}

impl Operation {
    fn execute(&self, stacks: &mut Vec<VecDeque<char>>) {
        //println!("{:?}", stacks);
        let mut amount = self.amount;
        while amount > 0 {
            if let Some(c) = stacks[self.from].pop_front() {
                stacks[self.to].push_front(c);
            }
            amount -= 1;
        }
    }
    fn execute_9001(&self, stacks: &mut Vec<VecDeque<char>>) {
        //println!("{:?}", stacks);
        let to_push: Vec<char> = stacks[self.from].drain(..self.amount).rev().collect();
        for p in to_push {
            stacks[self.to].push_front(p);
        }
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let mut split = input.split("\r\n\r\n");
    let stacks_str = split.next().unwrap();
    let operations_str = split.next().unwrap();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for line in stacks_str.lines() {
        let v: Vec<char> = line.chars().collect();
        // [A] [B] [C]
        //  1   5   9  ... -> index = (i - 1) / 4
        let mut i = 1;
        while let Some(&c) = v.get(i) {
            if c != ' ' {
                let target_stack = (i - 1) / 4;
                while stacks.len() <= target_stack {
                    stacks.push(VecDeque::new());
                }
                stacks[target_stack].push_back(c);
            }
            i += 4;
        }
    }

    let mut operations = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    for line in operations_str.lines() {
        let captures = RE.captures(line).unwrap();
        operations.push(Operation {
            amount: usize::from_str_radix(&captures[1], 10)?,
            from: usize::from_str_radix(&captures[2], 10)? - 1,
            to: usize::from_str_radix(&captures[3], 10)? - 1,
        })
    }

    Ok(TaskData { stacks, operations })
}

fn part_one(input: &str) -> Result<String> {
    let mut data = parse_input(input)?;
    data.execute_operations();
    Ok(data.stack_top_str())
}

fn part_two(input: &str) -> Result<String> {
    let mut data = parse_input(input)?;
    data.execute_operations_9001();
    Ok(data.stack_top_str())
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
        assert_eq!(answer, "CMZ");
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, "QGTHFZBHV");
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, "MCD");
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, "MGDMPSZTM");
        Ok(())
    }
}
