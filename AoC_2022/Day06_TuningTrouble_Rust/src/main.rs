use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::{self, Read};

struct TaskData {
    signal: Vec<char>,
}

fn parse_input(input: &str) -> Result<TaskData> {
    let signal: Vec<char> = input.lines().next().unwrap().chars().collect();
    Ok(TaskData { signal })
}

fn first_start_of_packet_marker(signal: &[char]) -> Result<usize> {
    let marker_len = 4;
    find_marker(marker_len, signal)
}

fn first_start_of_message_marker(signal: &[char]) -> Result<usize> {
    let marker_len = 14;
    find_marker(marker_len, signal)
}

fn find_marker(marker_len: usize, signal: &[char]) -> Result<usize> {
    let mut found = false;
    let mut index = marker_len;
    for (i, cs) in signal.windows(marker_len).enumerate() {
        if is_unique_chars(cs) {
            index += i;
            found = true;
            break;
        }
    }
    if found {
        Ok(index)
    } else {
        Err(anyhow!("Marker not found"))
    }
}

fn is_unique_chars(cs: &[char]) -> bool {
    let mut encountered: HashSet<char> = HashSet::new();
    for &c in cs {
        if encountered.contains(&c) {
            return false;
        }
        encountered.insert(c);
    }
    true
}

fn part_one(input: &str) -> Result<usize> {
    let TaskData { signal } = parse_input(input)?;
    Ok(first_start_of_packet_marker(&signal)?)
}

fn part_two(input: &str) -> Result<usize> {
    let TaskData { signal } = parse_input(input)?;
    Ok(first_start_of_message_marker(&signal)?)
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
        assert_eq!(answer, 7);
        Ok(())
    }
    #[test]
    fn test_one_1() -> Result<()> {
        let answer = super::part_one("bvwbjplbgvbhsrlpgdmjqwftvncz")?;
        assert_eq!(answer, 5);
        Ok(())
    }
    #[test]
    fn test_one_2() -> Result<()> {
        let answer = super::part_one("nppdvjthqldpwncqszvftbrmjlhg")?;
        assert_eq!(answer, 6);
        Ok(())
    }
    #[test]
    fn test_one_3() -> Result<()> {
        let answer = super::part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?;
        assert_eq!(answer, 10);
        Ok(())
    }
    #[test]
    fn test_one_4() -> Result<()> {
        let answer = super::part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?;
        assert_eq!(answer, 11);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 1198);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 19);
        Ok(())
    }
    #[test]
    fn test_two_1() -> Result<()> {
        let answer = super::part_two("bvwbjplbgvbhsrlpgdmjqwftvncz")?;
        assert_eq!(answer, 23);
        Ok(())
    }
    #[test]
    fn test_two_2() -> Result<()> {
        let answer = super::part_two("nppdvjthqldpwncqszvftbrmjlhg")?;
        assert_eq!(answer, 23);
        Ok(())
    }
    #[test]
    fn test_two_3() -> Result<()> {
        let answer = super::part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?;
        assert_eq!(answer, 29);
        Ok(())
    }
    #[test]
    fn test_two_4() -> Result<()> {
        let answer = super::part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?;
        assert_eq!(answer, 26);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 3120);
        Ok(())
    }
}
