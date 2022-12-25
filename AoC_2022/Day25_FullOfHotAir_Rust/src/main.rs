use anyhow::Result;
use std::io::{self, Read};

struct TaskData {
    snafu_numbers: Vec<Vec<char>>,
}

fn snafu_to_i128(snafu: &[char]) -> i128 {
    let mut num = 0;
    let mut power = 1;
    for c in snafu.iter().rev() {
        match c {
            '2' => num += 2 * power,
            '1' => num += power,
            '0' => (),
            '-' => num -= power,
            '=' => num -= 2 * power,
            _ => panic!("invalid snafu number"),
        }
        power *= 5;
    }
    num
}

fn i128_to_snafu(num: i128) -> Vec<char> {
    let mut curr = num;
    let mut snafu = Vec::new();
    let mut carry = 0;
    while curr > 0 || carry != 0 {
        let modu = curr % 5;
        let snaf = match (modu, carry) {
            (0, 0) => ('0', 0),
            (1, 0) => ('1', 0),
            (2, 0) => ('2', 0),
            (3, 0) => ('=', 1),
            (4, 0) => ('-', 1),
            (0, 1) => ('1', 0),
            (1, 1) => ('2', 0),
            (2, 1) => ('=', 1),
            (3, 1) => ('-', 1),
            (4, 1) => ('0', 1),
            _ => panic!(
                "invalid state: curr:{}, modu:{}, carry:{}",
                curr, modu, carry
            ),
        };
        carry = snaf.1;
        snafu.push(snaf.0);
        curr /= 5;
    }
    // 0, 23 => (=, 1)
    // 1, 4 => (0, 1)
    // 1, 0 => (1, 0)
    // 64:
    // 64 % 5 == 4 => (-, 1)
    // 1, 12 % 5 == 2 => (2, 0)
    // 2 % 5 == 2 => (2, 1),
    // 1 => 1
    // 120-
    // 28 to snafu => %5 == 3 => (=, 1)
    // 5 to snafu => %5 == 0 => 0 + 1
    // 1 to snafu => %5 == 1 => 1
    // 28 is 11=
    snafu.reverse();
    snafu
}

fn parse_input(input: &str) -> Result<TaskData> {
    let snafu_numbers = input.lines().map(|l| l.chars().collect()).collect();
    Ok(TaskData { snafu_numbers })
}

fn part_one(input: &str) -> Result<String> {
    let TaskData { snafu_numbers } = parse_input(input)?;
    let mut sum = 0;
    for snafu in snafu_numbers {
        sum += snafu_to_i128(&snafu);
    }
    println!("dez: {}", sum);
    let snafu = i128_to_snafu(sum);
    Ok(String::from_iter(snafu.iter()))
}

fn part_two(input: &str) -> Result<i128> {
    let _ = parse_input(input)?;
    Ok(-1)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let t = std::time::Instant::now();
    let part_one = part_one(&input)?;
    let elapsed = t.elapsed();
    println!("Part one: {} in {:0.2?}", part_one, elapsed);
    let t = std::time::Instant::now();
    let part_two = part_two(&input)?;
    let elapsed = t.elapsed();
    println!("Part two: {} in {:0.2?}", part_two, elapsed);
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
        assert_eq!(answer, "0");
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, "0");
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
