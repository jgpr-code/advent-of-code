use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

struct TaskData {
    signal: Vec<char>,
}

fn parse_input(input: &str) -> Result<TaskData> {
    let signal: Vec<char> = input.lines().next().unwrap().chars().collect();
    Ok(TaskData { signal })
}

fn only_unique(four: &[char]) -> bool {
    four[0] != four[1]
        && four[0] != four[2]
        && four[0] != four[3]
        && four[1] != four[2]
        && four[1] != four[3]
        && four[2] != four[3]
}

fn only_unique2(f: &[char]) -> bool {
    let mut encountered: HashSet<char> = HashSet::new();
    for &c in f {
        if encountered.contains(&c) {
            return false;
        }
        encountered.insert(c);
    }
    true
}

fn part_one(input: &str) -> Result<i128> {
    let TaskData { signal } = parse_input(input)?;
    let mut index = 0;
    for (i, c) in signal.windows(4).enumerate() {
        if only_unique(c) {
            index = i;
            break;
        }
    }
    Ok(index as i128 + 4)
}

fn part_two(input: &str) -> Result<i128> {
    let TaskData { signal } = parse_input(input)?;
    let mut index = 0;
    for (i, c) in signal.windows(14).enumerate() {
        if only_unique2(c) {
            index = i;
            break;
        }
    }
    Ok(index as i128 + 14)
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
