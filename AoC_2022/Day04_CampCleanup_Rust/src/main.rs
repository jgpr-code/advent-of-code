use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

struct Range {
    begin: i128,
    end: i128,
}

impl From<(&str, &str)> for Range {
    fn from(range_strs: (&str, &str)) -> Self {
        let begin = i128::from_str_radix(range_strs.0, 10).unwrap();
        let end = i128::from_str_radix(range_strs.1, 10).unwrap();
        Range { begin, end }
    }
}

impl Range {
    fn contains_range(&self, other: &Range) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }
    fn overlaps_range(&self, other: &Range) -> bool {
        self.begin <= other.begin && other.begin <= self.end
            || self.begin <= other.end && other.end <= self.end
    }
    fn some_range_contained(ranges: &(Range, Range)) -> bool {
        ranges.0.contains_range(&ranges.1) || ranges.1.contains_range(&ranges.0)
    }
    fn some_range_overlapped(ranges: &(Range, Range)) -> bool {
        ranges.0.overlaps_range(&ranges.1) || ranges.1.overlaps_range(&ranges.0)
    }
}

struct TaskData {
    elf_pairs: Vec<(Range, Range)>,
}

fn parse_input(input: &str) -> Result<TaskData> {
    // 1-2,3-4
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }

    let elf_pairs: Vec<_> = input
        .lines()
        .map(|l| {
            let caps = RE.captures(l).unwrap();
            (
                Range::from((&caps[1], &caps[2])),
                Range::from((&caps[3], &caps[4])),
            )
        })
        .collect();
    Ok(TaskData { elf_pairs })
}

fn part_one(input: &str) -> Result<i128> {
    let TaskData { elf_pairs } = parse_input(input)?;
    let contained = elf_pairs
        .iter()
        .map(|p| Range::some_range_contained(p) as i128)
        .sum();
    Ok(contained)
}

fn part_two(input: &str) -> Result<i128> {
    let TaskData { elf_pairs } = parse_input(input)?;
    let contained = elf_pairs
        .iter()
        .map(|p| Range::some_range_overlapped(p) as i128)
        .sum();
    Ok(contained)
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
        assert_eq!(answer, 2);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 459);
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
