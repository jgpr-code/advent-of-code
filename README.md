# AdventOfCode

This repository collects all solutions to AdventOfCode over the years.

## Structure

- Separate commonly used utility into own projects the naming scheme for those will be ``Utility_<Lang>`` (e.g. Utility_Cpp).
- ``AoC_<Year>`` contains the entries:
  - ``Day<dd>_<Name>_<Lang>`` which always is its own project!

## Languages

- C#
- C++
- Clojure
- F#
- Haskell
- Kotlin
- Powershell
- Python
- Racket
- Rust
- Typescript

### Rust

- Utility crate that copies a common template instead of using cargo new
  - need to learn how to add own cargo commands
  - of course also need to implement this copy functionality afterwards
  - should maybe support different templates
  - this is probably like a cli tool so check out the crates for that
- Own AoC utiltiy crate
- Use anyhow or thiserror or build own error functionality (can go in aoc crate as well)

Common structure:

```Rust
use std::io::{self, Read};

fn main() -> /* TODO */ {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    let input = parse_buffer(&buffer)?;
    part_one(&input);
    part_two(&input);
    Ok(())
}

fn parse_buffer(buffer: &str) -> T

fn part_one(buffer: &str) {
    let T = parse_input(buffer);
    ...
}

fn part_two(buffer: &str) {
    let T = parse_input(buffer);
    ...
}

// TODO: unit_tests with timing
#[cfg(test)]
mod tests {
    fn test_part_one() // run part_one on test.txt
    fn test_part_two() // run part_two on test.txt
    fn input_part_one() // run part_one on input.txt
    fn input_part_two() // run part_two on input.txt
}

```

How I imagine it should look for a days implementation:

```Rust

// gets up to two (could also be more) functions (part_one, part_two)
// and runs them on the usual input stuff
aoc_main!(part_one, part_two)

fn parse_input() { ... }

fn part_one() { ... }

fn part_two() { ... }

// gets up to two (could also be more) functions (part_one, part_two) with their results
// and creates the necessary tests
aoc_tests!((part_one, "expected"), (part_two, "expected"))


```

what are the goals for each task?

running part1 and part2 and the tests test.txt (small) and input.txt (real)

this is a two part task:

1. the aoc utilities library
2. cargo generate template

workflow:

1. cargo generate template
2. create test.txt and input.txt by copying stuff from aoc website manually (maybe improve this later)
3. implement parse_input part_one and part_two
4. DONE!

how is data transformed during the different parts of the program?

input is parsed into a custom type (this might be a library type)
e.g. Vec<i64> or something completely different like FooInput.

part_one and part_two handle that type