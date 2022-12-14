use anyhow::Result;
use std::cmp::Ordering;
use std::io::{self, Read};
//use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Elem(i128),
    List(Vec<Packet>),
}

impl Packet {
    fn extract_elem(line: &[char]) -> (i128, usize) {
        let mut s = String::new();
        let mut line_it = line.iter();
        let mut read = 0;
        while let Some(&c) = line_it.next() {
            match c {
                ']' => break, // don't read away ]
                ',' => {
                    read += 1;
                    break;
                }
                c if c.is_ascii_digit() => {
                    read += 1;
                    s.push(c)
                }
                _ => panic!("extract elem encountered unexpected elem {}", c),
            }
        }
        (i128::from_str_radix(&s, 10).unwrap(), read)
    }
}

fn cmp_slice(l: &[Packet], r: &[Packet]) -> Option<std::cmp::Ordering> {
    let mut pos = 0;
    loop {
        let opl = l.get(pos);
        let opr = r.get(pos);
        match (opl, opr) {
            (None, None) => return Some(Ordering::Equal),
            (Some(_), None) => return Some(Ordering::Greater),
            (None, Some(_)) => return Some(Ordering::Less),
            (Some(pl), Some(pr)) => match pl.partial_cmp(pr) {
                Some(Ordering::Equal) => pos += 1,
                ord => return ord,
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        println!("Compare: {:?}, {:?}", self, other);
        match (self, other) {
            (Packet::Elem(l), Packet::Elem(r)) => l.partial_cmp(r),
            (Packet::List(l), Packet::List(r)) => cmp_slice(l, r),
            (Packet::Elem(l), r) => Packet::List(vec![Packet::Elem(*l)]).partial_cmp(r),
            (l, Packet::Elem(r)) => l.partial_cmp(&Packet::List(vec![Packet::Elem(*r)])),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for Packet {
    fn from(packet_line: &str) -> Self {
        let line_chars: Vec<char> = packet_line.chars().collect();
        let mut pos = 0;
        let mut stack: Vec<Packet> = Vec::new();
        // println!("{:?}", line_chars);
        while pos < packet_line.len() {
            // println!("pos: {}", pos);
            // println!("stack: {:?}", stack);
            // println!("at {}", line_chars[pos]);
            match line_chars[pos] {
                '[' => {
                    stack.push(Packet::List(Vec::new()));
                    pos += 1;
                }
                c if c.is_ascii_digit() => {
                    let (elem, inc) = Self::extract_elem(&line_chars[pos..]);
                    pos += inc;
                    if let Some(packet) = stack.last_mut() {
                        match packet {
                            Packet::List(l) => l.push(Packet::Elem(elem)),
                            _ => panic!("unexpected stack top"),
                        }
                    } else {
                        return Packet::Elem(elem);
                    }
                }
                ']' => {
                    pos += 1;
                    if let Some(finished_packet) = stack.pop() {
                        if let Some(target_packet) = stack.last_mut() {
                            match target_packet {
                                Packet::List(l) => l.push(finished_packet),
                                _ => panic!("unexpected stack top"),
                            }
                        } else {
                            return finished_packet;
                        }
                    }
                }
                ',' => pos += 1, // can happen after ]
                e => panic!("unexpected pos at {}", e),
            };
        }
        panic!("packet was not created succesfully");
        // if [ -> start new PacketList

        // [[4,[4]],4]
        // [ -> Vec<Packet> 1
        //  [ -> Vec<Packet> 2
        //   4 -> PacketElem for 2
        //    , -> finish PacketElem for 2
        //     [ -> Vec<Packet> 3
        //      4 -> PacketElem for 3
        //       ] -> finish PacketElem for 3, finish Vec<Packet> 3

        // [42,[]]
        // [ -> Vec<Packet> 1
        //  4 -> PacketElem for 1
        //   2 -> PacketElem for 1
        //    , -> finish PacketElem for 1
    }
}

#[derive(Debug)]
struct TaskData {
    packet_pairs: Vec<(Packet, Packet)>,
    debug_pairs: Vec<(String, String)>,
}

impl TaskData {}

fn parse_input(input: &str) -> Result<TaskData> {
    let mut packet_pairs = Vec::new();
    let mut debug_pairs = Vec::new();
    for packet_pair in input.split("\r\n\r\n") {
        let mut split = packet_pair.split("\r\n");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        packet_pairs.push((Packet::from(first), Packet::from(second)));
        debug_pairs.push((String::from(first), String::from(second)));
    }
    Ok(TaskData {
        packet_pairs,
        debug_pairs,
    })
}

fn part_one(input: &str) -> Result<i128> {
    let TaskData {
        packet_pairs,
        debug_pairs,
    } = parse_input(input)?;
    let mut i_sum = 0;
    for (i, (l, r)) in packet_pairs.iter().enumerate() {
        let comp = l < r;
        println!("{} == {} < {}", comp, debug_pairs[i].0, debug_pairs[i].1);
        println!("");
        if comp {
            i_sum += i + 1
        }
    }
    Ok(i_sum as i128)
}

fn part_two(input: &str) -> Result<i128> {
    let TaskData { packet_pairs, .. } = parse_input(input)?;
    let a_divider_packet = Packet::List(vec![(Packet::List(vec![Packet::Elem(2)]))]);
    let b_divider_packet = Packet::List(vec![(Packet::List(vec![Packet::Elem(6)]))]);
    let mut all_packets = Vec::new();
    for (l, r) in packet_pairs.iter() {
        all_packets.push(l);
        all_packets.push(r);
    }
    all_packets.push(&a_divider_packet);
    all_packets.push(&b_divider_packet);
    all_packets.sort();
    let mut distress = 1;
    for (i, &p) in all_packets.iter().enumerate() {
        if p == &a_divider_packet || p == &b_divider_packet {
            distress *= i + 1;
        }
    }
    Ok(distress as i128)
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
        assert_eq!(answer, 13);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 5330);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 140);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 27648);
        Ok(())
    }
}
