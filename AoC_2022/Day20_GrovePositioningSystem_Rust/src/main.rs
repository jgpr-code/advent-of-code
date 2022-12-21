use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct TaskData {
    numbers: Vec<i128>,
    decrypt_numbers: Vec<i128>,
    amount_numbers: usize,
    orig_zero: usize,
    orig2curr: HashMap<usize, usize>,
    curr2orig: HashMap<usize, usize>,
}

impl TaskData {
    fn _debug(&self) {
        println!("{:?}", self);
        for i in 0..self.amount_numbers {
            print!("{}, ", self.numbers[self.curr2orig[&i]]);
        }
        println!("");
    }
    fn circle_add(&self, index: usize, number: i128) -> usize {
        let mut ni = index as i128 + number;
        while ni < 0 {
            ni += self.amount_numbers as i128 - 1;
        }
        while ni as usize >= self.amount_numbers {
            ni -= self.amount_numbers as i128 - 1;
        }
        ni as usize
    }
    // Initial arrangement:
    // 1, 2, -3, 3, -2, 0, 4
    //[0  1   2  3   4  5  6] curr2orig

    // 1 moves between 2 and -3:
    // 2, 1, -3, 3, -2, 0, 4
    //[1  0   2  3   4  5  6]
    // 1 moves to [1] -> update old 1..=1 curr2orig[0] = curr2orig[1]

    // 2 moves between -3 and 3:
    // 1, -3, 2, 3, -2, 0, 4
    // 0   2  1  3   4  5  6
    // 2 moves to [2] -> update 1..=2

    // -3 moves between -2 and 0:
    // 1, 2, 3, -2, -3, 0, 4
    // 0  1  3   4   2  5  6

    // 3 moves between 0 and 4:
    // 1, 2, -2, -3, 0, 3, 4

    // -2 moves between 4 and 1:
    // 1, 2, -3, 0, 3, 4, -2

    // 0 does not move:
    // 1, 2, -3, 0, 3, 4, -2

    // 4 moves between -3 and 0:
    // 1, 2, -3, 4, 0, 3, -2

    // 1, -3, 2, 3, -2, 0, 4
    // 1, 2, 3, -2, -3, 0, 4

    // 2, 1
    // 2, 1, 3
    // 1, 3, 2

    // -2 10 2 1
    // 0<->0, 1<->1, 2<->2, 3<->3 (orig <-> curr)
    // orig = 0, curr = 0, num = -2
    // new_curr_for_orig = orig + num = -2 => -2 + 3 = 1
    // update off -2 to -1
    //   curr_to_update = curr + off = -2 => -2 + 3 = 1
    //   orig_to_update = curr2orig[curr_to_update] = 1
    //   new_curr = curr_to_update - 1 = 0
    //   insert orig_to_update<->new_curr
    //   0<->0, 1<->0, 2<->2, 3<->3
    //   curr_to_update = curr + off = -1 => -1 + 3 = 2
    //   orig_to_update = curr2orig[curr_to_update] = 2
    //   new_curr = curr_to_update + 1

    // 10 -2 2 1
    // -2 10 2 1
    // -2 2 10 1
    // -2 1 2 10

    // 2a 1 2b 3
    // 1 2b 2a 3
    // 2b 1 2a 3
    //
    fn mix(&mut self) {
        for (orig, num) in self.numbers.iter().enumerate() {
            //self.debug();
            //println!("moving {}", *num);
            let curr = self.orig2curr[&orig];
            let new_curr_for_orig = self.circle_add(curr, *num);
            // (orig, new_curr_for_orig)
            for curr_to_update in 0..new_curr_for_orig {
                // update with value to the right
                let curr_to_right = curr_to_update + 1;
                if curr_to_right <= curr {
                    continue;
                }
                let orig_from_right = self.curr2orig[&curr_to_right];
                self.curr2orig.insert(curr_to_update, orig_from_right);
                self.orig2curr.insert(orig_from_right, curr_to_update);
            }
            for curr_to_update in (new_curr_for_orig + 1..=curr).rev() {
                let curr_to_left = curr_to_update - 1;
                if curr_to_left >= curr {
                    continue;
                }
                let orig_from_left = self.curr2orig[&curr_to_left];
                self.curr2orig.insert(curr_to_update, orig_from_left);
                self.orig2curr.insert(orig_from_left, curr_to_update);
            }
            // println!("insert {} <--> {}", orig, new_curr_for_orig);
            self.curr2orig.insert(new_curr_for_orig, orig);
            self.orig2curr.insert(orig, new_curr_for_orig);
        }
        //self.debug();
    }
    fn apply_encrypt(&mut self, encrypt: i128) {
        for (i, num) in self.numbers.iter_mut().enumerate() {
            *num *= encrypt;
            self.decrypt_numbers[i] = *num;
            *num %= self.amount_numbers as i128 - 1;
        }
    }
    fn get_coordinates(&mut self) -> (i128, i128, i128) {
        let curr_zero = self.orig2curr[&self.orig_zero];
        let ix = (curr_zero + 1000) % self.amount_numbers;
        let iy = (ix + 1000) % self.amount_numbers;
        let iz = (iy + 1000) % self.amount_numbers;
        (
            self.decrypt_numbers[self.curr2orig[&ix]],
            self.decrypt_numbers[self.curr2orig[&iy]],
            self.decrypt_numbers[self.curr2orig[&iz]],
        )
    }
}

fn parse_input(input: &str) -> Result<TaskData> {
    let numbers: Vec<i128> = input
        .lines()
        .map(|l| i128::from_str_radix(l, 10).unwrap())
        .collect();
    let amount_numbers = numbers.len();
    let mut orig2curr = HashMap::new();
    let mut curr2orig = HashMap::new();
    let mut orig_zero = amount_numbers; // invalid value
    for (i, num) in numbers.iter().enumerate() {
        if num == &0 {
            orig_zero = i;
        }
        orig2curr.insert(i, i);
        curr2orig.insert(i, i);
    }
    Ok(TaskData {
        numbers: numbers.clone(),
        decrypt_numbers: numbers.clone(),
        amount_numbers,
        orig_zero,
        orig2curr,
        curr2orig,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.mix();
    let (x, y, z) = data.get_coordinates();
    println!("{} {} {}", x, y, z);
    Ok(x + y + z)
}

fn part_two(input: &str) -> Result<i128> {
    let mut data = parse_input(input)?;
    data.apply_encrypt(811589153);
    for i in 0..10 {
        println!("{}", i);
        data.mix();
    }
    let (x, y, z) = data.get_coordinates();
    println!("{} {} {}", x, y, z);
    Ok(x + y + z)
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
