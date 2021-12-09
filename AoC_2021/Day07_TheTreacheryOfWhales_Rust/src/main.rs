use std::cmp;
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

fn part_two(buffer: &str) {
    let mut crab_positions: Vec<i32> = buffer.split(",").map(|x| x.parse().unwrap()).collect();
    crab_positions.sort_unstable();
    let min = crab_positions.first().unwrap();
    let max = crab_positions.last().unwrap();
    let mut cost_min = i32::MAX;
    for i in *min..*max + 1 {
        let cost = crab_positions
            .iter()
            .fold(0, |acc, x| acc + gauss_distance(i, *x));
        cost_min = cmp::min(cost_min, cost);
    }
    println!("Part 2: {}", cost_min);
}

fn gauss_distance(a: i32, b: i32) -> i32 {
    (a - b).abs() * ((a - b).abs() + 1) / 2
}

// sum 1 to n = (n * (n+1)) / 2
// (a, b, c)
// determine x such that min |a-x| + |b-x| + |c-x|
// d(a, x) = sum k=1 to |a-x|: k => gauss |a-x| * (|a-x| + 1) / 2 => (x-a)^2 + |x-a| => 2(x-a) + sign(x-a)
//

// how can this be solved mathematically:
// min_x sum i=1 to n: d( |crab_i - x| )
// d(x) = x * (x+1) / 2
// reformulate to:
// min_z sum i=1 to n: d( z )
// s.t. z_i >= +(crab_i - x)
//      z_i >= -(crab_i - x)
// constrained quadratic optimization problem
