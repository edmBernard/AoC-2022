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
      let mut char_list = chars.to_owned();
      char_list.sort();
      for (v1, v2) in char_list.iter().tuple_windows::<(_, _)>() {
        if v1 == v2 {
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
      let mut char_list = chars.to_owned();  // to_own is faster than .iter.collect
      char_list.sort();
      // tuple_windows is faster than using dedup and checking the length
      for (v1, v2) in char_list.iter().tuple_windows::<(_, _)>() {
        if v1 == v2 {
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

  const MARKER_LEN: usize = 4;
  let part1 = content
    .chars()
    .collect::<Vec<_>>()
    .windows(MARKER_LEN)
    .enumerate()
    .find_map(|(index, chars)| {
      for i in 0..MARKER_LEN-1 {
        if chars[i+1..].contains(&chars[i]) {
          return None;
        }
      }
      Some(index + MARKER_LEN)
    })
    .ok_or("No marker found")?;

  const MESSAGE_LEN: usize = 14;
  let part2 = content
    .chars()
    .collect::<Vec<_>>()
    .windows(MESSAGE_LEN)
    .enumerate()
    .find_map(|(index, chars)| {
      // this simpler solution was so much faster
      for i in 0..MESSAGE_LEN-1 {
        if chars[i+1..].contains(&chars[i]) {
          return None;
        }
      }
      Some(index + MESSAGE_LEN)
    })
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
    main:   day06_speed,  "data/day06.txt",              [1282, 3513];
    test1:  day06_speed,  "data/day06_test1.txt",        [7, 19];
  );
}
