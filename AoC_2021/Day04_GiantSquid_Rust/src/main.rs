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

fn parse_input(buffer: &str) -> (Vec<i32>, Vec<BingoCard>) {
    let mut iter = buffer.split_terminator("\r\n\r\n");

    let guesses: Vec<i32> = iter
        .nth(0)
        .expect("there was no guesses line")
        .split(",")
        .map(|x| x.parse::<i32>().expect("failed to parse a guess as i32"))
        .collect();

    let bingo_cards: Vec<BingoCard> = iter.map(|x| BingoCard::new(x)).collect();

    (guesses, bingo_cards)
}

fn part_one(buffer: &str) {
    let (guesses, mut bingo_cards) = parse_input(buffer);

    let (last_guess, first_winner) =
        play_until_first_winner(&guesses, &mut bingo_cards).expect("no first winner");

    println!("Part 1: {}", last_guess * first_winner.sum_of_unmarked());
}

fn part_two(buffer: &str) {
    let (guesses, mut bingo_cards) = parse_input(buffer);

    let (last_guess, last_winner) =
        play_until_last_winner(&guesses, &mut bingo_cards).expect("no last winner");

    println!("Part 2: {}", last_guess * last_winner.sum_of_unmarked());
}

fn play_until_first_winner(
    guesses: &[i32],
    bingo_cards: &mut [BingoCard],
) -> Option<(i32, BingoCard)> {
    for guess in guesses.iter() {
        for bingo_card in bingo_cards.iter_mut() {
            bingo_card.play_guess(guess);
            if bingo_card.is_winning {
                return Some((*guess, bingo_card.clone()));
            }
        }
    }
    None
}

fn play_until_last_winner(
    guesses: &[i32],
    bingo_cards: &mut [BingoCard],
) -> Option<(i32, BingoCard)> {
    let required_winners = bingo_cards.len();
    let mut current_winners = 0;
    for guess in guesses.iter() {
        for bingo_card in bingo_cards.iter_mut() {
            if bingo_card.is_winning {
                continue;
            }
            bingo_card.play_guess(guess);
            if bingo_card.is_winning {
                current_winners += 1;
                if current_winners == required_winners {
                    return Some((*guess, bingo_card.clone()));
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
struct BingoCard {
    card: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    num_to_pos: HashMap<i32, (usize, usize)>,
    correct_in_row: HashMap<usize, usize>,
    correct_in_col: HashMap<usize, usize>,
    marked_on_card: HashSet<i32>,
    pub is_winning: bool,
}

impl BingoCard {
    fn new(buffer: &str) -> BingoCard {
        let card: Vec<Vec<i32>> = buffer
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| {
                        x.parse::<i32>()
                            .expect("failed to parse a card entry as i32")
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let rows = card.len();
        let cols = card[0].len();
        let mut num_to_pos = HashMap::new();
        for row in 0..rows {
            for col in 0..cols {
                num_to_pos.insert(card[row][col], (row, col));
            }
        }

        BingoCard {
            card,
            rows,
            cols,
            num_to_pos,
            correct_in_row: HashMap::new(),
            correct_in_col: HashMap::new(),
            marked_on_card: HashSet::new(),
            is_winning: false,
        }
    }

    fn play_guess(&mut self, guess: &i32) {
        if self.marked_on_card.contains(guess) {
            return;
        }
        if let Some((row, col)) = self.num_to_pos.get(guess) {
            self.marked_on_card.insert(*guess);
            let in_row = self.correct_in_row.entry(*row).or_insert(0);
            let in_col = self.correct_in_col.entry(*col).or_insert(0);
            *in_row += 1;
            *in_col += 1;
            if *in_row == self.cols || *in_col == self.rows {
                self.is_winning = true;
            }
        }
    }

    fn sum_of_unmarked(&self) -> i32 {
        let mut sum = 0;
        for row in self.card.iter() {
            for value in row.iter() {
                if !self.marked_on_card.contains(value) {
                    sum += value;
                }
            }
        }
        sum
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
