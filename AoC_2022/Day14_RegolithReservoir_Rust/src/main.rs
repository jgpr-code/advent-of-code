use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, Read};

struct TaskData {
    grid: HashMap<(i128, i128), char>,
    lowest_rock: i128,
    sand_start: (i128, i128),
}

impl TaskData {
    fn fill_cave(&mut self) {
        let dxs = vec![0, -1, 1];
        let dy = 1;
        loop {
            let mut pos = self.sand_start;
            loop {
                let mut moved = false;
                for dx in dxs.iter() {
                    let new_pos = (pos.0 + dx, pos.1 + dy);
                    if !self.grid.contains_key(&new_pos) {
                        // grid only ever contains blocking, either O or #
                        pos = new_pos;
                        moved = true;
                        if new_pos.1 > self.lowest_rock {
                            return; // cave is filled completely
                        }
                        break;
                    }
                }
                if !moved {
                    self.grid.insert(pos, 'O');
                    break; // inner loop -> new sand start
                }
            }
        }
    }
    fn fill_cave2(&mut self) {
        let dxs = vec![0, -1, 1];
        let dy = 1;
        let bottom_y = self.lowest_rock + 2;
        loop {
            let mut pos = self.sand_start;
            loop {
                let mut moved = false;
                for dx in dxs.iter() {
                    let new_pos = (pos.0 + dx, pos.1 + dy);
                    if !self.grid.contains_key(&new_pos) && new_pos.1 < bottom_y {
                        // grid only ever contains blocking, either O or #
                        pos = new_pos;
                        moved = true;
                        break;
                    }
                }
                if !moved {
                    self.grid.insert(pos, 'O');
                    if pos == (500, 0) {
                        return; // cave filled
                    }
                    break; // inner loop -> new sand start
                }
            }
        }
    }
    fn count_sand(&self) -> i128 {
        self.grid.iter().filter(|(k, v)| **v == 'O').count() as i128
    }
}

// x right
// y down

// sand pouring from 500,0, 1/t only when prev rests, fall down 1 if poss, tile below blocked(rock/sand) -> diagonal (downleft, downright)
fn parse_input(input: &str) -> Result<TaskData> {
    let mut grid: HashMap<(i128, i128), char> = HashMap::new();
    for line in input.lines() {
        let dests = line.split(" -> ");
        let mut dest_points = Vec::new();
        for dest in dests {
            let xy: Vec<&str> = dest.split(",").collect();
            let x = i128::from_str_radix(xy[0], 10).unwrap();
            let y = i128::from_str_radix(xy[1], 10).unwrap();
            dest_points.push((x, y));
        }
        for ab in dest_points.windows(2) {
            let a = ab[0];
            let b = ab[1];
            // println!("{:?} -> {:?}", a, b);
            if a.0 == b.0 {
                let x = a.0;
                let mut from = a.1;
                let mut to = b.1;
                if from > to {
                    let tmp = from;
                    from = to;
                    to = tmp;
                }
                for y in from..=to {
                    grid.insert((x, y), '#');
                }
            } else if a.1 == b.1 {
                let y = a.1;
                let mut from = a.0;
                let mut to = b.0;
                if from > to {
                    let tmp = from;
                    from = to;
                    to = tmp;
                }
                for x in from..=to {
                    grid.insert((x, y), '#');
                }
            } else {
                panic!("parse");
            }
        }
    }
    let lowest_rock = grid
        .iter()
        .map(|((_, y), &c)| if c == '#' { *y } else { 0 })
        .max()
        .unwrap();
    Ok(TaskData {
        grid,
        lowest_rock,
        sand_start: (500, 0),
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.fill_cave();
    Ok(data.count_sand())
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.fill_cave2();
    Ok(data.count_sand())
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
        assert_eq!(answer, 0);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 0);
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
