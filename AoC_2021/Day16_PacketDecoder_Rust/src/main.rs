use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin
        .read_to_string(&mut buffer)
        .expect("failed to read file");
    let input = parse_buffer(&buffer);
    println!("{:?}", input);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

// there is probably a function for this but I couldn't find one fast enough so here is my thingy
fn hex_to_binary(c: char) -> String {
    String::from(match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    })
}

struct PacketReader<'a> {
    // returns the packet and the amount of bits it contained
    buffer: &'a str,
}

impl<'a> PacketReader<'a> {
    fn new(buffer: &'a str) -> Self {
        PacketReader { buffer }
    }
    fn read_packet(&mut self) -> (usize, Packet) {
        let mut total_read = 0;
        let version = u8::from_str_radix(self.read(3, &mut total_read), 2).unwrap();
        let type_id = u8::from_str_radix(self.read(3, &mut total_read), 2).unwrap();

        if type_id == 4 {
            // literal packet
            let mut binary_literal = String::new();
            loop {
                let part = self.read(5, &mut total_read);
                println!("{:?}", part);
                let (leading, content) = part.split_at(1);
                binary_literal.push_str(content);
                if leading == "0" {
                    break;
                }
            }
            let literal = u128::from_str_radix(&binary_literal, 2).unwrap();
            (
                total_read,
                Packet {
                    version,
                    type_id,
                    literal,
                    packets: Vec::new(),
                },
            )
        } else {
            let mut packets = Vec::new();
            let length_type_id = self.read(1, &mut total_read);
            if length_type_id == "0" {
                let length = usize::from_str_radix(self.read(15, &mut total_read), 2).unwrap();
                let mut sub_read = 0;
                while sub_read < length {
                    let (sub_read_inc, packet) = self.read_packet();
                    sub_read += sub_read_inc;
                    packets.push(packet);
                }
                total_read += sub_read
            } else {
                let num_packets = usize::from_str_radix(self.read(11, &mut total_read), 2).unwrap();
                for _ in 0..num_packets {
                    let (read_inc, packet) = self.read_packet();
                    total_read += read_inc;
                    packets.push(packet);
                }
            }
            (
                total_read,
                Packet {
                    version,
                    type_id,
                    literal: 0,
                    packets,
                },
            )
        }
    }

    fn read(&mut self, amount: usize, total: &mut usize) -> &str {
        let (read, rest) = self.buffer.split_at(amount);
        self.buffer = rest;
        *total += amount;
        read
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    literal: u128,
    packets: Vec<Packet>,
}

impl Packet {
    fn evaluate(&self) -> usize {
        0
    }
}

fn parse_buffer(buffer: &str) -> Packet {
    // convert hex to binary
    let binary: String = buffer
        .chars()
        .flat_map(|c| hex_to_binary(c).chars().collect::<Vec<_>>())
        .collect();

    let mut reader = PacketReader::new(&binary[..]);
    let (_, top_level_packet) = reader.read_packet();
    top_level_packet
}

fn part_one(top_level_packet: &Packet) -> u32 {
    println!("{:?}", parse_buffer("D2FE28"));
    let mut version_sum: u32 = 0;
    let mut packet_queue: VecDeque<&Packet> = VecDeque::new();
    packet_queue.push_back(top_level_packet);
    while let Some(packet) = packet_queue.pop_front() {
        version_sum += packet.version as u32;
        for contained_packet in packet.packets.iter() {
            packet_queue.push_back(contained_packet);
        }
    }
    version_sum
}

fn part_two(top_level_packet: &Packet) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_operator_operator_literal() {
        let packet = "8A004A801A8002F478";
        let input = parse_buffer(packet);
        assert_eq!(part_one(&input), 16);
    }

    #[test]
    fn operator_2operator_4literal_example_one() {
        let packet = "620080001611562C8802118E34";
        let input = parse_buffer(packet);
        assert_eq!(part_one(&input), 12);
    }

    #[test]
    fn operator_2operator_4literal_example_two() {
        let packet = "C0015000016115A2E0802F182340";
        let input = parse_buffer(packet);
        assert_eq!(part_one(&input), 23);
    }

    #[test]
    fn operator_operator_operator_5literal() {
        let packet = "A0016C880162017C3686B18A3D4780";
        let input = parse_buffer(packet);
        assert_eq!(part_one(&input), 31);
    }
}
