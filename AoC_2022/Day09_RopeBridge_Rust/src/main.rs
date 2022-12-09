use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Copy, Clone)]
enum RopeMove {
    Up(i128),
    Down(i128),
    Left(i128),
    Right(i128),
}

impl From<&str> for RopeMove {
    fn from(line: &str) -> Self {
        let splits: Vec<&str> = line.split(" ").collect();
        if splits.len() != 2 {
            panic!("line in unexpected format!");
        }
        let amount = i128::from_str_radix(splits[1], 10).unwrap();
        match splits[0] {
            "U" => Self::Up(amount),
            "D" => Self::Down(amount),
            "L" => Self::Left(amount),
            "R" => Self::Right(amount),
            _ => panic!("unexpected direction"),
        }
    }
}

struct TaskData {
    rope_moves: Vec<RopeMove>,
    rope_knots: Vec<(i128, i128)>,
    set_tail_pos: HashSet<(i128, i128)>,
}

impl TaskData {
    fn print_debug(&self) {
        println!("{:?}", self.rope_knots);
        for y in -7..=0 {
            for x in 0..=7 {
                let mut found = false;
                for (i, pos) in self.rope_knots.iter().enumerate() {
                    if pos == &(x, y) {
                        if i == 0 {
                            print!("H");
                        } else {
                            print!("{}", i);
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!(".");
                }
            }
            println!("");
        }
        println!("")
    }
    fn count_tail_pos(&self) -> i128 {
        self.set_tail_pos.len() as i128
    }

    fn setup_knots(&mut self, amount: i128) {
        for _ in 0..amount {
            self.rope_knots.push((0, 0));
        }
    }

    fn update_knot(current: (i128, i128), target: (i128, i128)) -> (i128, i128) {
        let (hx, hy) = target;
        let (tx, ty) = current;
        let (dx, dy) = (hx - tx, hy - ty);

        match (dx, dy) {
            (x, y) if x > 1 && y > 1 => (tx + 1, ty + 1),
            (x, y) if x < -1 && y > 1 => (tx - 1, ty + 1),
            (x, y) if x > 1 && y < -1 => (tx + 1, ty - 1),
            (x, y) if x < -1 && y < -1 => (tx - 1, ty - 1),
            (x, y) if x > 1 && y.abs() > 0 => (tx + 1, hy), //diagonal movement in x
            (x, y) if x < -1 && y.abs() > 0 => (tx - 1, hy),
            (x, y) if y > 1 && x.abs() > 0 => (hx, ty + 1), //diagonal movement in y
            (x, y) if y < -1 && x.abs() > 0 => (hx, ty - 1),
            (x, 0) if x > 1 => (tx + 1, ty),
            (x, 0) if x < -1 => (tx - 1, ty),
            (0, y) if y > 1 => (tx, ty + 1),
            (0, y) if y < -1 => (tx, ty - 1),
            _ => (tx, ty),
        }
    }

    fn update_tail(&mut self) {
        let last_i = self.rope_knots.len() - 1;
        for i in 1..=last_i {
            let target = self.rope_knots[i - 1];
            let current = self.rope_knots[i];
            self.rope_knots[i] = Self::update_knot(current, target);
            if i == last_i {
                self.set_tail_pos.insert(self.rope_knots[i]);
            }
        }
    }
    // -1 x 1
    // -1
    //  y
    //  1
    fn go(&mut self, amount: i128, dx: i128, dy: i128) {
        for _ in 0..amount {
            //self.print_debug();
            let head_pos = self.rope_knots[0];
            self.rope_knots[0] = (head_pos.0 + dx, head_pos.1 + dy);
            self.update_tail();
        }
        //println!("after go:");
        //self.print_debug();
    }
    fn execute_move(&mut self, rope_move: &RopeMove) {
        use RopeMove::*;
        match rope_move {
            Up(amount) => self.go(*amount, 0, -1),
            Down(amount) => self.go(*amount, 0, 1),
            Left(amount) => self.go(*amount, -1, 0),
            Right(amount) => self.go(*amount, 1, 0),
        }
    }
    fn execute_all(&mut self) {
        let rope_moves = self.rope_moves.clone();
        for rope_move in rope_moves.iter() {
            self.execute_move(&rope_move);
        }
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let rope_moves: Vec<RopeMove> = input.lines().map(|l| RopeMove::from(l)).collect();
    let mut set_tail_pos: HashSet<(i128, i128)> = HashSet::new();
    let rope_knots = Vec::new();
    set_tail_pos.insert((0, 0));
    Ok(TaskData {
        rope_moves,
        rope_knots,
        set_tail_pos,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.setup_knots(2);
    data.execute_all();
    Ok(data.count_tail_pos())
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.setup_knots(10);
    data.execute_all();
    Ok(data.count_tail_pos())
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
        assert_eq!(answer, 13);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 6190);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 1);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 2516);
        Ok(())
    }
}
