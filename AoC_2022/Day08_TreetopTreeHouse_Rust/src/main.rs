use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

struct TaskData {
    tree_grid: Vec<Vec<i128>>,
    rows: usize,
    cols: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_row_col(&self, xy: (usize, usize)) -> (usize, usize) {
        let (x, y) = xy;
        match self {
            Direction::Up | Direction::Down => (y, x),
            Direction::Left | Direction::Right => (x, y),
        }
    }
}

fn max(a: i128, b: i128) -> i128 {
    if a > b {
        a
    } else {
        b
    }
}

impl TaskData {
    fn count_visible_from_treehouse_in_direction(
        &self,
        treehouse: (usize, usize),
        direction: Direction,
    ) -> i128 {
        use Direction::*;
        let (treehouse_row, treehouse_col) = treehouse;
        let treehouse_height = self.tree_grid[treehouse_row][treehouse_col];
        let range_vec: Vec<usize> = match direction {
            Up => (0..treehouse_row).rev().collect(),
            Down => (treehouse_row + 1..self.rows).collect(),
            Left => ((0..treehouse_col).rev()).collect(),
            Right => (treehouse_col + 1..self.cols).collect(),
        };
        let mut visible_trees = 0;
        for i in range_vec.into_iter() {
            let index = match direction {
                Up | Down => (i, treehouse_col),
                Left | Right => (treehouse_row, i),
            };
            let current_tree = self.tree_grid[index.0][index.1];
            visible_trees += 1;
            if current_tree >= treehouse_height {
                break;
            }
        }
        visible_trees
    }
    fn get_scenic_score(&self, treehouse: (usize, usize)) -> i128 {
        use Direction::*;
        let up = self.count_visible_from_treehouse_in_direction(treehouse, Up);
        let down = self.count_visible_from_treehouse_in_direction(treehouse, Down);
        let left = self.count_visible_from_treehouse_in_direction(treehouse, Left);
        let right = self.count_visible_from_treehouse_in_direction(treehouse, Right);
        up * down * left * right
    }
    fn best_scenic_score(&self) -> i128 {
        let mut best_score = -1;
        for row in 1..self.rows - 1 {
            for col in 1..self.cols - 1 {
                best_score = max(best_score, self.get_scenic_score((row, col)));
            }
        }
        best_score
    }

    fn count_visible_from_direction(
        &self,
        direction: Direction,
        counted: &mut HashSet<(usize, usize)>,
        outer_iter: impl Iterator<Item = usize>,
        inner_iter: impl Iterator<Item = usize> + Clone,
    ) -> i128 {
        let mut total = 0;
        for x in outer_iter {
            let mut min_size = -1;
            for y in inner_iter.clone() {
                let (row, col) = direction.to_row_col((x, y));
                let current_tree = self.tree_grid[row][col];
                if counted.contains(&(row, col)) {
                    min_size = max(min_size, current_tree); // tree can be smaller!
                    continue;
                }
                if current_tree > min_size {
                    min_size = current_tree;
                    counted.insert((row, col));
                    total += 1;
                }
            }
        }
        total
    }

    fn count_visible_from_borders(&self) -> i128 {
        let (rows, cols) = (self.rows, self.cols);

        // count from each side, don't count twice
        let mut counted: HashSet<(usize, usize)> = HashSet::new();
        let mut total = 0;
        use Direction::*;
        total += self.count_visible_from_direction(Up, &mut counted, 0..cols, 0..rows);
        total += self.count_visible_from_direction(Down, &mut counted, 0..cols, (0..rows).rev());
        total += self.count_visible_from_direction(Left, &mut counted, 0..rows, 0..cols);
        total += self.count_visible_from_direction(Right, &mut counted, 0..rows, (0..cols).rev());
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
    let rows = tree_grid.len();
    let cols = tree_grid[0].len();
    Ok(TaskData {
        tree_grid,
        rows,
        cols,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    Ok(data.count_visible_from_borders())
}

fn part_two(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    Ok(data.best_scenic_score())
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
        assert_eq!(answer, 8);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 535680);
        Ok(())
    }
}
