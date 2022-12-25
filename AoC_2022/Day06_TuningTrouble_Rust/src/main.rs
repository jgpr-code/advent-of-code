use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
//use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

struct TaskData {
    signal: Vec<char>,
}

fn parse_input(input: &str) -> Result<TaskData> {
    let first_line = input.lines().next().context("input had no lines")?;
    let signal: Vec<char> = first_line.chars().collect();

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

fn find_marker_slow(marker_len: usize, signal: &[char]) -> Result<usize> {
    let mut found = false;
    let mut index = marker_len;
    for (i, cs) in signal.windows(marker_len).enumerate() {
        if is_unique_chars_slow(cs) {
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

// TODO: implement a really fast version

// struct FastMarkerFinder<const MARKER_LEN: usize> {
//     index: usize,
//     unique_count: usize,
//     current_chunk: VecDeque<char>,
//     encounters: HashMap<char, usize>,
// }
// a b c a
// count = 3
// unique = 2
// map {a: 2} {b: 1} {c: 1}
//   b c a b
// map {a: 1} {b: 2} {c: 1}
// unique 2 count 3

// fn find_marker_fast(marker_len: usize, signal: &[char]) -> usize {
//     let mut sig_iter = signal.iter();
//     let foo = sig_iter.take(marker_len);
//     for f in foo {}
//     let d: VecDeque<char> = VecDeque::from_iter(foo.cloned());
//     let set: HashSet<char> = HashSet::from_iter(foo.cloned());
//     0
// }

fn first_start_of_message_marker_slow(signal: &[char]) -> Result<usize> {
    let marker_len = 14;
    find_marker_slow(marker_len, signal)
}

fn is_unique_chars_slow(cs: &[char]) -> bool {
    let set: HashSet<char> = HashSet::from_iter(cs.iter().cloned());
    set.len() == cs.len()
}

fn part_two_slow(input: &str) -> Result<usize> {
    let TaskData { signal } = parse_input(input)?;
    Ok(first_start_of_message_marker_slow(&signal)?)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part one: {}", part_one(&input)?);
    println!("Part two: {}", part_two(&input)?);

    // worst case time test
    let mut worst_case = vec!['a'; 10000000];
    let mut end = vec![
        'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    ];
    worst_case.append(&mut end);
    let worst_case_str: String = worst_case.into_iter().collect();

    let t = std::time::Instant::now();
    let _ = part_two(&worst_case_str);
    println!("Part two {:0.2?}", t.elapsed());

    let t = std::time::Instant::now();
    let _ = part_two_slow(&worst_case_str);
    println!("Part two slow {:0.2?}", t.elapsed());
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

    // Use "cargo test --release part_two -- --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 3120);
        Ok(())
    }
    #[test]
    fn part_two_slow() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two_slow(&INPUT)?;
        eprintln!("Part two slow took {:0.2?}", t.elapsed());
        assert_eq!(answer, 3120);
        Ok(())
    }
}
