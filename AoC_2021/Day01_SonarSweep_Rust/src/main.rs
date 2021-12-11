use std::error::Error;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    let input = parse_buffer(&buffer)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
    Ok(())
}

fn parse_buffer(buffer: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let parsed = buffer
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;
    Ok(parsed)
}

fn part_one(input: &Vec<i64>) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part_two(input: &Vec<i64>) -> usize {
    input.windows(4).filter(|w| w[0] < w[3]).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Once;

    static INIT: Once = Once::new();
    static mut TEST: Vec<i64> = Vec::new();
    static mut INPUT: Vec<i64> = Vec::new();

    fn init() {
        unsafe {
            INIT.call_once(|| {
                TEST = read_from_file("test.txt");
                INPUT = read_from_file("input.txt");
            });
        }
    }

    fn read_from_file(filename: &str) -> Vec<i64> {
        let buffer = fs::read_to_string(filename)
            .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg));

        parse_buffer(&buffer).unwrap_or_else(|msg| panic!("error parsing {}: {}", filename, msg))
    }

    fn get_test() -> &'static Vec<i64> {
        unsafe {
            init();
            &TEST
        }
    }

    fn get_input() -> &'static Vec<i64> {
        unsafe {
            init();
            &INPUT
        }
    }

    #[test]
    fn part_one_on_test() {
        let test = get_test();
        let answer = part_one(&test);
        assert_eq!(answer, 7);
    }
    #[test]
    fn part_one_on_input() {
        let input = get_input();
        let answer = part_one(&input);
        assert_eq!(answer, 1477);
    }
    #[test]
    fn part_two_on_test() {
        let test = get_test();
        let answer = part_two(&test);
        assert_eq!(answer, 5);
    }
    #[test]
    fn part_two_on_input() {
        let input = get_input();
        let answer = part_two(&input);
        assert_eq!(answer, 1523);
    }
}
