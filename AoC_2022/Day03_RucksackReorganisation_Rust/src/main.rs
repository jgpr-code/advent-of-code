use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

struct Rucksack {
    content: Vec<char>,
}

impl Rucksack {
    fn get_compartments(&self) -> (Vec<char>, Vec<char>) {
        let middle = self.content.len() / 2;
        let (first, second) = self.content.split_at(middle);
        (
            first.into_iter().cloned().collect::<Vec<_>>(),
            second.into_iter().cloned().collect::<Vec<_>>(),
        )
    }
    fn in_both_compartments(&self) -> char {
        let (first, second) = self.get_compartments();
        let first_set = first.iter().cloned().collect::<HashSet<_>>();
        let second_set = second.iter().cloned().collect::<HashSet<_>>();
        let intersection: Vec<_> = first_set.intersection(&second_set).collect();
        *intersection[0]
    }
}

fn to_priority(c: char) -> i128 {
    let offset = if c.is_ascii_lowercase() {
        'a' as u8
    } else {
        'A' as u8 - 26
    };
    (c as u8 - offset + 1) as i128
}

struct TaskData {
    rucksacks: Vec<Rucksack>,
}

fn parse_input(buffer: &str) -> Result<TaskData> {
    let rucksacks = buffer
        .lines()
        .map(|l| Rucksack {
            content: l.chars().collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();
    Ok(TaskData { rucksacks })
}

fn part_one(input: &str) -> Result<i128> {
    let TaskData { rucksacks } = parse_input(input)?;
    let mut priority_sum = 0;
    for rucksack in rucksacks.iter() {
        priority_sum += to_priority(rucksack.in_both_compartments());
    }
    Ok(priority_sum)
}

fn part_two(input: &str) -> Result<i128> {
    let _ = parse_input(input)?;
    Ok(-1)
}

fn main() -> Result<()> {
    println!("{}", to_priority('c'));
    println!("{}", to_priority('D'));
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("Part one: {}", part_one(&buffer)?);
    println!("Part two: {}", part_two(&buffer)?);
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
        assert_eq!(answer, 157);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 8298);
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
