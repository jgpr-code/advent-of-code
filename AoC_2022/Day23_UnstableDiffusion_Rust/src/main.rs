use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

struct TaskData {
    elves: HashSet<(i128, i128)>,
    current: usize,     // 0 North, 1 South, 2 West, 3 East
    dr: Vec<Vec<i128>>, // dr[0][0, 1, 2] = NW, N, NE // clockwise around middle
    dc: Vec<Vec<i128>>, // dc[0][0, 1, 2] = NW, N, NE usw
    debug: bool,
}

impl TaskData {
    fn propose_moves(
        &self,
    ) -> (
        HashMap<(i128, i128), i128>,
        HashMap<(i128, i128), (i128, i128)>,
    ) {
        let mut moves: HashMap<(i128, i128), (i128, i128)> = HashMap::new();
        let mut destination_count = HashMap::new();
        // check all directions

        let dr = self.dr.clone();
        let dc = self.dc.clone();

        for elf in self.elves.iter() {
            let mut total = 0;
            for i in 0..4 {
                for j in 0..3 {
                    let consider = (elf.0 + dr[i][j], elf.1 + dc[i][j]);
                    if self.elves.contains(&consider) {
                        total += 1;
                        break;
                    }
                }
                if total > 0 {
                    break;
                }
            }
            let can_even_move = total != 0;
            if !can_even_move {
                continue;
            }
            let mut check = self.current;
            loop {
                // if self.debug {
                //     println!("{:?}", elf);
                //     println!("considering {}:", check);
                // }
                let mut elves_in_dir = 0;
                for i in 0..3 {
                    let consider = (elf.0 + dr[check][i], elf.1 + dc[check][i]);
                    if self.elves.contains(&consider) {
                        elves_in_dir += 1;
                    }
                }
                // if self.debug {
                //     println!("amount: {}", elves_in_dir);
                // }
                if elves_in_dir == 0 {
                    let proposed = (elf.0 + dr[check][1], elf.1 + dc[check][1]);
                    moves.insert(*elf, proposed); // destination_count makes sure that it doesn't matter to overwrite
                    let count = destination_count.entry(proposed).or_insert(0);
                    *count += 1;
                    break;
                }
                check = (check + 1) % 4;
                if check == self.current {
                    // all considered
                    break;
                }
            }
        }
        (destination_count, moves)
    }
    fn execute_moves(
        &mut self,
        destination_count: HashMap<(i128, i128), i128>,
        moves: HashMap<(i128, i128), (i128, i128)>,
    ) {
        if self.debug {
            println!("{:?}", self.elves);
            println!("{:?}", destination_count);
            println!("{:?}", moves);
        }
        for (from, to) in moves.into_iter() {
            if destination_count[&to] == 1 {
                self.elves.remove(&from);
                self.elves.insert(to);
            }
        }
        self.current = (self.current + 1) % 4;
    }
    fn simulate_n_rounds(&mut self, n: usize) {
        self.print_current();
        for _ in 0..n {
            let (destination_count, moves) = self.propose_moves();
            self.execute_moves(destination_count, moves);
            self.print_current();
        }
    }
    fn simulate_until(&mut self) -> i128 {
        let mut rounds = 0;
        loop {
            let (destination_count, moves) = self.propose_moves();
            if moves.len() == 0 {
                break;
            }
            self.execute_moves(destination_count, moves);
            rounds += 1;
        }
        rounds + 1
    }
    fn print_current(&self) {
        if !self.debug {
            return;
        }
        let mut rows: Vec<i128> = self.elves.iter().map(|(row, _col)| *row).collect();
        let mut cols: Vec<i128> = self.elves.iter().map(|(_row, col)| *col).collect();
        rows.sort();
        cols.sort();
        for row in *rows.first().unwrap()..=*rows.last().unwrap() {
            for col in *cols.first().unwrap()..=*cols.last().unwrap() {
                if self.elves.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
    fn count_ground(&self) -> i128 {
        let mut rows: Vec<i128> = self.elves.iter().map(|(row, _col)| *row).collect();
        let mut cols: Vec<i128> = self.elves.iter().map(|(_row, col)| *col).collect();
        rows.sort();
        cols.sort();
        let row_extend = rows.last().unwrap() - rows.first().unwrap() + 1;
        let col_extend = cols.last().unwrap() - cols.first().unwrap() + 1;
        row_extend * col_extend - self.elves.len() as i128
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let mut elves = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((row as i128, col as i128));
            }
        }
    }

    let dr = vec![
        vec![-1, -1, -1], // N
        vec![1, 1, 1],    // S
        vec![-1, 0, 1],
        vec![-1, 0, 1],
    ];
    let dc = vec![
        vec![-1, 0, 1],
        vec![-1, 0, 1],
        vec![-1, -1, -1], // W
        vec![1, 1, 1],    // E
    ];

    Ok(TaskData {
        elves,
        current: 0,
        dr,
        dc,
        debug: true,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.debug = false;
    data.simulate_n_rounds(10);
    Ok(data.count_ground())
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.debug = false;
    Ok(data.simulate_until())
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
        assert_eq!(answer, 110);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 4288);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 20);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 940);
        Ok(())
    }
}
