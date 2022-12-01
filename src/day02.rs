// #![allow(unused_variables)]

use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::Result;

pub fn day02(filename: &Path) -> Result<[u64; 2]> {
  let input = std::fs::File::open(filename)?;

  let buffered = BufReader::new(input);

  let mut input_puzzle = Vec::new();
  let mut one_elf = Vec::new();
  for line in buffered.lines() {
    let line_str = line?;
    if line_str.is_empty() {
      input_puzzle.push(one_elf.clone());
      one_elf.clear();
      continue;
    }
    let value = line_str.parse()?;
    one_elf.push(value);
  }
  // Push last elf inventory if the puzzle don't end with new line
  if !one_elf.is_empty() {
    input_puzzle.push(one_elf);
  }

  let mut prep_puzzle: Vec<u64> = input_puzzle.iter()
    .map(|one_elf| one_elf.iter().sum())
    .collect::<Vec<_>>();
  prep_puzzle.sort();
  prep_puzzle.reverse();

  // part1
  let part1: u64 = prep_puzzle[0];

  // part2
  let part2: u64 = prep_puzzle[0..3].iter().sum();

  Ok([part1, part2])
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day02, "data/day02.txt",                  [70720, 207148];
    test1:  day02, "data/day02_test1.txt",            [24000, 45000];
  );
}
