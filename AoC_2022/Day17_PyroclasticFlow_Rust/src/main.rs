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
    rocks: Vec<HashSet<Pos>>,
    jets_index: usize,
    jets: Vec<char>,
    // ground is always 0 in the coordinates however the real height is obtained by adding ground_offset
    highest: i128,
    ground_offset: i128,
    rested: HashSet<Pos>,
}

impl RockTetris {
    fn intersect(&self, rock: &HashSet<Pos>) -> bool {
        let intersect = rock.intersection(&self.rested).count();
        if intersect > 0 {
            return true;
        }
        let intersect = rock
            .iter()
            .filter(|p| p.x == 0 || p.x == 8 || p.y == self.ground_offset)
            .count();
        intersect > 0
    }
    fn jet(jet: char, rock: &HashSet<Pos>) -> HashSet<Pos> {
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
        loop {
            let jet = self.jets[self.jets_index];
            self.jets_index = (self.jets_index + 1) % self.jets.len();
            let jetted = Self::jet(jet, &current);
            if !self.intersect(&jetted) {
                current = jetted.clone();
            }

            let falled = Self::fall(&current);

            if self.intersect(&falled) {
                self.rested = self.rested.union(&current).map(|&p| p).collect();
                self.highest = cmp::max(self.highest, current.iter().map(|p| p.y).max().unwrap());
                self.cleanup(&current);
                return;
            } else {
                current = falled.clone();
            }
        }
    }
    fn cleanup(&mut self, current: &HashSet<Pos>) {
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
                self.discard_rested_at_and_below(y);
                return;
            }
        }
    }
    fn discard_rested_at_and_below(&mut self, y: i128) {
        self.ground_offset = y;
        self.rested = self.rested.iter().map(|&p| p).filter(|p| p.y > y).collect();
    }

    fn run_n_rocks(&mut self, n: i128) {
        for i in 1..=n {
            if i % 1_000_000_000 == 0 {
                println!("rock {}", i);
            }
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

fn parse_input(input: &str) -> Result<RockTetris> {
    let jets: Vec<char> = input.lines().nth(0).unwrap().chars().collect();

    Ok(RockTetris {
        rocks_index: 0,
        rocks: construct_rocks(),
        jets_index: 0,
        jets,

        highest: 0,
        ground_offset: 0,
        rested: HashSet::new(),
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.run_n_rocks(2022);
    Ok(data.highest)
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.run_n_rocks(1_000_000_000_000);
    Ok(data.highest)
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

// fn debug(&self, current: Option<&HashSet<Pos>>) {
//     for y in (0..=20).rev() {
//         for x in 0..=8 {
//             if y == 0 {
//                 print!("-");
//             } else if x == 0 || x == 8 {
//                 print!("|");
//             } else {
//                 if self.rested.contains(&Pos { x, y }) {
//                     print!("#");
//                 } else {
//                     if let Some(falling) = current {
//                         if falling.contains(&Pos { x, y }) {
//                             print!("@");
//                         } else {
//                             print!(".");
//                         }
//                     } else {
//                         print!(".");
//                     }
//                 }
//             }
//         }
//         println!("");
//     }
// }

// fn cleanup2(&mut self) {
//     // the idea here is to find the largest y for each x and then use the min of those y's and discard below
//     let mut largest_y: Vec<i128> = vec![0; 7]; // 1 indexed
//     for pos in self.rested.iter() {
//         largest_y[(pos.x - 1) as usize] = cmp::max(largest_y[(pos.x - 1) as usize], pos.y);
//     }
//     let safe_to_discard = *largest_y.iter().min().unwrap();
//     self.discard_rested_at_and_below(safe_to_discard);
// }
// fn cleanup_bfs(&mut self) {
//     let lowest_reachable = self.find_lowest_reachable_bfs();
//     self.discard_rested_at_and_below(lowest_reachable - 1);
// }
// fn find_lowest_reachable_bfs(&self) -> i128 {
//     // just start one above the highest
//     let start = Pos {
//         x: 0,
//         y: self.highest + 1,
//     };
//     let mut queue = VecDeque::new();
//     let mut visited = HashSet::new();
//     queue.push_back(start);
//     visited.insert(start);
//     while let Some(pos) = queue.pop_front() {
//         // try down left right
//         let dx = vec![-1, 0, 1];
//         let dy = vec![0, -1, 0];
//         for i in 0..3 {
//             let nx = pos.x + dx[i];
//             let ny = pos.y + dy[i];
//             if nx == 0 || nx == 8 || ny == 0 {
//                 continue;
//             }
//             let npos = Pos { x: nx, y: ny };
//             if visited.contains(&npos) || self.rested.contains(&npos) {
//                 continue;
//             }
//             queue.push_back(npos);
//             visited.insert(npos);
//         }
//     }
//     visited.iter().map(|p| p.y).min().unwrap()
// }
