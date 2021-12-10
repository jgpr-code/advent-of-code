use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    part_one(&buffer);
    part_two(&buffer);
    Ok(())
}

fn part_one(buffer: &str) {
    let syntax_error_score: i64 = buffer.lines().map(|l| score_line(l)).sum();
    println!("Part 1: {}", syntax_error_score);
}

fn score_line(line: &str) -> i64 {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            match stack.last() {
                Some(open) if is_corresponding_closing(*open, c) => stack.pop(),
                _ => return score_illegal(c),
            };
        }
    }
    0
}

fn is_opening(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn is_corresponding_closing(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => true,
        _ => false,
    }
}

fn score_illegal(illegal: char) -> i64 {
    match illegal {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn part_two(buffer: &str) {
    let incomplete_lines: Vec<&str> = buffer.lines().filter(|l| score_line(l) == 0).collect();
    let mut line_scores: Vec<i64> = incomplete_lines
        .iter()
        .map(|l| score_completion(l))
        .collect();
    line_scores.sort_unstable();
    let middle: usize = line_scores.len() / 2;
    println!("Part 2: {}", line_scores[middle]);
}

fn score_completion(line: &str) -> i64 {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            match stack.last() {
                Some(open) if is_corresponding_closing(*open, c) => stack.pop(),
                _ => panic!("please no illegal lines anymore"),
            };
        }
    }
    score_remaining(&mut stack)
}

fn score_remaining(stack: &mut Vec<char>) -> i64 {
    let mut score = 0;
    while let Some(c) = stack.pop() {
        score *= 5;
        score += bracket_score(c);
    }
    score
}

fn bracket_score(bracket: char) -> i64 {
    match bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
