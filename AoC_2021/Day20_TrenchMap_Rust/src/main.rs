use anyhow;
use std::io::{self, Read};

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    let mut input = parse_buffer(&buffer);
    println!("Part 1: {}", part_one(&mut input));
    println!("Part 2: {}", part_two(&mut input));
    Ok(())
}

#[derive(Debug)]
struct EnhanceableGrid {
    enhanced_mapping: Vec<char>,
    content: Vec<Vec<char>>,
    amount_enhanced: i32,
    is_swapping: bool,
}

impl From<&str> for EnhanceableGrid {
    fn from(buffer: &str) -> EnhanceableGrid {
        let splitted: Vec<&str> = buffer.split("\r\n\r\n").collect();

        let enhanced_mapping: Vec<char> = splitted[0].chars().collect();

        let content: Vec<Vec<char>> = splitted[1].lines().map(|l| l.chars().collect()).collect();

        let is_swapping = match (enhanced_mapping.first(), enhanced_mapping.last()) {
            (Some(c1), Some(c2)) if *c1 == '#' && *c2 == '.' => true,
            _ => false,
        };

        EnhanceableGrid {
            enhanced_mapping,
            content,
            amount_enhanced: 0,
            is_swapping,
        }
    }
}

impl EnhanceableGrid {
    fn rows(&self) -> i32 {
        self.content.len() as i32
    }
    fn cols(&self) -> i32 {
        self.content.get(0).map_or(0, |v| v.len()) as i32
    }
    fn is_inside(&self, row: i32, col: i32) -> bool {
        0 <= row && row < self.rows() && 0 <= col && col < self.cols()
    }
    fn get_at(&self, row: i32, col: i32) -> char {
        if !self.is_inside(row, col) {
            if self.is_swapping {
                if self.amount_enhanced % 2 == 0 {
                    '.'
                } else {
                    '#'
                }
            } else {
                '.'
            }
        } else {
            self.content[row as usize][col as usize]
        }
    }

    fn kernel_replacement(&self, kernel: &Vec<char>) -> char {
        let mut index = 0;
        let mut value = 1;
        for elem in kernel.iter().rev() {
            if *elem == '#' {
                index += value;
            }
            value *= 2;
        }
        self.enhanced_mapping[index]
    }

    fn enhance(&mut self) {
        let new_rows = self.rows() as usize + 2;
        let new_cols = self.cols() as usize + 2;
        let mut new_content: Vec<Vec<char>> = vec![vec!['.'; new_cols]; new_rows];
        let delta = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for row in 0..new_rows as i32 {
            for col in 0..new_cols as i32 {
                let mut kernel = Vec::new();
                for (drow, dcol) in delta.iter() {
                    kernel.push(self.get_at(row - 1 + drow, col - 1 + dcol));
                }
                new_content[row as usize][col as usize] = self.kernel_replacement(&kernel);
            }
        }
        self.content = new_content;
        self.amount_enhanced += 1;
    }

    fn count_hashes(&self) -> i64 {
        let mut amount = 0;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                if self.content[row as usize][col as usize] == '#' {
                    amount += 1;
                }
            }
        }
        amount
    }

    fn print_content(&self) {
        for row in 0..self.rows() {
            println!(
                "{:?}",
                self.content[row as usize].iter().collect::<String>()
            );
        }
        println!();
    }
}

fn parse_buffer(buffer: &str) -> EnhanceableGrid {
    EnhanceableGrid::from(buffer)
}

fn part_one(grid: &mut EnhanceableGrid) -> i64 {
    println!("{:?} {:?}", grid.rows(), grid.cols());
    grid.print_content();
    grid.enhance();
    grid.print_content();
    grid.enhance();
    grid.print_content();
    println!("{:?} {:?}", grid.rows(), grid.cols());
    grid.count_hashes()
}

fn part_two(grid: &mut EnhanceableGrid) -> i64 {
    for _ in 0..48 {
        grid.enhance();
    }
    grid.count_hashes()
}
