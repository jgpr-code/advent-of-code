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

Common structure:

```Rust
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    part_one(&buffer);
    part_two(&buffer);
    Ok(())
}

fn parse_input(buffer: &str) -> T

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
