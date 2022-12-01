use anyhow::Result;
use std::io::{self, Read};

struct Elves {
    elf_vec: Vec<Elf>,
}

struct Elf {
    calories: Vec<i128>,
}

impl Elf {
    fn sum_calories(&self) -> i128 {
        self.calories.iter().sum()
    }
}

fn parse_input(buffer: &str) -> Result<Elves> {
    let mut elf_vec: Vec<Elf> = Vec::new();
    // TODO: make this robust against \n vs \r\n
    let all_elf_calories = buffer.split("\r\n\r\n").collect::<Vec<_>>();

    for elf_calories in all_elf_calories {
        let calories = elf_calories
            .lines()
            .map(|l| i128::from_str_radix(l, 10))
            .collect::<Result<Vec<i128>, _>>()?;
        elf_vec.push(Elf { calories });
    }

    Ok(Elves { elf_vec })
}

fn part_one(input: &str) -> Result<i128> {
    let elves = parse_input(input)?;
    let total_calories: Vec<i128> = elves.elf_vec.iter().map(|e| e.sum_calories()).collect();
    let maximum = total_calories.iter().max().unwrap();
    Ok(*maximum)
}

fn part_two(input: &str) -> Result<i128> {
    let elves = parse_input(input)?;
    let mut total_calories: Vec<i128> = elves.elf_vec.iter().map(|e| e.sum_calories()).collect();
    total_calories.sort();
    let top3: i128 = total_calories.iter().rev().take(3).sum();
    Ok(top3)
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
        assert_eq!(answer, 24000);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 66306);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 45000);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 195292);
        Ok(())
    }
}
