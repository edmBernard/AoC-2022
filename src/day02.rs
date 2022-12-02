// #![allow(unused_variables)]
use std::path::Path;

use crate::Result;

//A for Rock, B for Paper, and C for Scissors
//X for Rock, Y for Paper, and Z for Scissors
fn to_int(choice: &str) -> u64 {
  match choice {
    "A" => 0,
    "B" => 1,
    "C" => 2,
    "X" => 0,
    "Y" => 1,
    "Z" => 2,
    _ => 1000000,
  }
}

fn result_part1(opponent: &str, me: &str) -> u64 {
  let opponent_value = to_int(opponent);
  let me_value = to_int(me);
  let outcome_score = if me_value == opponent_value {
    3
  } else if (opponent_value + 1) % 3 == me_value {
    // (opponent_value + 1) % 3 is the choice that beat opponent_value
    6
  } else {
    0
  };
  (me_value+1) + outcome_score
}

fn result_part2(opponent: &str, me: &str) -> u64 {
  let opponent_value = to_int(opponent);
  // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
  // 0 means you need to lose, 1 means you need to end the round in a draw, and 2 means you need to win
  let outcome = to_int(me);
  let choice_score = if outcome == 1 {
    opponent_value+1
  } else if outcome == 2 {
    ((opponent_value+1) % 3) + 1
  } else {
    ((opponent_value+2) % 3) + 1
  };
  choice_score + outcome * 3
}
pub fn day02(filename: &Path) -> Result<[u64; 2]> {
  let mut part1 = 0;
  let mut part2 = 0;
  for line in std::fs::read_to_string(filename)?.lines() {
    let line_str = line.split(" ").collect::<Vec<_>>();
    part1 += result_part1(line_str[0], line_str[1]);
    part2 += result_part2(line_str[0], line_str[1]);
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
