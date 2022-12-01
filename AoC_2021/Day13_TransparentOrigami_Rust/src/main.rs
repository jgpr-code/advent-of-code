use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

struct Origami {
    dots: HashSet<Dot>,
    instructions: VecDeque<FoldingInstruction>,
}

impl Origami {
    fn count_dots(&self) -> i128 {
        self.dots.len() as i128
    }
    fn execute_instruction(&mut self) -> bool {
        let mut new_dots = HashSet::new();
        if let Some(instruction) = self.instructions.pop_front() {
            for dot in &self.dots {
                new_dots.insert(instruction.fold_dot(dot));
            }
            self.dots = new_dots;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dot {
    x: i128,
    y: i128,
}

impl From<&str> for Dot {
    fn from(dot_str: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
        }
        let captures = RE.captures(dot_str).unwrap();
        Dot {
            x: i128::from_str_radix(&captures[1], 10).unwrap(),
            y: i128::from_str_radix(&captures[2], 10).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
enum FoldingInstruction {
    X(i128),
    Y(i128),
}

impl From<&str> for FoldingInstruction {
    fn from(instruction_str: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"fold along ([xy])=(\d+)").unwrap();
        }
        let captures = RE.captures(instruction_str).unwrap();
        match &captures[1] {
            "x" => FoldingInstruction::X(i128::from_str_radix(&captures[2], 10).unwrap()),
            "y" => FoldingInstruction::Y(i128::from_str_radix(&captures[2], 10).unwrap()),
            _ => panic!(),
        }
    }
}

impl FoldingInstruction {
    // fold on Y = 7
    // only dots Y > 7 are affected
    // dot (x, 10) for example
    // 10 - (7+1) = 2
    // (7-1) - (10 - (7+1)) = 4
    //
    // new coordinate becomes (x, 4)
    //
    // general formula:
    // fa - 1 - (a - (fa + 1))
    // fa - 1 - a + fa + 1
    // 2fa - a
    fn fold_dot(&self, dot: &Dot) -> Dot {
        match (self, dot) {
            (&FoldingInstruction::X(f), &Dot { x, y }) if x > f => Dot { x: 2 * f - x, y },
            (&FoldingInstruction::Y(f), &Dot { x, y }) if y > f => Dot { x, y: 2 * f - y },
            (_, &Dot { x, y }) => Dot { x, y },
        }
    }
}

fn parse_input(input: &str) -> Result<Origami> {
    let mut dots = HashSet::new();
    let mut instructions = VecDeque::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        dots.insert(Dot::from(line));
    }
    while let Some(line) = lines.next() {
        instructions.push_back(FoldingInstruction::from(line));
    }
    Ok(Origami { dots, instructions })
}

fn part_one(input: &str) -> Result<i128> {
    let mut origami = parse_input(input)?;
    origami.execute_instruction();

    Ok(origami.count_dots())
}

fn part_two(_input: &str) -> Result<i128> {
    Ok(0)
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("Part one: {}", part_one(&buffer)?);
    println!("Part two: {}", part_two(&buffer)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(answer, 17);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 751);
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
