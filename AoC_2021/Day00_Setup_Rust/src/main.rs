// try replicating Day02_PasswordPhilosophy from AoC_2020

use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

struct PasswordPolicy {
    c: char,
    min_c: usize,
    max_c: usize,
}

struct Password {
    password: String,
    policy: PasswordPolicy,
}

fn parse_line(line: &str) -> Password {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([1-9]\d*)-([0-9]*) ([a-z]): ([a-z]*)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    Password {
        password: caps.get(4).unwrap().as_str().to_string(),
        policy: PasswordPolicy {
            c: caps.get(3).unwrap().as_str().chars().next().unwrap(),
            min_c: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            max_c: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        },
    }
}

fn is_valid_password(password: &Password) -> bool {
    let count_c = password
        .password
        .chars()
        .filter(|c| *c == password.policy.c)
        .count();
    password.policy.min_c <= count_c && count_c <= password.policy.max_c
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    let mut amount_valid = 0;
    for line in buffer.lines() {
        let password = parse_line(line);
        if is_valid_password(&password) {
            amount_valid += 1;
        }
    }

    println!("Part 1: {}", amount_valid);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
