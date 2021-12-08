use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    part_one(&buffer);
    part_two(&buffer);
    Ok(())
}

fn parse_input(buffer: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut output = Vec::new();
    for line in buffer.lines() {
        let mut pipe_split = line.split("|");
        let digits: Vec<&str> = pipe_split
            .nth(0)
            .unwrap()
            .split_whitespace()
            .filter(|s| s.len() > 0)
            .collect();
        let output_digits: Vec<&str> = pipe_split
            .nth(0)
            .unwrap()
            .split_whitespace()
            .filter(|s| s.len() > 0)
            .collect();
        output.push((digits, output_digits));
    }
    output
}

fn part_one(buffer: &str) {
    let input = parse_input(buffer);
    let sum = input.iter().fold(0, |acc, (_, out)| {
        acc + out.iter().filter(|x| is_simple_digit(x)).count()
    });
    println!("Part 1: {}", sum);
}

fn is_simple_digit(digit: &str) -> bool {
    let len = digit.len();
    // 1 4 7 8 are simple
    len == 2 || len == 4 || len == 3 || len == 7
}

fn part_two(buffer: &str) {
    let input = parse_input(buffer);
    let answer = input.iter().fold(0, |acc, (d, od)| acc + decode(d, od));
    println!("Part 2: {}", answer);
}

// seven segment odering
//   0000
//  1    2
//  1    2
//   3333
//  4    5
//  4    5
//   6666
fn decode(digits: &Vec<&str>, output_digits: &Vec<&str>) -> i32 {
    // brute force
    let decodings = possible_decodings();
    for decoding in decodings.iter() {
        if let Some(mapping) = digit_mapping(digits, decoding) {
            //println!("{:?}", mapping);

            let mut thousands_chars: Vec<char> = output_digits[0].chars().collect();
            thousands_chars.sort();
            //println!("{:?}", thousands_chars);
            let thousands = mapping[&thousands_chars];

            let mut hundreds_chars: Vec<char> = output_digits[1].chars().collect();
            hundreds_chars.sort();
            //println!("{:?}", hundreds_chars);
            let hundreds = mapping[&hundreds_chars];

            let mut tens_chars: Vec<char> = output_digits[2].chars().collect();
            tens_chars.sort();
            //println!("{:?}", tens_chars);
            let tens = mapping[&tens_chars];

            let mut ones_chars: Vec<char> = output_digits[3].chars().collect();
            ones_chars.sort();
            //println!("{:?}", ones_chars);
            let ones = mapping[&ones_chars];

            return 1000 * thousands + 100 * hundreds + 10 * tens + ones;
        }
    }
    0
}

fn possible_decodings() -> Vec<HashMap<char, i32>> {
    let perms: Vec<Vec<i32>> = (0..7).permutations(7).collect();
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut mappings = Vec::new();
    for perm in perms.iter() {
        let mut mapping = HashMap::new();
        for (c, i) in chars.iter().zip(perm) {
            mapping.insert(*c, *i);
        }
        mappings.push(mapping);
    }
    mappings
}

fn digit_mapping<'a>(
    digits: &Vec<&'a str>,
    decoding: &HashMap<char, i32>,
) -> Option<HashMap<Vec<char>, i32>> {
    let mut digit_mapping = HashMap::new();
    for digit in digits.iter() {
        let segments = digit_to_segments(digit, decoding);
        if let Some(int_digit) = segments_to_digit(&segments) {
            let mut sorted_chars: Vec<char> = digit.chars().collect();
            sorted_chars.sort();
            digit_mapping.insert(sorted_chars, int_digit);
        }
    }

    if is_valid_mapping(&digit_mapping) {
        Some(digit_mapping)
    } else {
        None
    }
}

fn is_valid_mapping(digit_mapping: &HashMap<Vec<char>, i32>) -> bool {
    let values_set: HashSet<&i32> = digit_mapping.values().collect();
    for i in 0..10 {
        if !values_set.contains(&i) {
            return false;
        }
    }
    if values_set.iter().count() != 10 {
        return false;
    }
    true
}

fn digit_to_segments(digit: &str, decoding: &HashMap<char, i32>) -> Vec<i32> {
    let mut segment_vec: Vec<i32> = digit.chars().map(|c| decoding[&c]).collect();
    segment_vec.sort();
    segment_vec
}

// digit to len,[segment]:
// 0 -> 6,[0,1,2,4,5,6]
// 1 -> 2,[2,5]
// 2 -> 5,[0,2,3,4,6]
// 3 -> 5,[0,2,3,5,6]
// 4 -> 4,[1,2,3,5]
// 5 -> 5,[0,1,3,5,6]
// 6 -> 6,[0,1,3,4,5,6]
// 7 -> 3,[0,2,5]
// 8 -> 7,[0,1,2,3,4,5,6]
// 9 -> 6,[0,1,2,3,5,6]
fn segments_to_digit(segments: &Vec<i32>) -> Option<i32> {
    match segments[..] {
        [0, 1, 2, 4, 5, 6] => Some(0),
        [2, 5] => Some(1),
        [0, 2, 3, 4, 6] => Some(2),
        [0, 2, 3, 5, 6] => Some(3),
        [1, 2, 3, 5] => Some(4),
        [0, 1, 3, 5, 6] => Some(5),
        [0, 1, 3, 4, 5, 6] => Some(6),
        [0, 2, 5] => Some(7),
        [0, 1, 2, 3, 4, 5, 6] => Some(8),
        [0, 1, 2, 3, 5, 6] => Some(9),
        _ => None,
    }
}
