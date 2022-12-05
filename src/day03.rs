// #![allow(unused_variables)]
use std::path::Path;

use crate::Result;
use crate::utils::ReturnType;

fn to_priority(item: char) -> u32 {
  if item.is_lowercase() {
    item as u32 - 'a' as u32 + 1
  } else {
    item as u32 - 'A' as u32 + 27
  }
}

pub fn day03(filename: &Path) -> Result<ReturnType> {
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

    Ok(ReturnType::Numeric(part1, part2))
  }

pub fn day03_speed(filename: &Path) -> Result<ReturnType> {
  let mut chunk = Vec::new();
  let mut part1 = 0;
  let mut part2 = 0;
  for line in std::fs::read_to_string(filename)?.lines() {
    // part1
    let compartments_size = line.len() / 2;
    let compartments1 = &line[..compartments_size];
    let compartments2 = &line[compartments_size..];
    part1 += 'part1: {
      for elem1 in compartments1.chars() {
        if compartments2.contains(elem1) {
          break 'part1 to_priority(elem1) as u64;
        };
      }
      0
    };
    // part2
    chunk.push(line);
    if chunk.len() == 3 {
      part2 += 'part2: {
        for elem1 in chunk[0].chars() {
          if chunk[1].contains(elem1) && chunk[2].contains(elem1) {
            break 'part2 to_priority(elem1) as u64;
          };
        }
        0
      };
      chunk.clear();
    }
  }

  Ok(ReturnType::Numeric(part1, part2))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day03, "data/day03.txt",                  [8202, 2864];
    test1:  day03, "data/day03_test1.txt",            [157, 70];
    main:   day03_speed, "data/day03.txt",            [8202, 2864];
    test1:  day03_speed, "data/day03_test1.txt",      [157, 70];
  );
}
