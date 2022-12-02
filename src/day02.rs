// #![allow(unused_variables)]
use std::path::Path;

use crate::Result;

// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
fn to_int(choice: &str) -> Result<u64> {
  match choice {
    "A" => Ok(0),
    "B" => Ok(1),
    "C" => Ok(2),
    "X" => Ok(0),
    "Y" => Ok(1),
    "Z" => Ok(2),
    character => Err(format!("Invalid character : {}", character).into()),
  }
}

fn result_part1(opponent: &str, mine: &str) -> Result<u64> {
  let opponent_choice = to_int(opponent)?;
  let my_choice = to_int(mine)?;
  // I add +3 because I use unsigned integer and I don't want to have negative after substraction
  let outcome_score = (((my_choice + 1) + 3 - (opponent_choice + 1) + 1) % 3) * 3;
  Ok((my_choice+1) + outcome_score)
}

fn result_part2(opponent: &str, mine: &str) ->  Result<u64> {
  let opponent_choice = to_int(opponent)?;
  // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
  // 0 means you need to lose, 1 means you need to end the round in a draw, and 2 means you need to win
  let outcome = to_int(mine)?;

  // I add +3 because I use unsigned integer and I don't want to have negative after substraction
  let choice_score = ((opponent_choice + outcome + 3 - 1) % 3) + 1;
  Ok(choice_score + outcome * 3)
}

pub fn day02(filename: &Path) -> Result<[u64; 2]> {
  let mut part1 = 0;
  let mut part2 = 0;
  for line in std::fs::read_to_string(filename)?.lines() {
    let line_str = line.split(" ").collect::<Vec<_>>();
    part1 += result_part1(line_str[0], line_str[1])?;
    part2 += result_part2(line_str[0], line_str[1])?;
  }

  Ok([part1, part2])
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day02, "data/day02.txt",                  [11475, 16862];
    test1:  day02, "data/day02_test1.txt",            [15, 12];
  );
}
