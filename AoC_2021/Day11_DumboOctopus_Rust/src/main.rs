use anyhow::{Context, Result};
use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    let mut input_part_one = parse_buffer(&buffer)?;
    let mut input_part_two = input_part_one.clone();
    println!("Part 1: {}", part_one(&mut input_part_one));
    println!("Part 2: {}", part_two(&mut input_part_two));
    Ok(())
}

#[derive(Debug, Clone)]
struct DumboOctopusGrid {
    grid: Vec<Vec<u32>>,
}

impl DumboOctopusGrid {
    fn rows(&self) -> i32 {
        self.grid.len() as i32
    }

    fn cols(&self) -> i32 {
        self.grid.get(0).map_or(0, |v| v.len()) as i32
    }

    fn is_inside(&self, pos: (i32, i32)) -> bool {
        0 <= pos.0 && pos.0 < self.rows() && 0 <= pos.1 && pos.1 < self.cols()
    }

    fn value_at(&mut self, pos: (i32, i32)) -> Option<&mut u32> {
        if !self.is_inside(pos) {
            None
        } else {
            Some(&mut self.grid[pos.0 as usize][pos.1 as usize])
        }
    }

    fn simulate_day(&mut self) -> usize {
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
        let mut already_flashing: HashSet<(i32, i32)> = HashSet::new();
        let mut flashing_queue: VecDeque<(i32, i32)> = VecDeque::new();

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let pos = (row, col);
                if let Some(value) = self.value_at(pos) {
                    *value += 1;
                    if *value > 9 {
                        already_flashing.insert(pos);
                        flashing_queue.push_back(pos);
                    }
                }
            }
        }

        while let Some(pos) = flashing_queue.pop_front() {
            for offset in offsets.iter() {
                let neighbor_pos = (pos.0 + offset.0, pos.1 + offset.1);
                if already_flashing.contains(&neighbor_pos) {
                    continue;
                }
                if let Some(value) = self.value_at(neighbor_pos) {
                    *value += 1;
                    if *value > 9 {
                        already_flashing.insert(neighbor_pos);
                        flashing_queue.push_back(neighbor_pos);
                    }
                }
            }
        }
        for pos in already_flashing.iter() {
            if let Some(value) = self.value_at(*pos) {
                *value = 0
            }
        }
        already_flashing.iter().count()
    }
}

fn parse_buffer(buffer: &str) -> Result<DumboOctopusGrid> {
    let parsed = buffer
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).context("char was not a valid digit"))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;

    Ok(DumboOctopusGrid { grid: parsed })
}

fn part_one(input: &mut DumboOctopusGrid) -> usize {
    let mut flashes_total = 0;
    for _ in 0..100 {
        flashes_total += input.simulate_day();
    }
    flashes_total
}

fn part_two(input: &mut DumboOctopusGrid) -> usize {
    let mut day = 0;
    loop {
        day += 1;
        let amount = input.simulate_day();
        if amount == (input.rows() * input.cols()) as usize {
            break day;
        }
    }
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
        let mut test = get_test().clone();
        let answer = part_one(&mut test);
        assert_eq!(answer, 1656);
    }
    #[test]
    fn part_one_on_input() {
        let mut input = get_input().clone();
        let answer = part_one(&mut input);
        assert_eq!(answer, 1785);
    }
    #[test]
    fn part_two_on_test() {
        let mut test = get_test().clone();
        let answer = part_two(&mut test);
        assert_eq!(answer, 195);
    }
    #[test]
    fn part_two_on_input() {
        let mut input = get_input().clone();
        let answer = part_two(&mut input);
        assert_eq!(answer, 354);
    }
}
