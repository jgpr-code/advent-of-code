use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::io::{self, Read};
use std::mem;

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    part_one(&buffer);
    part_two(&buffer);
    Ok(())
}

fn parse_input(buffer: &str) -> Vec<LineSegment> {
    buffer.lines().map(|line| LineSegment::new(line)).collect()
}

fn part_one(buffer: &str) {
    let line_segments = parse_input(buffer);
    let mut line_counts: HashMap<(i32, i32), i32> = HashMap::new();
    for line_segment in line_segments.iter().filter(|s| s.is_axis_aligned()) {
        if line_segment.is_vertical() {
            let (x, (y_begin, y_end)) = line_segment.vertical_range().unwrap();
            for y in y_begin..y_end + 1 {
                let entry = line_counts.entry((x, y)).or_insert(0);
                *entry += 1;
            }
        } else {
            let (y, (x_begin, x_end)) = line_segment.horizontal_range().unwrap();
            for x in x_begin..x_end + 1 {
                let entry = line_counts.entry((x, y)).or_insert(0);
                *entry += 1;
            }
        }
    }
    let intersections = line_counts.iter().filter(|(_, val)| **val > 1).count();

    println!("Part 1: {}", intersections);
}

fn part_two(buffer: &str) {}

// Line segment
struct LineSegment {
    start_point: (i32, i32),
    end_point: (i32, i32),
}

impl LineSegment {
    fn new(line: &str) -> LineSegment {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")
                    .expect("failed to parse regex");
        }
        let captures = RE.captures(line).expect("line didn't match");
        let x1 = LineSegment::match_to_i32(captures.name("x1"));
        let y1 = LineSegment::match_to_i32(captures.name("y1"));
        let x2 = LineSegment::match_to_i32(captures.name("x2"));
        let y2 = LineSegment::match_to_i32(captures.name("y2"));
        LineSegment {
            start_point: (x1, y1),
            end_point: (x2, y2),
        }
    }
    fn is_vertical(&self) -> bool {
        self.start_point.0 == self.end_point.0
    }
    fn vertical_range(&self) -> Option<(i32, (i32, i32))> {
        if !self.is_vertical() {
            return None;
        }
        let x = self.start_point.0;
        let mut y_begin = self.start_point.1;
        let mut y_end = self.end_point.1;
        if y_begin > y_end {
            mem::swap(&mut y_begin, &mut y_end);
        };
        Some((x, (y_begin, y_end)))
    }
    fn is_horizontal(&self) -> bool {
        self.start_point.1 == self.end_point.1
    }
    fn horizontal_range(&self) -> Option<(i32, (i32, i32))> {
        if !self.is_horizontal() {
            return None;
        }
        let y = self.start_point.1;
        let mut x_begin = self.start_point.0;
        let mut x_end = self.end_point.0;
        if x_begin > x_end {
            mem::swap(&mut x_begin, &mut x_end);
        };
        Some((y, (x_begin, x_end)))
    }
    fn is_axis_aligned(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    fn match_to_i32(regex_match: Option<regex::Match>) -> i32 {
        match regex_match {
            Some(x) => x.as_str().parse().unwrap_or_default(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO common setup which loads the Strings from the files (test.txt, input.txt)
    #[test]
    fn part_one_test() {}
    #[test]
    fn part_one_input() {}
    #[test]
    fn part_two_test() {}
    #[test]
    fn part_two_input() {}
}
