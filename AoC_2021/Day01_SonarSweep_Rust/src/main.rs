use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    let iter = buffer.lines().map(|s| s.parse::<i32>().unwrap());
    let iterclone = iter.clone();
    let iterclone2 = iter.clone();
    let inreasing = iter.zip(iterclone.skip(1)).filter(|(x, y)| x < y).count();

    let vec: Vec<i32> = iterclone2.collect();
    let mut last_sum = vec[0] + vec[1] + vec[2];
    let mut count_increasing = 0;
    for i in 1..vec.len() - 1 {
        let sum = vec[i - 1] + vec[i] + vec[i + 1];
        if last_sum < sum {
            count_increasing += 1;
        }
        last_sum = sum;
    }

    println!("Part 1: {}", inreasing);
    println!("Part 2: {}", count_increasing);
    Ok(())
}
