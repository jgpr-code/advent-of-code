use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

struct TaskData {
    tree_grid: Vec<Vec<i128>>,
}

fn max(a: i128, b: i128) -> i128 {
    if a > b {
        a
    } else {
        b
    }
}

impl TaskData {
    fn get_scenic_score(&self, treehouse_row: usize, treehouse_col: usize) -> i128 {
        let rows = self.tree_grid.len();
        let cols = self.tree_grid[0].len();
        let treehouse_height = self.tree_grid[treehouse_row][treehouse_col];

        let mut up = 0;
        for row in (0..treehouse_row).rev() {
            let current_tree = self.tree_grid[row][treehouse_col];
            if current_tree < treehouse_height {
                up += 1;
            }
            if current_tree >= treehouse_height {
                up += 1;
                break;
            }
        }
        let mut down = 0;
        for row in treehouse_row + 1..rows {
            let current_tree = self.tree_grid[row][treehouse_col];
            if current_tree < treehouse_height {
                down += 1;
            }
            if current_tree >= treehouse_height {
                down += 1;
                break;
            }
        }
        let mut left = 0;
        for col in (0..treehouse_col).rev() {
            let current_tree = self.tree_grid[treehouse_row][col];
            if current_tree < treehouse_height {
                left += 1;
            }
            if current_tree >= treehouse_height {
                left += 1;
                break;
            }
        }
        let mut right = 0;
        for col in treehouse_col + 1..cols {
            let current_tree = self.tree_grid[treehouse_row][col];
            if current_tree < treehouse_height {
                right += 1;
            }
            if current_tree >= treehouse_height {
                right += 1;
                break;
            }
        }
        // println!("(row,col) = (up,down,left,right)");
        // println!(
        //     "({},{}) = ({},{},{},{})",
        //     treehouse_row, treehouse_col, up, down, left, right
        // );
        up * down * left * right
    }
    fn best_scenic_score(&self) -> i128 {
        let rows = self.tree_grid.len();
        let cols = self.tree_grid[0].len();
        let mut best_score = -1;
        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                best_score = max(best_score, self.get_scenic_score(row, col))
            }
        }
        best_score
    }

    fn count_visible(&self) -> i128 {
        let rows = self.tree_grid.len();
        let cols = self.tree_grid[0].len();
        // count from each side, don't count twice
        let mut counted: HashSet<(usize, usize)> = HashSet::new();
        let mut total = 0;
        // top
        for col in 0..cols {
            let mut min_size = -1;
            for row in 0..rows {
                // already found largest tree
                if counted.contains(&(row, col)) {
                    min_size = max(min_size, self.tree_grid[row][col]);
                    continue;
                }
                if self.tree_grid[row][col] > min_size {
                    total += 1;
                    min_size = max(min_size, self.tree_grid[row][col]);
                    counted.insert((row, col));
                }
            }
        }
        // bottom
        for col in 0..cols {
            let mut min_size = -1;
            for row in (0..rows).rev() {
                // already found largest tree
                if counted.contains(&(row, col)) {
                    min_size = max(min_size, self.tree_grid[row][col]);
                    continue;
                }
                if self.tree_grid[row][col] > min_size {
                    total += 1;
                    min_size = max(min_size, self.tree_grid[row][col]);
                    counted.insert((row, col));
                }
            }
        }
        // left
        for row in 0..rows {
            let mut min_size = -1;
            for col in 0..cols {
                if counted.contains(&(row, col)) {
                    min_size = max(min_size, self.tree_grid[row][col]);
                    continue;
                }
                if self.tree_grid[row][col] > min_size {
                    total += 1;
                    min_size = max(min_size, self.tree_grid[row][col]);
                    counted.insert((row, col));
                }
            }
        }
        // right
        for row in 0..rows {
            let mut min_size = -1;
            for col in (0..cols).rev() {
                if counted.contains(&(row, col)) {
                    min_size = max(min_size, self.tree_grid[row][col]);
                    continue;
                }
                if self.tree_grid[row][col] > min_size {
                    total += 1;
                    min_size = max(min_size, self.tree_grid[row][col]);
                    counted.insert((row, col));
                }
            }
        }
        let debug: Vec<(usize, usize)> = counted
            .into_iter()
            .filter(|&(a, b)| a != 0 && a != rows - 1 && b != 0 && b != cols - 1)
            .collect();
        println!("{:?}", debug);
        total
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let mut tree_grid = Vec::new();
    for line in input.lines() {
        let line_heights: Vec<i128> = line
            .chars()
            .map(|c| (c as u8 - '0' as u8) as i128)
            .collect();
        tree_grid.push(line_heights);
    }
    Ok(TaskData { tree_grid })
}

fn part_one(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    Ok(data.count_visible())
}

fn part_two(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    Ok(data.best_scenic_score())
}

fn main() -> Result<()> {
    println!("{}", part_one("00000\n00300\n00000")?);
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
        assert_eq!(answer, 21);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 1690);
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
