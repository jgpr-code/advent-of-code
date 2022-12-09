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
    head_pos: (i128, i128),
    tail_pos: (i128, i128),
    set_tail_pos: HashSet<(i128, i128)>,
}

impl TaskData {
    fn print_debug(&self) {
        println!("({}, {})", self.head_pos.0, self.head_pos.1);
        println!("({}, {})", self.tail_pos.0, self.tail_pos.1);
        for y in -7..=0 {
            for x in 0..=7 {
                if self.head_pos == (x, y) {
                    print!("H");
                } else if self.tail_pos == (x, y) {
                    print!("T");
                } else {
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

    fn come_closer(pos: i128, target_pos: i128) -> i128 {
        if pos < target_pos + 1 {
            pos + 1
        } else if target_pos - 1 < pos {
            pos - 1
        } else {
            pos
        }
    }

    fn update_tail(&mut self) {
        let (hx, hy) = self.head_pos;
        let (tx, ty) = self.tail_pos;
        let (dx, dy) = (hx - tx, hy - ty);

        let new_tail = match (dx, dy) {
            (x, y) if x > 1 && y.abs() > 0 => (tx + 1, hy), //diagonal movement in x
            (x, y) if x < -1 && y.abs() > 0 => (tx - 1, hy),
            (x, y) if y > 1 && x.abs() > 0 => (hx, ty + 1), //diagonal movement in y
            (x, y) if y < -1 && x.abs() > 0 => (hx, ty - 1),
            (x, 0) if x > 1 => (tx + 1, ty),
            (x, 0) if x < -1 => (tx - 1, ty),
            (0, y) if y > 1 => (tx, ty + 1),
            (0, y) if y < -1 => (tx, ty - 1),
            _ => (tx, ty),
        };
        // // allow distance of 1
        // if dx.abs() <= 1 && dy.abs() <= 1 {
        //     return;
        // }
        // // adapt x
        // //   T H -> hx - tx > 1 => tx+1
        // // H T -> hx - tx < -1 => tx-1
        // if dx.abs() > 2 || dy.abs() > 2 {
        //     panic!("assumptions don't hold")
        // }
        // let mut new_tx = tx;
        // let mut new_ty = ty;
        // if dx > 1 {
        //     new_tx += 1;
        // } else if dx < -1 {
        //     new_tx -= 1;
        // }
        // // T

        // // H  -> hy - ty > 1 => ty + 1
        // if dy > 1 {
        //     new_ty += 1;
        // } else if dy < -1 {
        //     new_ty -= 1;
        // }
        self.tail_pos = new_tail;
        self.set_tail_pos.insert(self.tail_pos);
    }
    // -1 x 1
    // -1
    //  y
    //  1
    fn go(&mut self, amount: i128, dx: i128, dy: i128) {
        for _ in 0..amount {
            //self.print_debug();
            self.head_pos = (self.head_pos.0 + dx, self.head_pos.1 + dy);
            self.update_tail();
        }
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
    let head_pos = (0, 0);
    let tail_pos = head_pos;
    let mut set_tail_pos: HashSet<(i128, i128)> = HashSet::new();
    set_tail_pos.insert((0, 0));
    Ok(TaskData {
        rope_moves,
        head_pos,
        tail_pos,
        set_tail_pos,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.execute_all();
    Ok(data.count_tail_pos())
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
