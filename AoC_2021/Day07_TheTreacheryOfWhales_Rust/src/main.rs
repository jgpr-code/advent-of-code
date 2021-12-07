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
    let mut crab_positions: Vec<i32> = buffer.split(",").map(|x| x.parse().unwrap()).collect();
    let align_to = median(&mut crab_positions);
    let cost = crab_positions
        .iter()
        .fold(0, |acc, x| acc + (align_to - x).abs());
    println!("Part 1: {}", cost);
}

fn median(nums: &mut Vec<i32>) -> i32 {
    nums.sort_unstable();
    let idx = nums.len() / 2;
    nums[idx]
}

fn part_two(buffer: &str) {}

// (a, b, c)
// determine x such that min |a-x| + |b-x| + |c-x|
