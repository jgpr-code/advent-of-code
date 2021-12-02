use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    let commands: Vec<Command> = buffer.lines().map(|x| Command::new(x)).collect();

    part_one(&commands);
    part_two(&commands);
    Ok(())
}

fn part_one(commands: &Vec<Command>) {
    let mut location = Location {
        horizontal: 0,
        depth: 0,
    };
    location = commands.iter().fold(location, |l, c| l.execute_command(c));
    println!("Part 1: {}", location.multiply());
}

fn part_two(commands: &Vec<Command>) {
    let mut submarine_state = SubmarineState {
        location: Location {
            horizontal: 0,
            depth: 0,
        },
        aim: 0,
    };
    submarine_state = commands
        .iter()
        .fold(submarine_state, |s, c| s.execute_command(c));
    println!("Part 2: {}", submarine_state.location.multiply());
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    pub fn new(line: &str) -> Command {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let amount = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => Command::Forward(0),
        }
    }
}

struct Location {
    horizontal: i32,
    depth: i32,
}

impl Location {
    pub fn multiply(&self) -> i32 {
        self.horizontal * self.depth
    }
    pub fn execute_command(mut self, command: &Command) -> Location {
        match command {
            Command::Forward(amount) => self.horizontal += amount,
            Command::Down(amount) => self.depth += amount,
            Command::Up(amount) => self.depth -= amount,
        };
        self
    }
}

struct SubmarineState {
    location: Location,
    aim: i32,
}

impl SubmarineState {
    pub fn execute_command(mut self, command: &Command) -> SubmarineState {
        match command {
            Command::Forward(amount) => {
                self.location.horizontal += amount;
                self.location.depth += amount * self.aim;
            }
            Command::Down(amount) => self.aim += amount,
            Command::Up(amount) => self.aim -= amount,
        }
        self
    }
}
