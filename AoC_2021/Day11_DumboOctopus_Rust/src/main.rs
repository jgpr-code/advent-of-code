use std::collections::HashSet;
use std::io::{self, Read};
use thiserror::Error as ThisError;
// use anyhow::Error as AnyhowError;

#[derive(ThisError, Debug)]
enum AocError {
    #[error("io")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("could not convert char to digit: {0}")]
    InvalidCharToDigit(char),
    /*
    #[error(transparent)]
    Other(#[from] AnyhowError),
    */
}

fn main() -> Result<(), AocError> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    let input = parse_buffer(&buffer)?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
    Ok(())
}

#[derive(Debug)]
struct DumboOctopusGrid {
    grid: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl DumboOctopusGrid {
    fn new(grid: Vec<Vec<u32>>) -> DumboOctopusGrid {
        let rows = grid.
    }
    fn simulate_day(&mut self) {
        let offsets: Vec<(i32, i32)> = vec![
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];
        let already_flashing: HashSet<(i32, i32)> = HashSet::new();
    }
    fn is_inside(&self, pos: (i32, i32)) -> bool {
        0 <= pos.0 && pos.0 < self.rows as i32 && 0 <= pos.1 && pos.1 < self.cols as i32
    }
}

fn parse_buffer(buffer: &str) -> Result<DumboOctopusGrid, AocError> {
    let parsed = buffer
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).ok_or(AocError::InvalidCharToDigit(c)))
                .collect::<Result<Vec<u32>, AocError>>()
        })
        .collect::<Result<Vec<Vec<u32>>, AocError>>()?;

    Ok(DumboOctopusGrid { grid: parsed })
}

fn part_one(input: &DumboOctopusGrid) -> usize {
    println!("{:?}", input);
    0
}

fn part_two(input: &DumboOctopusGrid) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Once;

    static INIT: Once = Once::new();
    static mut TEST: DumboOctopusGrid = DumboOctopusGrid { grid: Vec::new() };
    static mut INPUT: DumboOctopusGrid = DumboOctopusGrid { grid: Vec::new() };

    fn init() {
        unsafe {
            INIT.call_once(|| {
                TEST = read_from_file("test.txt");
                INPUT = read_from_file("input.txt");
            });
        }
    }

    fn read_from_file(filename: &str) -> DumboOctopusGrid {
        let buffer = fs::read_to_string(filename)
            .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg));

        parse_buffer(&buffer).unwrap_or_else(|msg| panic!("error parsing {}: {}", filename, msg))
    }

    fn get_test() -> &'static DumboOctopusGrid {
        unsafe {
            init();
            &TEST
        }
    }

    fn get_input() -> &'static DumboOctopusGrid {
        unsafe {
            init();
            &INPUT
        }
    }

    #[test]
    fn part_one_on_test() {
        let test = get_test();
        let answer = part_one(&test);
        assert_eq!(answer, 1656);
    }
    #[test]
    fn part_one_on_input() {
        let input = get_input();
        let answer = part_one(&input);
        assert_eq!(answer, 0);
    }
    #[test]
    fn part_two_on_test() {
        let test = get_test();
        let answer = part_two(&test);
        assert_eq!(answer, 0);
    }
    #[test]
    fn part_two_on_input() {
        let input = get_input();
        let answer = part_two(&input);
        assert_eq!(answer, 0);
    }
}
