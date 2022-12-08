#![allow(unused_variables)]
use itertools::Itertools;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

pub fn day06(filename: &Path) -> Result<ReturnType> {
  let file_content = std::fs::read_to_string(filename)?;
  let content = file_content.lines().next().ok_or("Empty File")?;

  let part1 = content
    .chars()
    .collect::<Vec<_>>()
    .windows(4)
    .enumerate()
    .filter_map(|(index, chars)| {
      for v in chars.iter().permutations(2).unique() {
        if v[0] == v[1] {
          return None;
        }
      }
      Some(index + 4)
    })
    .next()
    .ok_or("No marker found")?;

  let part2 = content
    .chars()
    .collect::<Vec<_>>()
    .windows(14)
    .enumerate()
    .filter_map(|(index, chars)| {
      for v in chars.iter().permutations(2).unique() {
        if v[0] == v[1] {
          return None;
        }
      }
      Some(index + 14)
    })
    .next()
    .ok_or("No message found")?;

  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

pub fn day06_speed(filename: &Path) -> Result<ReturnType> {
  let file_content = std::fs::read_to_string(filename)?;
  let content = file_content.lines().next().ok_or("Empty File")?;

  let part1 = content
    .chars()
    .collect::<Vec<_>>()
    .windows(4)
    .enumerate()
    .filter_map(|(index, chars)| {
      let mut char_list = chars.iter().collect::<Vec<_>>();
      char_list.sort();
      char_list.dedup();
      if char_list.len() < 4 {
        return None;
      }
      Some(index + 4)
    })
    .next()
    .ok_or("No marker found")?;

  let part2 = content
    .chars()
    .collect::<Vec<_>>()
    .windows(14)
    .enumerate()
    .filter_map(|(index, chars)| {
      let mut char_list = chars.iter().collect::<Vec<_>>();
      char_list.sort();
      char_list.dedup();
      if char_list.len() < 14 {
        return None;
      }
      Some(index + 14)
    })
    .next()
    .ok_or("No message found")?;

  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day06,        "data/day06.txt",              [1282, 3513];
    test1:  day06,        "data/day06_test1.txt",        [7, 19];
  );
}
