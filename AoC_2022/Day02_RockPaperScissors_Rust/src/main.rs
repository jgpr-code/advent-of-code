use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

fn char_to_choice(c: char) -> Choice {
    use Choice::*;
    match c {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissor,
        _ => panic!("invalid choice"),
    }
}

fn chars_to_choices(match_up: &(char, char)) -> (Choice, Choice) {
    // X lose, Y draw, Z win
    use Choice::*;
    match match_up {
        &('A', 'X') => (Rock, Scissor),
        &('A', 'Y') => (Rock, Rock),
        &('A', 'Z') => (Rock, Paper),
        &('B', 'X') => (Paper, Rock),
        &('B', 'Y') => (Paper, Paper),
        &('B', 'Z') => (Paper, Scissor),
        &('C', 'X') => (Scissor, Paper),
        &('C', 'Y') => (Scissor, Scissor),
        &('C', 'Z') => (Scissor, Rock),
        _ => panic!("invalid choice"),
    }
}

fn get_your_score(opponent: Choice, you: Choice) -> i128 {
    use Choice::*;
    let match_score = match (opponent, you) {
        (Rock, Paper) | (Paper, Scissor) | (Scissor, Rock) => 6,
        (Rock, Rock) | (Paper, Paper) | (Scissor, Scissor) => 3,
        _ => 0,
    };
    let choice_score = match you {
        Rock => 1,
        Paper => 2,
        Scissor => 3,
    };
    match_score + choice_score
}

struct TaskData {
    matches: Vec<(char, char)>,
}

impl TaskData {
    fn get_final_score(&self) -> i128 {
        self.matches
            .iter()
            .map(|m| get_your_score(char_to_choice(m.0), char_to_choice(m.1)))
            .sum()
    }
    fn get_final_score_part_two(&self) -> i128 {
        self.matches
            .iter()
            .map(|m| {
                let (opponent, you) = chars_to_choices(m);
                get_your_score(opponent, you)
            })
            .sum()
    }
}

fn parse_input(buffer: &str) -> Result<TaskData> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    }
    let matches = buffer
        .lines()
        .map(|l| {
            let caps = RE.captures(l).unwrap();
            (
                caps[1].chars().nth(0).unwrap(),
                caps[2].chars().nth(0).unwrap(),
            )
        })
        .collect::<Vec<(char, char)>>();

    Ok(TaskData { matches })
}

fn part_one(input: &str) -> Result<i128> {
    let data = parse_input(input)?;

    Ok(data.get_final_score())
}

fn part_two(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    Ok(data.get_final_score_part_two())
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
        assert_eq!(answer, 15);
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
        assert_eq!(answer, 12);
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
