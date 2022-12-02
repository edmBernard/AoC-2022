// #![allow(unused_variables)]

use std::path::Path;

use crate::Result;

pub fn day01(filename: &Path) -> Result<[u64; 2]> {

  let mut input_puzzle = Vec::new();
  let mut one_elf = Vec::new();
  for line in std::fs::read_to_string(filename)?.lines() {
    let line_str = line;
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


pub fn day01_speed(filename: &Path) -> Result<[u64; 2]> {

  let mut input_puzzle = Vec::new();
  let mut one_elf = 0;
  for line in std::fs::read_to_string(filename)?.lines() {
    let line_str = line;
    if line_str.is_empty() {
      input_puzzle.push(one_elf);
      one_elf = 0;
      continue;
    }
    let value = line_str.parse::<u64>()?;
    one_elf += value;
  }
  // Push last elf inventory if the puzzle don't end with new line
  if one_elf != 0 {
    input_puzzle.push(one_elf);
  }

  input_puzzle.sort();
  input_puzzle.reverse();

  // part1
  let part1: u64 = input_puzzle[0];

  // part2
  let part2: u64 = input_puzzle[0..3].iter().sum();

  Ok([part1, part2])
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day01,        "data/day01.txt",       [70720, 207148];
    test1:  day01,        "data/day01_test1.txt", [24000, 45000];
    main:   day01_speed,  "data/day01.txt",       [70720, 207148];
    test1:  day01_speed,  "data/day01_test1.txt", [24000, 45000];
  );
}
