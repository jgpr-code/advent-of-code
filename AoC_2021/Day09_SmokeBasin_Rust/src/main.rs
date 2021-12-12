use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    part_one(&buffer);
    part_two(&buffer);
    Ok(())
}

fn part_one(buffer: &str) {
    let grid = Grid::new(buffer);
    let risk_levels_sum: i32 = grid
        .low_points()
        .iter()
        .map(|(r, c)| grid.grid[*r as usize][*c as usize] + 1)
        .sum();

    println!("Part 1: {}", risk_levels_sum);
}

fn part_two(buffer: &str) {
    let grid = Grid::new(buffer);
    let low_points = grid.low_points();
    let mut basin_sizes: Vec<usize> = low_points.iter().map(|lp| grid.basin_size(*lp)).collect();
    basin_sizes.sort();

    println!("low_points: {:?}", low_points);
    println!("basin_sizes: {:?}", basin_sizes);
    let answer: usize = basin_sizes.iter().rev().take(3).product();
    println!("Part 2: {}", answer);
}

struct Grid {
    grid: Vec<Vec<i32>>,
    rows: i32,
    cols: i32,
    drow: Vec<i32>,
    dcol: Vec<i32>,
}

impl Grid {
    fn new(buffer: &str) -> Grid {
        let grid: Vec<Vec<i32>> = buffer
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
            .collect();

        let rows: i32 = grid.len() as i32;
        let cols: i32 = grid[0].len() as i32;
        // row, col
        // up, down, left, right
        let drow: Vec<i32> = vec![-1, 1, 0, 0];
        let dcol: Vec<i32> = vec![0, 0, -1, 1];
        Grid {
            grid,
            rows,
            cols,
            drow,
            dcol,
        }
    }

    fn low_points(&self) -> Vec<(i32, i32)> {
        let mut low_points = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let current_height = self.grid[row as usize][col as usize];
                let mut found_smaller_or_equal = false;
                for k in 0..self.drow.len() {
                    let r = row + self.drow[k];
                    let c = col + self.dcol[k];
                    if r < 0 || c < 0 || r >= self.rows || c >= self.cols {
                        continue;
                    }
                    if self.grid[r as usize][c as usize] <= current_height {
                        found_smaller_or_equal = true;
                        break;
                    }
                }
                if !found_smaller_or_equal {
                    // low_point
                    low_points.push((row, col));
                }
            }
        }
        low_points
    }

    fn basin_size(&self, low_point: (i32, i32)) -> usize {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        queue.push_back(low_point);
        visited.insert((low_point.0, low_point.1));

        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();
            // println!("current: {:?}", (row, col));
            for k in 0..self.drow.len() {
                let r = row + self.drow[k];
                let c = col + self.dcol[k];
                if visited.contains(&(r, c))
                    || r < 0
                    || c < 0
                    || r >= self.rows
                    || c >= self.cols
                    || self.grid[r as usize][c as usize] == 9
                {
                    continue;
                }
                let height_difference =
                    self.grid[r as usize][c as usize] - self.grid[row as usize][col as usize];

                if height_difference >= 0 {
                    queue.push_back((r, c));
                    visited.insert((r, c));
                }
            }
        }

        visited.len()
    }
}
