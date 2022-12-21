use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Read};
use std::rc::Rc;

enum MonkeyKind {
    Num(i128),
    Op(char),
}

struct Monkey {
    name: String,
    kind: MonkeyKind,
}

struct TaskData {
    monkeys: HashMap<String, Monkey>,
    monkey_connections: HashMap<String, Vec<String>>,
}

impl TaskData {
    fn yell_from(&mut self, monkey_name: &str) -> i128 {
        let monkey = &self.monkeys[monkey_name];
        match monkey.kind {
            MonkeyKind::Num(yell) => yell,
            MonkeyKind::Op(op) => {
                let monkeys = self.monkey_connections[monkey_name].clone();
                let a = self.yell_from(&monkeys[0]);
                let b = self.yell_from(&monkeys[1]);
                match op {
                    '*' => a * b,
                    '/' => a / b,
                    '+' => a + b,
                    '-' => a - b,
                    _ => panic!("unknown operand: {}", op),
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    lazy_static! {
        static ref OP_MONKEY: Regex = Regex::new(r"(\w+): (\w+) ([+\-*/]) (\w+)").unwrap();
        static ref NUM_MONKEY: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
    }
    let mut monkeys = HashMap::new();
    let mut monkey_connections = HashMap::new();
    for line in input.lines() {
        if OP_MONKEY.is_match(line) {
            let caps = OP_MONKEY.captures(line).unwrap();
            let name = String::from(&caps[1]);
            let a = String::from(&caps[2]);
            let op = caps[3].chars().nth(0).unwrap();
            let b = String::from(&caps[4]);
            monkeys.insert(
                name.clone(),
                Monkey {
                    name: name.clone(),
                    kind: MonkeyKind::Op(op),
                },
            );
            monkey_connections.insert(name, vec![a, b]);
        } else if NUM_MONKEY.is_match(line) {
            let caps = NUM_MONKEY.captures(line).unwrap();
            let name = String::from(&caps[1]);
            let num = i128::from_str_radix(&caps[2], 10).unwrap();
            monkeys.insert(
                name.clone(),
                Monkey {
                    name: name.clone(),
                    kind: MonkeyKind::Num(num),
                },
            );
        } else {
            panic!("What monkey is this?");
        }
    }
    Ok(TaskData {
        monkeys,
        monkey_connections,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    let answer = data.yell_from("root");
    Ok(answer)
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
