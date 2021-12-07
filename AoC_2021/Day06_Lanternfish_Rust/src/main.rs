use std::collections::HashMap;
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
    let initial_timers: Vec<i64> = buffer
        .split(",")
        .map(|s| s.parse().expect("couldn't parse as i64"))
        .collect();
    let amount_fishes = fishes_for_days(&initial_timers, 80);

    println!("Part 1: {}", amount_fishes);
}

fn fishes_for_days(initial_timers: &Vec<i64>, days: i64) -> i64 {
    let mut swarm = Lanternfishes {
        memo: HashMap::new(),
    };
    let amount_fishes = initial_timers
        .iter()
        .fold(0, |acc, t| acc + swarm.how_many_fish(*t, days));
    amount_fishes
}

fn part_two(buffer: &str) {
    let initial_timers: Vec<i64> = buffer
        .split(",")
        .map(|s| s.parse().expect("couldn't parse as i64"))
        .collect();
    let amount_fishes = fishes_for_days(&initial_timers, 256);

    println!("Part 2: {}", amount_fishes);
}

struct Lanternfishes {
    memo: HashMap<(i64, i64), i64>,
}
impl Lanternfishes {
    // question: how many fish did this fish spawn in his lifetime

    // example with initial spawn 1
    // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16
    // 1->0->6->5->4->3->2->1->0->6->5->4->3->2->1->0->6 ...
    //       S                    S                    S
    // f(1, 0..1) = 0
    // f(1, 2..8) = 1
    // f(1, 9..15) = 2
    // f(1, 16..22) = 3

    // example with initial spawn 4
    // 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19
    // 4->3->2->1->0->6->5->4->3->2->1->0->6->5->4->3->2->1->0->6
    //                S                    S                    S
    // f(4, 0..4) = 0
    // f(4, 5..11) = 1
    // f(4, 12..18) = 2

    // recursion for f(1, 16)
    // f(1, 13) = 1 + f(8, 16-1-1) + f(8, (16-1-1-7))
    fn how_many_fish(&mut self, initial_timer: i64, days_remaining: i64) -> i64 {
        let memo_key = (initial_timer, days_remaining);
        if self.memo.contains_key(&memo_key) {
            return self.memo[&memo_key];
        }
        let mut answer = 1;
        let mut remaining_days = days_remaining;
        remaining_days -= initial_timer + 1;
        while remaining_days >= 0 {
            answer += self.how_many_fish(8, remaining_days);
            remaining_days -= 7;
        }
        self.memo.insert(memo_key, answer);
        answer
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
