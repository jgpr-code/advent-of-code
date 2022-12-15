use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Sensor {
    position: (i128, i128),
    closest_beacon: (i128, i128),
}
impl Sensor {
    fn safe_abs(a: i128, b: i128) -> i128 {
        let ba = a as i128;
        let bb = b as i128;
        if ba > bb {
            (ba - bb).abs()
        } else {
            (bb - ba).abs()
        }
    }
    fn distance(a: (i128, i128), b: (i128, i128)) -> i128 {
        Self::safe_abs(a.0, b.0) + Self::safe_abs(a.1, b.1)
    }
    fn distance_to_closest(&self) -> i128 {
        Self::distance(self.position, self.closest_beacon)
    }
    fn in_closest_beacon_range(&self, position: (i128, i128)) -> bool {
        let distance_to_closest: i128 = self.distance_to_closest();
        let distance_to_position: i128 = Self::distance(self.position, position);
        distance_to_position <= distance_to_closest
    }
    fn in_closest_beacon_range_next_x(&self, position: (i128, i128)) -> (bool, i128) {
        let distance_to_closest: i128 = self.distance_to_closest();
        let dx = self.position.0 - position.0;
        let dy = (self.position.1 - position.1).abs();
        // 9
        // dx neg on the right
        // dx dy
        // max_off = 2(9-7) + 1
        // abzug (9 - y - x)
        // 0 9 -> +1 =

        // 1 8 -> +3 = 2dx + 8-9 + 1
        // 0 8 -> +2
        // -1 8 -> +1 = -2 -1 + 1

        // 2 7 -> +5
        // 1 7 -> +4   max_off = 2(9-7) + 1 = 5, abzug = (9 - 7 - 1) = 1 daher max_off - abzug = 4
        // 0 7 -> +3
        // -1 7 -> +2
        // -2 7 -> +1
        let max_off = 2 * (distance_to_closest - dy) + 1;
        let abzug = distance_to_closest - dy - dx;
        //             #
        //            ###
        //           #####
        //          #######
        // usw
        let next = max_off - abzug;
        let distance_to_position: i128 = Self::distance(self.position, position);
        (distance_to_position <= distance_to_closest, next)
    }
    fn mark_grid(&self, grid: &mut HashMap<(i128, i128), char>) {
        let (pos_x, pos_y) = self.position;
        let delta = self.distance_to_closest();
        for x in pos_x - delta..=pos_x + delta {
            for y in pos_y - delta..=pos_y + delta {
                if !self.in_closest_beacon_range((x, y)) {
                    continue;
                }
                if !grid.contains_key(&(x, y)) {
                    grid.insert((x, y), '#');
                }
            }
        }
        grid.insert(self.position, 'S');
        grid.insert(self.closest_beacon, 'B');
    }
    fn mark_grid_only_relevant(&self, y: i128, grid: &mut HashMap<(i128, i128), char>) {
        let (pos_x, _pos_y) = self.position;
        let delta = self.distance_to_closest();
        for x in pos_x - delta..=pos_x + delta {
            if !self.in_closest_beacon_range((x, y)) {
                continue;
            }
            if !grid.contains_key(&(x, y)) {
                grid.insert((x, y), '#');
            }
        }
        grid.insert(self.position, 'S');
        grid.insert(self.closest_beacon, 'B');
    }
}

impl From<&str> for Sensor {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let pos_x = i128::from_str_radix(&caps[1], 10).unwrap();
        let pos_y = i128::from_str_radix(&caps[2], 10).unwrap();
        let beacon_x = i128::from_str_radix(&caps[3], 10).unwrap();
        let beacon_y = i128::from_str_radix(&caps[4], 10).unwrap();
        Sensor {
            position: (pos_x, pos_y),
            closest_beacon: (beacon_x, beacon_y),
        }
    }
}

struct TaskData {
    sensors: Vec<Sensor>,
    grid: HashMap<(i128, i128), char>,
}
impl TaskData {
    fn check_row_y(&mut self, y_check: i128) -> i128 {
        for sensor in self.sensors.iter() {
            println!("checking {:?}", sensor);
            sensor.mark_grid_only_relevant(y_check, &mut self.grid);
        }
        self.grid
            .iter()
            .filter(|((_x, y), c)| *y == y_check && **c == '#')
            .count() as i128
    }

    fn is_in_range_of_any(&self) -> (i128, i128) {
        let max = 4000000;
        //let max = 20;
        for y in 0..=max {
            let mut x = 0;
            while x <= max {
                let fil: Vec<i128> = self
                    .sensors
                    .iter()
                    .map(|s| s.in_closest_beacon_range_next_x((x, y)))
                    .filter(|(t, _off)| *t == true)
                    .map(|(_, off)| off)
                    .collect();
                if fil.len() == 0 {
                    return (x, y);
                } else {
                    x += fil.iter().max().unwrap();
                }
            }
        }
        panic!("not found");
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let mut sensors = Vec::new();
    let grid = HashMap::new();
    for line in input.lines() {
        sensors.push(Sensor::from(line));
    }
    Ok(TaskData { sensors, grid })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    // let y = 2000000;
    let y = 10;
    Ok(data.check_row_y(y))
}

fn part_two(input: &str) -> Result<i128> {
    let data = parse_input(input)?;
    let (x, y) = data.is_in_range_of_any();
    Ok(x * 4000000 + y)
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
