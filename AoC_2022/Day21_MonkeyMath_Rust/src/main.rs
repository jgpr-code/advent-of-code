use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
enum MonkeyKind {
    Num(i128),
    Op(char),
}

#[derive(Debug)]
struct Monkey {
    name: String,
    kind: MonkeyKind,
    influenced_by_human: bool,
}

#[derive(Debug)]
struct TaskData {
    monkeys: HashMap<String, Monkey>,
    monkey_connections: HashMap<String, Vec<String>>,
}

impl TaskData {
    fn yell_from(&self, monkey_name: &str) -> i128 {
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
    fn update_influenced_by_human(&mut self, monkey_name: &str) -> bool {
        if monkey_name == "humn" {
            return true;
        }
        let monkey = &self.monkeys[monkey_name];
        match monkey.kind {
            MonkeyKind::Num(_) => return false,
            _ => (),
        }
        let monkeys = self.monkey_connections[monkey_name].clone();
        let a = self.update_influenced_by_human(&monkeys[0]);
        let b = self.update_influenced_by_human(&monkeys[1]);
        if a && b {
            panic!("human influence to great");
        }
        let influenced = a || b;
        let monkey = self.monkeys.get_mut(monkey_name).unwrap();
        monkey.influenced_by_human = influenced;
        return influenced;
    }
    fn solve_for_left(right: i128, op: char, target: i128) -> i128 {
        // x 'op' right == target
        // x - right == target => x == target + right
        // x + right == target => x == target - right
        // x * right == target => x == target / right
        // x / right == target => x == target * right
        match op {
            '-' => target + right,
            '+' => target - right,
            '*' => target / right,
            '/' => target * right,
            _ => panic!("can't solve for unknown op"),
        }
    }
    fn solve_for_right(left: i128, op: char, target: i128) -> i128 {
        // left 'op' x == target
        // left - x == target => x == left - target
        // left + x == target => x == target - left
        // left * x == target => x == target / left
        // left / x == target => x == left / target
        match op {
            '-' => left - target,
            '+' => target - left,
            '*' => target / left,
            '/' => left / target,
            _ => panic!("can't solve for unknown op"),
        }
    }
    fn find_human_yell(&self, monkey_name: &str, target: i128) -> i128 {
        let monkey = &self.monkeys[monkey_name];
        let monkeys = self.monkey_connections[monkey_name].clone();
        let (left_name, right_name) = (monkeys[0].clone(), monkeys[1].clone());
        let left_monkey = &self.monkeys[&left_name];
        let _right_monkey = &self.monkeys[&right_name];
        let humn = String::from("humn");
        let op = match monkey.kind {
            MonkeyKind::Op(c) => c,
            _ => panic!("find_human_yell called on leave"),
        };
        if left_name == humn {
            let right_value = self.yell_from(&right_name);
            return Self::solve_for_left(right_value, op, target);
        } else if right_name == humn {
            let left_value = self.yell_from(&left_name);
            return Self::solve_for_right(left_value, op, target);
        } else {
            if left_monkey.influenced_by_human {
                let right_val = self.yell_from(&right_name);
                let next_target = Self::solve_for_left(right_val, op, target);
                return self.find_human_yell(&left_name, next_target);
            } else {
                let left_val = self.yell_from(&left_name);
                let next_target = Self::solve_for_right(left_val, op, target);
                return self.find_human_yell(&right_name, next_target);
            }
        }
    }
    fn what_should_human_yell(&mut self) -> i128 {
        self.update_influenced_by_human("root");
        let monkeys = self.monkey_connections["root"].clone();
        let a = &self.monkeys[&monkeys[0]];
        let b = &self.monkeys[&monkeys[1]];
        if a.influenced_by_human && b.influenced_by_human {
            panic!("too much human influence");
        }
        if a.influenced_by_human {
            let target = self.yell_from(&b.name);
            self.find_human_yell(&a.name, target)
        } else {
            let target = self.yell_from(&a.name);
            self.find_human_yell(&b.name, target)
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
                    influenced_by_human: false,
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
                    influenced_by_human: false,
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
    let data = parse_input(input)?;
    let answer = data.yell_from("root");
    Ok(answer)
}

fn _brute_force_part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    let mut answer = -1;
    let root = data.monkeys.get_mut("root").unwrap();
    root.kind = MonkeyKind::Op('-');
    for try_yell in (1000000..i128::MAX).rev() {
        let human = data.monkeys.get_mut("humn").unwrap();
        human.kind = MonkeyKind::Num(try_yell);
        if data.yell_from("root") == 0 {
            answer = try_yell;
            break;
        }
    }
    Ok(answer)
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    let answer = data.what_should_human_yell();
    Ok(answer)
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
        assert_eq!(answer, 152);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 83056452926300);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 301);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 3469704905529);
        Ok(())
    }
}
