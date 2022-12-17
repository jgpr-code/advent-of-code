use anyhow::Result;
use std::cmp;
use std::collections::HashSet;
use std::io::{self, Read};

// 7 wide
// left rock edge 2 away from left wall
// bottom edge is three above highest rock or floor
// jet push then fall
// movement only when no wall rock or floor

// Grid
// 0 1 2 3 4 5 6 7 8
// |               |
// |               |
// |               |
// |               |
// |               |
// |               |
// | - - - - - - - | 0

// highest rock is 0 in the beginning (floor)

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i128,
    y: i128,
}

impl Pos {
    fn jet(&self, jet: Jet) -> Pos {
        match jet {
            Jet::Left => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Jet::Right => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
    fn fall(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(c: char) -> Self {
        match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("wrong jet char"),
        }
    }
}

struct TaskData {
    rocks_index: usize,
    rocks: Vec<HashSet<Pos>>,
    highest: i128,
    jets_index: usize,
    jets: Vec<Jet>,
    rested: HashSet<Pos>,
}

impl TaskData {
    fn debug(&self, current: Option<&HashSet<Pos>>) {
        for y in (0..=20).rev() {
            for x in 0..=8 {
                if y == 0 {
                    print!("-");
                } else if x == 0 || x == 8 {
                    print!("|");
                } else {
                    if self.rested.contains(&Pos { x, y }) {
                        print!("#");
                    } else {
                        if let Some(falling) = current {
                            if falling.contains(&Pos { x, y }) {
                                print!("@");
                            } else {
                                print!(".");
                            }
                        } else {
                            print!(".");
                        }
                    }
                }
            }
            println!("");
        }
    }
    fn intersect(&self, rock: &HashSet<Pos>) -> bool {
        // intersect with ground
        let intersect = rock.intersection(&self.rested).count();
        if intersect > 0 {
            return true;
        }
        let intersect = rock
            .iter()
            .filter(|p| p.x == 0 || p.x == 8 || p.y == 0)
            .count();
        intersect > 0
    }
    fn jet(jet: Jet, rock: &HashSet<Pos>) -> HashSet<Pos> {
        rock.iter().map(|p| p.jet(jet)).collect()
    }
    fn fall(rock: &HashSet<Pos>) -> HashSet<Pos> {
        rock.iter().map(|p| p.fall()).collect()
    }
    fn next_rock(&mut self) -> HashSet<Pos> {
        let initial = &self.rocks[self.rocks_index];
        self.rocks_index = (self.rocks_index + 1) % self.rocks.len();
        let x_off = 2 + 1;
        let y_off = self.highest + 3 + 1;
        initial
            .iter()
            .map(|p| Pos {
                x: p.x + x_off,
                y: p.y + y_off,
            })
            .collect()
    }
    fn handle_rock(&mut self, rock: &HashSet<Pos>) {
        let mut current = rock.clone();
        // self.debug(Some(&current));
        loop {
            let jet = self.jets[self.jets_index];
            self.jets_index = (self.jets_index + 1) % self.jets.len();
            let jetted = Self::jet(jet, &current);
            if !self.intersect(&jetted) {
                current = jetted.clone();
            }
            // println!("after jet");
            // self.debug(Some(&current));
            let falled = Self::fall(&current);
            // println!("after fall (before correct)");
            // self.debug(Some(&falled));
            if self.intersect(&falled) {
                self.rested = self.rested.union(&current).map(|&p| p).collect();
                self.highest = cmp::max(self.highest, current.iter().map(|p| p.y).max().unwrap());
                return;
            } else {
                current = falled.clone();
            }
        }
    }
    fn run_n_rocks(&mut self, n: i128) {
        for _ in 1..=n {
            let rock = self.next_rock();
            self.handle_rock(&rock);
        }
    }
}

// Rock shapes repeat from top to bottom

fn construct_rocks() -> Vec<HashSet<Pos>> {
    // 0 0 is always bottom left
    // points are inserted from top to bottom and left to right (not that it matters)

    // ####
    let mut line = HashSet::new();
    line.insert(Pos { x: 0, y: 0 });
    line.insert(Pos { x: 1, y: 0 });
    line.insert(Pos { x: 2, y: 0 });
    line.insert(Pos { x: 3, y: 0 });

    // .#.
    // ###
    // .#.
    let mut cross = HashSet::new();
    cross.insert(Pos { x: 1, y: 2 });
    cross.insert(Pos { x: 0, y: 1 });
    cross.insert(Pos { x: 1, y: 1 });
    cross.insert(Pos { x: 2, y: 1 });
    cross.insert(Pos { x: 1, y: 0 });

    // ..#
    // ..#
    // ###
    let mut l_shape = HashSet::new();
    l_shape.insert(Pos { x: 2, y: 2 });
    l_shape.insert(Pos { x: 2, y: 1 });
    l_shape.insert(Pos { x: 0, y: 0 });
    l_shape.insert(Pos { x: 1, y: 0 });
    l_shape.insert(Pos { x: 2, y: 0 });

    // #
    // #
    // #
    // #
    let mut down_line = HashSet::new();
    down_line.insert(Pos { x: 0, y: 3 });
    down_line.insert(Pos { x: 0, y: 2 });
    down_line.insert(Pos { x: 0, y: 1 });
    down_line.insert(Pos { x: 0, y: 0 });

    // ##
    // ##
    let mut block = HashSet::new();
    block.insert(Pos { x: 0, y: 1 });
    block.insert(Pos { x: 1, y: 1 });
    block.insert(Pos { x: 0, y: 0 });
    block.insert(Pos { x: 1, y: 0 });

    vec![line, cross, l_shape, down_line, block]
}

fn parse_input(input: &str) -> Result<TaskData> {
    let jets: Vec<Vec<Jet>> = input
        .lines()
        .map(|l| l.chars().map(|c| Jet::from(c)).collect())
        .collect();

    Ok(TaskData {
        rocks_index: 0,
        rocks: construct_rocks(),
        highest: 0,
        jets_index: 0,
        jets: jets[0].clone(),
        rested: HashSet::new(),
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.run_n_rocks(2022);
    //data.debug(None);
    Ok(data.highest)
}

fn part_two(input: &str) -> Result<i128> {
    let _ = parse_input(input)?;
    Ok(-1)
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
