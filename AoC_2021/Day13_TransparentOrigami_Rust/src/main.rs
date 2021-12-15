use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin
        .read_to_string(&mut buffer)
        .expect("failed to read file");
    let mut input_part_one = parse_buffer(&buffer);
    let mut input_part_two = input_part_one.clone();
    println!("{:?}", input_part_one);
    println!("Part 1: {}", part_one(&mut input_part_one));
    println!("Part 2: {}", part_two(&mut input_part_two));
}

struct Grid {
    content: Vec<Vec<char>>,
}

impl Grid {

}

#[derive(Debug, Clone)]
struct Origami {
    grid: Grid,
    instructions: Vec<FoldingInstruction>,
}

impl Origami {
    fn execute_instruction(&mut self, instruction: FoldingInstruction) {
        match instruction {
            FoldingInstruction::X(fold_at) => {
                // split at X
                // vertically_flip right part
            }
            FoldingInstruction::Y(fold_at) => {}
        }
    }

    fn flip(grid: Grid, axis: usize) -> Grid {
        grid.clone()
    }

    fn split(&self, axis: usize, at: usize) -> (Grid, Grid) {
        (grid, grid)
    }

    fn fold(&mut self, grid: Grid, axis: usize, at: usize) -> Grid {
        let (a, b) = self.split(axis, at);
        let new_grid = merge(a, b.flip(axis))
    }

    fn split_grid(grid: Grid, folding_instruction: FoldingInstruction) -> (Grid, Grid) {
        (grid, grid)
    }

    fn merge_grids(grid1: Grid, grid2: Grid) -> Grid {
        let merged = grid1.clone();
        let rows = merged.len();
        let cols = merged[0].len();
        for row in 0..rows {
            for col in 0..cols {}
        }
        merged
    }

    fn merge_dots(dot1: char, dot2: char) -> char {
        match (dot1, dot2) {
            ('#', _) => '#',
            (_, '#') => '#',
            _ => '.',
        }
    }
}

#[derive(Debug, Clone)]
enum FoldingInstruction {
    X(usize),
    Y(usize),
}

fn parse_buffer(buffer: &str) -> Origami {
    Origami {}
}

fn part_one(origami: &mut Origami) -> i64 {
    0
}

fn part_two(origami: &mut Origami) -> i64 {
    0
}
