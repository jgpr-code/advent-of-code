use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    AddX(i128),
    Noop,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref ADDX: Regex = Regex::new(r"addx (-?\d+)").unwrap();
            static ref NOOP: Regex = Regex::new(r"noop").unwrap();
        }
        if NOOP.is_match(line) {
            Self::Noop
        } else {
            let caps = ADDX.captures(line).unwrap();
            let amount = i128::from_str_radix(&caps[1], 10).unwrap();
            Self::AddX(amount)
        }
    }
}

impl Instruction {
    fn modify(&self, x: i128) -> i128 {
        match self {
            Self::AddX(amount) => x + amount,
            Self::Noop => x,
        }
    }
}

struct TaskData {
    completed_cycles: i128,
    current_x: i128,
    instructions: Vec<Instruction>,
    target_x: Vec<i128>, // 20 60 100 140 180 220 cycles
}

impl TaskData {
    fn cycles(instruction: &Instruction) -> i128 {
        match instruction {
            &Instruction::AddX(_) => 2,
            _ => 1,
        }
    }
    fn sum_targets(&self) -> i128 {
        self.target_x.iter().sum()
    }
    fn is_target_cycle(cycle_before: i128, cycle_after: i128) -> (bool, i128) {
        let targets: Vec<i128> = vec![20, 60, 100, 140, 180, 220];
        for target in targets {
            if cycle_before == target || cycle_before < target && cycle_after > target {
                return (true, target);
            }
        }
        (false, 0)
    }
    fn execute_instruction(&mut self, instruction: &Instruction) {
        // println!("-----------");
        // println!("{:?}", instruction);
        // println!("before cycles: {}", self.completed_cycles);
        // println!("before x: {}", self.current_x);
        let new_completed_cycles = self.completed_cycles + Self::cycles(instruction);
        let (cond, i) = Self::is_target_cycle(self.completed_cycles, new_completed_cycles);
        if cond {
            self.target_x.push(self.current_x * i);
        }
        self.current_x = instruction.modify(self.current_x);
        self.completed_cycles = new_completed_cycles;
        // println!("after cycles: {}", self.completed_cycles);
        // println!("after x: {}", self.current_x);
    }
    fn execute(&mut self) {
        let instructions = self.instructions.clone();
        for instruction in instructions.iter() {
            self.execute_instruction(instruction);
        }
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let instructions: Vec<_> = input.lines().map(|l| Instruction::from(l)).collect();
    Ok(TaskData {
        completed_cycles: 1,
        current_x: 1,
        instructions,
        target_x: Vec::new(),
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.execute();
    println!("{:?}", data.target_x);
    Ok(data.sum_targets())
}

fn part_two(input: &str) -> Result<i128> {
    let _ = parse_input(input)?;
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
