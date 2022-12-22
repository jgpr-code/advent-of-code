use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

#[derive(Debug, Clone)]
enum Command {
    Forward(i128),
    Turn(char),
}

// #[derive(Debug)]
// enum Facing {
//     Right,  // 0
//     Down,   // 1
//     Left,   // 2
//     Up,     // 3
// }

#[derive(Debug)]
struct TaskData {
    position: ((i128, i128), usize), // (pos, facing)
    range_on_row: Vec<(i128, i128)>, // end exclusive
    rows: usize,
    range_on_col: Vec<(i128, i128)>, // end exclusive
    cols: usize,
    the_map: Vec<Vec<char>>,
    commands: Vec<Command>,
}
impl TaskData {
    fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.position.0 == (row as i128, col as i128) {
                    print!("X");
                } else {
                    print!("{}", self.the_map[row][col]);
                }
            }
            println!("");
        }
    }

    fn turn_right(&mut self) {
        let facing = self.position.1;
        let new_facing = (facing + 1) % 4;
        self.position.1 = new_facing;
    }
    fn turn_left(&mut self) {
        let facing = self.position.1;
        let new_facing = (facing + 3) % 4;
        self.position.1 = new_facing;
    }
    fn forward(&mut self, amount: i128) {
        let drow = vec![0, 1, 0, -1];
        let dcol = vec![1, 0, -1, 0];
        let facing = self.position.1;

        for _step in 0..amount {
            let pos = self.position.0;
            let row_range = self.range_on_row[pos.0 as usize];
            let col_range = self.range_on_col[pos.1 as usize];
            let mut npos = (pos.0 + drow[facing], pos.1 + dcol[facing]);
            if npos.1 < row_range.0 {
                npos.1 = row_range.1 - 1;
            }
            if npos.1 >= row_range.1 {
                npos.1 = row_range.0;
            }
            if npos.0 < col_range.0 {
                npos.0 = col_range.1 - 1;
            }
            if npos.0 >= col_range.1 {
                npos.0 = col_range.0;
            }
            if self.the_map[npos.0 as usize][npos.1 as usize] != '#' {
                self.position = (npos, facing);
                // only for rendering
                self.the_map[pos.0 as usize][pos.1 as usize] = match facing {
                    0 => '>',
                    1 => 'v',
                    2 => '<',
                    3 => '^',
                    _ => panic!("facing must be in [0, 3]"),
                };
            } else {
                break;
            }
        }
    }
    fn execute_commands(&mut self) {
        let commands = self.commands.clone();
        for command in commands {
            match command {
                Command::Forward(amount) => self.forward(amount),
                Command::Turn(dir) => match dir {
                    'R' => self.turn_right(),
                    'L' => self.turn_left(),
                    _ => panic!("Only R or L is allowed for turns"),
                },
            }
        }
    }
    fn get_password(&self) -> i128 {
        let row = self.position.0 .0 + 1;
        let col = self.position.0 .1 + 1;
        let facing = self.position.1;
        1000 * row + 4 * col + facing as i128
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let mut split = input.split("\r\n\r\n");
    let map_input = split.next().unwrap();
    let commands_input = split.next().unwrap();
    let commands = parse_commands(commands_input);

    let mut range_on_row: Vec<(i128, i128)> = Vec::new();
    let mut range_on_col: Vec<(i128, i128)> = Vec::new();
    let mut the_map: Vec<Vec<char>> = map_input.lines().map(|l| l.chars().collect()).collect();
    let length_longest_line = the_map.iter().map(|v| v.len()).max().unwrap();
    for line in the_map.iter_mut() {
        let current_length = line.len();
        let missing = length_longest_line - current_length;
        if missing > 0 {
            line.append(&mut vec![' '; missing]);
        }
    }
    let rows = the_map.len();
    let cols = the_map[0].len();
    // scan rows
    for row in 0..rows {
        let mut previous = ' ';
        let mut beginning = 0;
        let mut end = cols;
        for col in 0..cols {
            let current = the_map[row][col];
            if previous == ' ' && current != ' ' {
                beginning = col;
            }
            if previous != ' ' && current == ' ' {
                end = col;
            }
            previous = current;
        }
        range_on_row.push((beginning as i128, end as i128));
    }
    // scan cols
    for col in 0..cols {
        let mut previous = ' ';
        let mut beginning = 0;
        let mut end = rows;
        for row in 0..rows {
            let current = the_map[row][col];
            if previous == ' ' && current != ' ' {
                beginning = row;
            }
            if previous != ' ' && current == ' ' {
                end = row;
            }
            previous = current;
        }
        range_on_col.push((beginning as i128, end as i128));
    }
    Ok(TaskData {
        position: ((0, range_on_row[0].0), 0),
        range_on_row,
        rows,
        range_on_col,
        cols,
        the_map,
        commands,
    })
}
fn parse_commands(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+|[LR])").unwrap();
    }
    for cap in RE.captures_iter(input) {
        let part = &cap[1];
        if let Ok(amount) = i128::from_str_radix(part, 10) {
            commands.push(Command::Forward(amount));
        } else {
            if part.len() != 1 {
                panic!("something is wrong");
            }
            commands.push(Command::Turn(part.chars().nth(0).unwrap()));
        }
    }
    commands
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    println!("{:?}", data);
    data.execute_commands();
    data.print();
    let password = data.get_password();
    Ok(password)
}

fn part_two(input: &str) -> Result<i128> {
    let _ = parse_input(input)?;
    Ok(-1)
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
