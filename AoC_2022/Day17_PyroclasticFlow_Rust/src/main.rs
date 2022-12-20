use anyhow::Result;
use std::cmp;
use std::collections::{BTreeSet, HashMap};
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i128,
    y: i128,
}

impl Pos {
    fn jet(&self, jet: char) -> Pos {
        match jet {
            '<' => Pos {
                x: self.x - 1,
                y: self.y,
            },
            '>' => Pos {
                x: self.x + 1,
                y: self.y,
            },
            _ => panic!("not a jet"),
        }
    }
    fn fall(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y - 1,
        }
    }
}

struct RockTetris {
    rocks_index: usize,
    rocks: Vec<BTreeSet<Pos>>,
    jets_index: usize,
    jets: Vec<char>,
    // ground is always 0 in the coordinates however the real height is obtained by adding ground_offset
    total_rocks_dropped: i128,
    highest: i128,
    ground_offset: i128,
    rested: BTreeSet<Pos>,
    // store the encountered completions after the discard happened
    // insert key: (rocks_index, jets_index, rested) value: (total_rocks_dropped, ground_offset)
    encountered_completions: HashMap<(usize, usize, BTreeSet<Pos>), (i128, i128)>,
}

impl RockTetris {
    fn run_n_rocks(&mut self, n: i128) {
        while self.total_rocks_dropped < n {
            let rock = self.next_rock();
            self.total_rocks_dropped += 1;
            self.handle_rock(&rock, n);
        }
    }
    fn next_rock(&mut self) -> BTreeSet<Pos> {
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
    fn handle_rock(&mut self, rock: &BTreeSet<Pos>, n: i128) {
        let mut current = rock.clone();

        loop {
            let jet = self.jets[self.jets_index];
            self.jets_index = (self.jets_index + 1) % self.jets.len();
            let mut next = Self::jet(jet, &current);
            if !self.intersect(&next) {
                std::mem::swap(&mut current, &mut next);
            }
            let mut next = Self::fall(&current);
            if self.intersect(&next) {
                self.rested = self.rested.union(&current).map(|&p| p).collect();
                self.highest = cmp::max(self.highest, current.iter().map(|p| p.y).max().unwrap());
                self.check_line_completion(&current, n);
                return;
            } else {
                std::mem::swap(&mut current, &mut next);
            }
        }
    }
    fn get_heighest(&self) -> i128 {
        self.highest + self.ground_offset
    }
    fn intersect(&self, rock: &BTreeSet<Pos>) -> bool {
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
    fn jet(jet: char, rock: &BTreeSet<Pos>) -> BTreeSet<Pos> {
        rock.iter().map(|p| p.jet(jet)).collect()
    }
    fn fall(rock: &BTreeSet<Pos>) -> BTreeSet<Pos> {
        rock.iter().map(|p| p.fall()).collect()
    }

    fn check_line_completion(&mut self, current: &BTreeSet<Pos>, n: i128) {
        // check if any line where the tile landed is complete
        let mut lines_to_check = current.iter().map(|p| p.y).collect::<Vec<_>>();
        lines_to_check.sort_by(|a, b| b.cmp(a));
        for y in lines_to_check {
            let mut amount = 0;
            for x in 1..=7 {
                if self.rested.contains(&Pos { x, y }) {
                    amount += 1;
                }
            }
            if amount == 7 {
                // line's full
                self.handle_line_completion(y, n);
                return;
            }
        }
    }
    fn handle_line_completion(&mut self, y: i128, n: i128) {
        self.highest -= y;
        self.ground_offset += y;
        self.rested = self
            .rested
            .iter()
            .filter(|p| p.y > y)
            .map(|p| Pos { x: p.x, y: p.y - y })
            .collect();
        let completion_key = (self.rocks_index, self.jets_index, self.rested.clone());
        if self.encountered_completions.contains_key(&completion_key) {
            let ground_now = self.ground_offset;
            let rocks_now = self.total_rocks_dropped;
            let (rocks_before, ground_before) = self.encountered_completions[&completion_key];
            let ground_delta = ground_now - ground_before;
            let rocks_delta = rocks_now - rocks_before;
            println!(
                "ground now {} vs ground before {} delta: {}",
                ground_now, ground_before, ground_delta
            );
            println!(
                "rocks now {} vs rocks before {} delta: {}",
                rocks_now, rocks_before, rocks_delta
            );
            // how much can be skipped
            // rocks_now + n * rocks_delta == n_rocks
            let delta = (n - rocks_now) / rocks_delta;
            self.total_rocks_dropped += delta * rocks_delta;
            self.ground_offset += delta * ground_delta;
        }
        self.encountered_completions.insert(
            completion_key,
            (self.total_rocks_dropped, self.ground_offset),
        );
    }
    fn debug(&self, current: Option<&BTreeSet<Pos>>) {
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
}

fn construct_rocks() -> Vec<BTreeSet<Pos>> {
    // 0 0 is always bottom left
    // points are inserted from top to bottom and left to right (not that it matters)

    // ####
    let mut line = BTreeSet::new();
    line.insert(Pos { x: 0, y: 0 });
    line.insert(Pos { x: 1, y: 0 });
    line.insert(Pos { x: 2, y: 0 });
    line.insert(Pos { x: 3, y: 0 });

    // .#.
    // ###
    // .#.
    let mut cross = BTreeSet::new();
    cross.insert(Pos { x: 1, y: 2 });
    cross.insert(Pos { x: 0, y: 1 });
    cross.insert(Pos { x: 1, y: 1 });
    cross.insert(Pos { x: 2, y: 1 });
    cross.insert(Pos { x: 1, y: 0 });

    // ..#
    // ..#
    // ###
    let mut l_shape = BTreeSet::new();
    l_shape.insert(Pos { x: 2, y: 2 });
    l_shape.insert(Pos { x: 2, y: 1 });
    l_shape.insert(Pos { x: 0, y: 0 });
    l_shape.insert(Pos { x: 1, y: 0 });
    l_shape.insert(Pos { x: 2, y: 0 });

    // #
    // #
    // #
    // #
    let mut down_line = BTreeSet::new();
    down_line.insert(Pos { x: 0, y: 3 });
    down_line.insert(Pos { x: 0, y: 2 });
    down_line.insert(Pos { x: 0, y: 1 });
    down_line.insert(Pos { x: 0, y: 0 });

    // ##
    // ##
    let mut block = BTreeSet::new();
    block.insert(Pos { x: 0, y: 1 });
    block.insert(Pos { x: 1, y: 1 });
    block.insert(Pos { x: 0, y: 0 });
    block.insert(Pos { x: 1, y: 0 });

    vec![line, cross, l_shape, down_line, block]
}

fn parse_input(input: &str) -> Result<RockTetris> {
    let jets: Vec<char> = input.lines().nth(0).unwrap().chars().collect();

    Ok(RockTetris {
        rocks_index: 0,
        rocks: construct_rocks(),
        jets_index: 0,
        jets,
        total_rocks_dropped: 0,
        highest: 0,
        ground_offset: 0,
        rested: BTreeSet::new(),
        encountered_completions: HashMap::new(),
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.run_n_rocks(2022);
    data.debug(None);
    Ok(data.get_heighest())
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.run_n_rocks(1_000_000_000_000);
    Ok(data.get_heighest())
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
