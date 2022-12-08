#![allow(unused_variables)]
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

pub fn day08(filename: &Path) -> Result<ReturnType> {
  let mut board_part1: Vec<Vec<u8>> = Vec::new();
  let mut board_part2: Vec<Vec<u8>> = Vec::new();
  for line in std::fs::read_to_string(filename)?.lines() {
    board_part1.push(Vec::new());
    for tree in line.chars() {
      board_part1
        .last_mut()
        .ok_or("board is empty")?
        .push(tree.to_digit(10).ok_or("Fail to parse tree height")? as u8);
    }
  }
  let height = board_part1.len();
  let width = board_part1[0].len();
  // let mut board_max_top_bottom: Vec<Vec<u8>> = vec![vec![0; width]; height];
  let mut board_max_top_bottom = board_part1.clone();
  let mut board_max_left_right = board_part1.clone();
  for row in 1..height - 1 {
    for col in 1..width - 1 {
      board_max_top_bottom[row][col] = board_part1[row -1 ][col].max(board_max_top_bottom[row - 1][col]);
      board_max_left_right[row][col] = board_part1[row][col - 1].max(board_max_left_right[row][col - 1]);
    }
  }
  let mut board_max_bottom_top = board_part1.clone();
  let mut board_max_right_left = board_part1.clone();
  for row in (1..height - 1).rev() {
    for col in (1..width - 1).rev() {
      board_max_bottom_top[row][col] = board_part1[row + 1][col].max(board_max_bottom_top[row + 1][col]);
      board_max_right_left[row][col] = board_part1[row][col + 1].max(board_max_right_left[row][col + 1]);
    }
  }

  let mut part1 = (height * 2 + width * 2 - 4) as u64;
  for row in 1..height - 1 {
    for col in 1..width - 1 {
      let tree = board_part1[row][col];
      part1 += if tree > board_max_top_bottom[row][col]
        || tree > board_max_left_right[row][col]
        || tree > board_max_bottom_top[row][col]
        || tree > board_max_right_left[row][col]
      {
        1
      } else {
        0
      }
    }
  }
  let mut part2 = 0;
  for row in 0..height {
    for col in 0..width {

      let tree = board_part1[row][col];
      let top = 'block: {
        for (index, r) in (0..row).rev().enumerate() {
          if board_part1[r][col] >= tree {
            break 'block index+1;
          }
        };
        (0..row).len()
      };
      let left = 'block: {
        for (index, c) in (0..col).rev().enumerate() {
          if board_part1[row][c] >= tree {
            break 'block index+1;
          }
        };
        (0..col).len()
      };
      let bottom = 'block: {
        for (index, r) in (row+1..height).enumerate() {
          if board_part1[r][col] >= tree {
            break 'block index+1;
          }
        };
        (row+1..height).len()
      };
      let right = 'block: {
        for (index, c) in (col+1..width).enumerate() {
          if board_part1[row][c] >= tree {
            break 'block index+1;
          }
        };
        (col+1..width).len()
      };
      part2 = part2.max(top * left * bottom * right);
    }
  };
  Ok(ReturnType::Numeric(part1, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day08,        "data/day08.txt",              [1688, 410400];
    test1:  day08,        "data/day08_test1.txt",        [21, 8];
  );
}
