// #![allow(unused_variables)]
use std::path::Path;

use crate::Result;
use crate::utils::ReturnType;

// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
trait InterpreteToInt {
  fn interprete(&self) -> Result<u64>;
}

impl InterpreteToInt for &str {
  fn interprete(&self) -> Result<u64> {
    match *self {
      "A" => Ok(0),
      "B" => Ok(1),
      "C" => Ok(2),
      "X" => Ok(0),
      "Y" => Ok(1),
      "Z" => Ok(2),
      character => Err(format!("Invalid character : {}", character).into()),
    }
  }
}

impl InterpreteToInt for char {
  #[inline(always)]
  fn interprete(&self) -> Result<u64> {
    match self {
      'A' => Ok(0),
      'B' => Ok(1),
      'C' => Ok(2),
      'X' => Ok(0),
      'Y' => Ok(1),
      'Z' => Ok(2),
      character => Err(format!("Invalid character : {}", character).into()),
    }
  }
}

#[inline(always)]
fn result_part1(opponent_choice: u64, my_choice: u64) -> Result<u64> {
  // I add +3 because I use unsigned integer and I don't want to have negative after substraction
  let outcome_score = (((my_choice + 1) + 3 - (opponent_choice + 1) + 1) % 3) * 3;
  Ok((my_choice+1) + outcome_score)
}

#[inline(always)]
fn result_part2(opponent_choice: u64, outcome: u64) -> Result<u64> {
  // 0 means you need to lose, 1 means you need to end the round in a draw, and 2 means you need to win
  // I add +3 because I use unsigned integer and I don't want to have negative after substraction
  let choice_score = ((opponent_choice + outcome + 3 - 1) % 3) + 1;
  Ok(choice_score + outcome * 3)
}

pub fn day02(filename: &Path) -> Result<ReturnType> {
  let mut part1 = 0;
  let mut part2 = 0;
  for line in std::fs::read_to_string(filename)?.lines() {
    let line_str = line.split(" ").filter_map(|v| v.interprete().ok()).collect::<Vec<_>>();
    part1 += result_part1(line_str[0], line_str[1])?;
    part2 += result_part2(line_str[0], line_str[1])?;
  }

  Ok(ReturnType::Numeric(part1, part2))
}

pub fn day02_speed(filename: &Path) -> Result<ReturnType> {
  let mut part1 = 0;
  let mut part2 = 0;
  for line in std::fs::read_to_string(filename)?.lines() {
    let value1 = line.chars().nth(0).expect("Can't index/Out of bound").interprete()?;
    let value2 = line.chars().nth(2).expect("Can't index/Out of bound").interprete()?;
    part1 += result_part1(value1, value2)?;
    part2 += result_part2(value1, value2)?;
  }

  Ok(ReturnType::Numeric(part1, part2))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day02,        "data/day02.txt",       [11475, 16862];
    test1:  day02,        "data/day02_test1.txt", [15, 12];
    main:   day02_speed,  "data/day02.txt",       [11475, 16862];
    test1:  day02_speed,  "data/day02_test1.txt", [15, 12];
  );
}
