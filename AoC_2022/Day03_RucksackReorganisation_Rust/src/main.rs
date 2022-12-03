use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

struct Rucksack {
    content: Vec<char>,
}

impl<'a> From<&'a Rucksack> for HashSet<&'a char> {
    fn from(sack: &'a Rucksack) -> Self {
        sack.content.iter().collect()
    }
}

impl Rucksack {
    fn split_sack(&self) -> [Self; 2] {
        let middle = self.content.len() / 2;
        let (first, second) = self.content.split_at(middle);
        [
            Self {
                content: first.iter().cloned().collect(),
            },
            Self {
                content: second.iter().cloned().collect(),
            },
        ]
    }
    fn intersect_sacks(sacks: &[Self]) -> HashSet<&char> {
        let mut sacks_iter = sacks.into_iter();
        let mut accu: HashSet<&char> = if let Some(rucksack) = sacks_iter.next() {
            HashSet::from(rucksack)
        } else {
            return HashSet::new();
        };
        for sack in sacks_iter {
            accu = accu.intersection(&HashSet::from(sack)).cloned().collect();
        }
        accu
    }
    fn to_priority(c: char) -> i128 {
        let offset = if c.is_ascii_lowercase() {
            'a' as u8
        } else {
            'A' as u8 - 26
        };
        (c as u8 - offset + 1) as i128
    }
    fn priority_of_sacks(sacks: &[Self]) -> i128 {
        Self::intersect_sacks(sacks)
            .into_iter()
            .map(|&c| Self::to_priority(c))
            .sum::<i128>()
    }
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
    let priority_sum = rucksacks
        .into_iter()
        .map(|r| Rucksack::priority_of_sacks(&r.split_sack()))
        .sum();

    Ok(priority_sum)
}

fn part_two(input: &str) -> Result<i128> {
    let TaskData { rucksacks } = parse_input(input)?;
    let priority_sum = rucksacks
        .chunks(3)
        .map(|c| Rucksack::priority_of_sacks(c))
        .sum();

    Ok(priority_sum)
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
        assert_eq!(answer, 70);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 2708);
        Ok(())
    }
}
