use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    let lines: Vec<&str> = buffer.lines().collect();

    part_one(&lines);
    part_two(&lines);
    Ok(())
}

fn part_one(lines: &Vec<&str>) {
    let num_reports = lines.len();
    let line_length = lines.first().unwrap().len();
    let mut count_ones: Vec<usize> = vec![0; line_length];

    for line in lines {
        for (i, _) in line.match_indices("1") {
            count_ones[i] += 1;
        }
    }

    let gamma_rate_binary: Vec<char> = count_ones
        .iter()
        .map(|c| if *c > num_reports / 2 { '1' } else { '0' })
        .collect();
    println!("{:?}", gamma_rate_binary);

    let gamma_rate_string: String = gamma_rate_binary.iter().collect();
    let epsilon_rate_string: String = gamma_rate_binary
        .iter()
        .map(|c| if *c == '1' { '0' } else { '1' })
        .collect();

    let gamma_rate = u32::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_string, 2).unwrap();

    println!("Part 1: {}", gamma_rate * epsilon_rate);
}

fn part_two(lines: &Vec<&str>) {
    let mut oxygen_generator_rating = lines.clone();
    let mut co2_scrubber_rating = lines.clone();
    for pos in 0..lines.first().unwrap().len() {
        if oxygen_generator_rating.len() != 1 {
            let required_in_pos =
                required_in_pos(pos, &oxygen_generator_rating, Rating::OxygenGenerator);
            oxygen_generator_rating = filter_by_pos(pos, required_in_pos, oxygen_generator_rating);
        }
        if co2_scrubber_rating.len() != 1 {
            let required_in_pos = required_in_pos(pos, &co2_scrubber_rating, Rating::Co2Scrubber);
            co2_scrubber_rating = filter_by_pos(pos, required_in_pos, co2_scrubber_rating);
        }
    }

    let oxygen_generator_rating =
        u32::from_str_radix(oxygen_generator_rating.first().unwrap(), 2).unwrap();
    let co2_scrubber_rating = u32::from_str_radix(co2_scrubber_rating.first().unwrap(), 2).unwrap();

    println!("Part 2: {}", oxygen_generator_rating * co2_scrubber_rating);
}

enum Rating {
    OxygenGenerator,
    Co2Scrubber,
}

fn required_in_pos(pos: usize, lines: &Vec<&str>, rating: Rating) -> char {
    let num_reports = lines.len();
    let ones = lines
        .iter()
        .map(|slice| slice.chars().nth(pos).unwrap())
        .filter(|c| *c == '1')
        .count();
    let zeros = num_reports - ones;

    match rating {
        Rating::OxygenGenerator => {
            if ones >= zeros {
                '1'
            } else {
                '0'
            }
        }
        Rating::Co2Scrubber => {
            if zeros <= ones {
                '0'
            } else {
                '1'
            }
        }
    }
}

fn filter_by_pos(pos: usize, required_in_pos: char, lines: Vec<&str>) -> Vec<&str> {
    lines
        .into_iter()
        .filter(|slice| (*slice).chars().nth(pos).unwrap() == required_in_pos)
        .collect()
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
