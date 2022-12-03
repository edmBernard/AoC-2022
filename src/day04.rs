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

pub fn day04(filename: &Path) -> Result<[u64; 2]> {
  let file_content = std::fs::read_to_string(filename)?;
  let part1 = file_content
    .lines()
    .map(|line| {
      let compartments_size = line.len() / 2;
      let compartments1 = &line[..compartments_size];
      let compartments2 = &line[compartments_size..];
      for elem1 in compartments1.chars() {
        if compartments2.contains(elem1) {
          return to_priority(elem1) as u64;
        };
      }
      0
    })
    .sum();
  let part2 = file_content
    .lines()
    .collect::<Vec<_>>()
    .chunks(3)
    .map(|elfs_group| {
      for elem1 in elfs_group[0].chars() {
        if elfs_group[1].contains(elem1) && elfs_group[2].contains(elem1) {
          return to_priority(elem1) as u64;
        };
      }
      0
    })
    .sum();

  Ok([part1, part2])
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day04, "data/day04.txt",                  [8202, 2864];
    test1:  day04, "data/day04_test1.txt",            [157, 70];
  );
}
