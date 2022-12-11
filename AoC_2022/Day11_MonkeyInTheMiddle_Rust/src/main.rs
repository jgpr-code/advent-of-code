use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{self, Read};

struct Monkey {
    num: usize,
    items: VecDeque<i128>,
    worry_fn: Box<dyn Fn(i128) -> i128>,
    divisor_for_test: i128,
    throw_true: usize,
    throw_false: usize,
    inspection_count: i128,
}

impl Monkey {
    fn print(&self) {
        println!("Monkey: {}", self.num);
        println!("Items: {:?}", self.items);
        println!("Test: {}", self.divisor_for_test);
        println!("True: {}", self.throw_true);
        println!("False: {}", self.throw_false);
    }
}

impl From<&str> for Monkey {
    fn from(monkey_lines: &str) -> Self {
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        lazy_static! {
            static ref MONKEY: Regex = Regex::new(r"Monkey (\d+)").unwrap();
            static ref ITEM: Regex = Regex::new(r"(\d+)").unwrap();
            static ref OP: Regex =
                Regex::new(r"Operation: new = (old|\d+) ([+*]) (old|\d+)").unwrap();
            static ref TEST: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
            static ref TRUE: Regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
            static ref FALSE: Regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        }
        // Name
        let mut line_it = monkey_lines.lines();
        let caps = MONKEY.captures(line_it.next().unwrap()).unwrap();
        let monkey_num = usize::from_str_radix(&caps[1], 10).unwrap();

        // Items
        let item_line = line_it.next().unwrap();
        let mut monkey_items: VecDeque<i128> = VecDeque::new();
        for caps in ITEM.captures_iter(item_line) {
            monkey_items.push_back(i128::from_str_radix(&caps[1], 10).unwrap());
        }

        // OP
        let caps = OP.captures(line_it.next().unwrap()).unwrap();
        let op1 = &caps[1];
        let op = &caps[2];
        let op2 = &caps[3];
        let worry_fn: Box<dyn Fn(i128) -> i128> = match (op1, op, op2) {
            ("old", "+", "old") => Box::new(move |x| x + x),
            ("old", "*", "old") => Box::new(move |x| x * x),
            ("old", "+", s) => {
                let op2 = i128::from_str_radix(s, 10).unwrap();
                Box::new(move |x| x + op2)
            }
            ("old", "*", s) => {
                let op2 = i128::from_str_radix(s, 10).unwrap();
                Box::new(move |x| x * op2)
            }
            _ => panic!("unhandled OP"),
        };

        // TEST
        let caps = TEST.captures(line_it.next().unwrap()).unwrap();
        let divisor_for_test = i128::from_str_radix(&caps[1], 10).unwrap();
        let caps = TRUE.captures(line_it.next().unwrap()).unwrap();
        let throw_true = usize::from_str_radix(&caps[1], 10).unwrap();
        let caps = FALSE.captures(line_it.next().unwrap()).unwrap();
        let throw_false = usize::from_str_radix(&caps[1], 10).unwrap();

        Monkey {
            num: monkey_num,
            items: monkey_items,
            worry_fn,
            divisor_for_test,
            throw_true,
            throw_false,
            inspection_count: 0,
        }
    }
}

struct TaskData {
    current_monkey: usize,
    monkeys: Vec<Monkey>,
    round: usize,
}

impl TaskData {
    fn inspect_items_until_rounds(&mut self, rounds: usize) {
        loop {
            let current = self.current_monkey;
            while let Some(item) = self.monkeys[current].items.pop_front() {
                let func = self.monkeys[current].worry_fn.as_ref();
                //let new_worry = func(item);
                let new_worry = func(item) / 3;
                let throw_to = if new_worry % self.monkeys[current].divisor_for_test == 0 {
                    self.monkeys[current].throw_true
                } else {
                    self.monkeys[current].throw_false
                };
                self.monkeys[throw_to].items.push_back(new_worry);
                self.monkeys[current].inspection_count += 1;
            }
            self.current_monkey += 1;
            if self.current_monkey % self.monkeys.len() == 0 {
                self.current_monkey = 0;
                self.round += 1;
                if self.round == rounds {
                    return;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let monkeys = input.split("\r\n\r\n").map(|s| Monkey::from(s)).collect();
    Ok(TaskData {
        current_monkey: 0,
        monkeys,
        round: 0,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    for m in data.monkeys.iter() {
        m.print();
        println!("");
    }
    data.inspect_items_until_rounds(20);
    let mut r: Vec<i128> = data.monkeys.iter().map(|m| m.inspection_count).collect();
    r.sort_by(|a, b| b.cmp(a));
    println!("{:?}", r);
    Ok(r[0] * r[1])
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    for m in data.monkeys.iter() {
        m.print();
        println!("");
    }
    data.inspect_items_until_rounds(10000);
    let mut r: Vec<i128> = data.monkeys.iter().map(|m| m.inspection_count).collect();
    r.sort_by(|a, b| b.cmp(a));
    println!("{:?}", r);
    Ok(r[0] * r[1])
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
        assert_eq!(answer, 10605);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 120756);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 2713310158);
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
