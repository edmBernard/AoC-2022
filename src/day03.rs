// #![allow(unused_variables)]
use std::path::Path;

use crate::Result;

fn to_priority(item: char) -> u32 {
  if item.is_lowercase() {
    item as u32 - 'a' as u32 + 1
  } else {
    item as u32 - 'A' as u32 + 27
  }
}

// fn result_part1(opponent: &str, mine: &str) -> Result<u64> {
//   let opponent_choice = to_int(opponent)?;
//   let my_choice = to_int(mine)?;
//   // I add +3 because I use unsigned integer and I don't want to have negative after substraction
//   let outcome_score = (((my_choice + 1) + 3 - (opponent_choice + 1) + 1) % 3) * 3;
//   Ok((my_choice+1) + outcome_score)
// }

// fn result_part2(opponent: &str, mine: &str) ->  Result<u64> {
//   let opponent_choice = to_int(opponent)?;
//   // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
//   // 0 means you need to lose, 1 means you need to end the round in a draw, and 2 means you need to win
//   let outcome = to_int(mine)?;

//   // I add +3 because I use unsigned integer and I don't want to have negative after substraction
//   let choice_score = ((opponent_choice + outcome + 3 - 1) % 3) + 1;
//   Ok(choice_score + outcome * 3)
// }

pub fn day03(filename: &Path) -> Result<[u64; 2]> {
  let mut part1 = 0;
  let mut part2 = 0;
  // let mut duplicates = Vec::new();
  for line in std::fs::read_to_string(filename)?.lines() {
    let compartments_size = line.len() / 2;
    let compartments1 = &line[..compartments_size];
    let compartments2 = &line[compartments_size..];
    part1 += 'block: {
      for elem1 in compartments1.chars() {
        if compartments2.contains(elem1) {
          println!("char:{} priority:{}", elem1, to_priority(elem1));
          break 'block to_priority(elem1) as u64;
        };
      }
      0
    }
  }
  part2 = std::fs::read_to_string(filename)?.lines().collect::<Vec<_>>().chunks(3)
  .map(|elfs_group| {
      for elem1 in elfs_group[0].chars() {
        if elfs_group[1].contains(elem1) && elfs_group[2].contains(elem1) {
          return to_priority(elem1) as u64;
        };
      }
      0
    }).sum();

  Ok([part1, part2])
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day03, "data/day03.txt",                  [8202, 16862];
    test1:  day03, "data/day03_test1.txt",            [157, 70];
  );
}
