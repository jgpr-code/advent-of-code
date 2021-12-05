use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    part_one(&buffer);
    Ok(())
}

fn part_one(buffer: &str) {
    let mut iter = buffer.split_terminator("\r\n\r\n");

    let guesses: Vec<i32> = iter
        .nth(0)
        .expect("there was no guesses line")
        .split(",")
        .map(|x| x.parse::<i32>().expect("failed to parse a guess as i32"))
        .collect();

    let mut bingo_cards: Vec<BingoCard> = iter.map(|x| BingoCard::new(x)).collect();

    let (last_guess, bingo_winner) =
        play_bingo(&guesses, &mut bingo_cards).expect("no winner at all");

    println!("Part 1: {:?}", last_guess * bingo_winner.sum_of_unmarked());

    //println!("{:?}", guesses);
    //println!("{:?}", bingo_cards);
}

fn play_bingo(guesses: &[i32], bingo_cards: &mut [BingoCard]) -> Option<(i32, BingoCard)> {
    for i in 0..guesses.len() {
        for bingo_card in bingo_cards.iter_mut() {
            if bingo_card.play_guess(&guesses[i]) {
                return Some((guesses[i], bingo_card.clone()));
            }
        }
    }
    None
}

//fn play_bingo_guess(guess: i32, )

#[derive(Debug, Clone)]
struct BingoCard {
    card: Vec<Vec<i32>>,

    num_to_pos: HashMap<i32, (usize, usize)>,

    correct_in_row: HashMap<usize, usize>,
    correct_in_col: HashMap<usize, usize>,

    marked_on_card: HashSet<i32>,
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
        let mut correct_in_row = HashMap::new();
        for row in 0..rows {
            correct_in_row.insert(row, 0);
        }
        let mut correct_in_col = HashMap::new();
        for col in 0..cols {
            correct_in_col.insert(col, 0);
        }

        BingoCard {
            card,
            num_to_pos,
            correct_in_row,
            correct_in_col,
            marked_on_card: HashSet::new(),
        }
    }

    fn play_guess(&mut self, guess: &i32) -> bool {
        if self.marked_on_card.contains(guess) {
            return false;
        }
        if let Some((row, col)) = self.num_to_pos.get(guess) {
            let in_row = self.correct_in_row.entry(*row).or_insert(0);
            let in_col = self.correct_in_col.entry(*col).or_insert(0);
            *in_row += 1;
            *in_col += 1;
            self.marked_on_card.insert(*guess);
            if *in_row == 5 || *in_col == 5 {
                return true;
            }
        }
        return false;
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
