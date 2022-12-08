#![allow(unused_variables)]
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

#[derive(Debug, Clone)]
struct Board {
  data: Vec<u8>,
  width: usize,
}

trait BoardTrait {
  fn get(&self, x: usize, y: usize) -> u8;
  fn get_mut(&mut self, x: usize, y: usize) -> &mut u8;
  fn get_height(&mut self) -> usize;
}

impl BoardTrait for Board {
  fn get(&self, x: usize, y: usize) -> u8 {
    self.data[x + y * self.width]
  }
  fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
    let w = self.width;
    &mut self.data[x + y * w]
  }
  fn get_height(&mut self) -> usize {
    self.data.len() / self.width
  }
}

impl std::fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for h in 0..(self.data.len() / self.width) {
      if h != 0 {
        write!(f, "\n")?
      }
      for w in 0..self.width {
        write!(f, "{} ", self.get(w, h))?
      }
    }
    Ok(())
  }
}

// I use vectorized board, that reduce the boundary check on vector access compare to vector of vector
pub fn day08(filename: &Path) -> Result<ReturnType> {
  let mut board = Board {
    data: Vec::new(),
    width: 0,
  };
  for line in std::fs::read_to_string(filename)?.lines() {
    if board.width == 0 {
      board.width = line.chars().count();
    }
    for tree in line.chars() {
      board
        .data
        .push(tree.to_digit(10).ok_or("Fail to parse tree height")? as u8);
    }
  }

  let mut part1 = (board.get_height() * 2 + board.width * 2 - 4) as u64;
  let mut part2 = 0;
  for y in 1..board.get_height() - 1 {
    for x in 1..board.width - 1 {
      let current = board.get(x, y);
      // toward Top
      let mut top = 0;
      let mut reach_top_border = true;
      for sight_y in (0..y).rev() {
        top += 1;
        if current > board.get(x, sight_y) {
        } else {
          reach_top_border = false;
          break;
        };
      }

      // toward Bottom
      let mut bottom = 0;
      let mut reach_bottom_border = true;
      for sight_y in y+1..board.get_height() {
        bottom += 1;
        if current > board.get(x, sight_y) {
        } else {
          reach_bottom_border = false;
          break;
        };
      }

      // toward Left
      let mut left = 0;
      let mut reach_left_border = true;
      for sight_x in (0..x).rev() {
        left += 1;
        if current > board.get(sight_x, y) {
        } else {
          reach_left_border = false;
          break;
        };
      }

      // toward Right
      let mut right = 0;
      let mut reach_right_border = true;
      for sight_x in x+1..board.width {
        right += 1;
        if current > board.get(sight_x, y) {
        } else {
          reach_right_border = false;
          break;
        };
      }

      part1 += (reach_top_border || reach_bottom_border || reach_left_border || reach_right_border) as u64;
      part2 = part2.max(top * bottom * left * right);
    }
  }
  Ok(ReturnType::Numeric(part1, part2))
}


// I use vectorized board, that reduce the boundary check on vector access compare to vector of vector
pub fn day08_speed(filename: &Path) -> Result<ReturnType> {
  let mut board = Board {
    data: Vec::new(),
    width: 0,
  };
  for line in std::fs::read_to_string(filename)?.lines() {
    if board.width == 0 {
      board.width = line.chars().count();
    }
    for tree in line.chars() {
      board
        .data
        .push(tree.to_digit(10).ok_or("Fail to parse tree height")? as u8);
    }
  }

  let mut part1 = (board.get_height() * 2 + board.width * 2 - 4) as u64;
  let mut part2 = 0;
  for y in 1..board.get_height() - 1 {
    for x in 1..board.width - 1 {
      let current = board.get(x, y);
      // toward Top
      let mut top = 0;
      let mut reach_top_border = true;
      for sight_y in (0..y).rev() {
        top += 1;
        if current > board.get(x, sight_y) {
        } else {
          reach_top_border = false;
          break;
        };
      }

      // toward Bottom
      let mut bottom = 0;
      let mut reach_bottom_border = true;
      for sight_y in y+1..board.get_height() {
        bottom += 1;
        if current > board.get(x, sight_y) {
        } else {
          reach_bottom_border = false;
          break;
        };
      }

      // toward Left
      let mut left = 0;
      let mut reach_left_border = true;
      for sight_x in (0..x).rev() {
        left += 1;
        if current > board.get(sight_x, y) {
        } else {
          reach_left_border = false;
          break;
        };
      }

      // toward Right
      let mut right = 0;
      let mut reach_right_border = true;
      for sight_x in x+1..board.width {
        right += 1;
        if current > board.get(sight_x, y) {
        } else {
          reach_right_border = false;
          break;
        };
      }

      part1 += (reach_top_border || reach_bottom_border || reach_left_border || reach_right_border) as u64;
      part2 = part2.max(top * bottom * left * right);
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
    main:   day08,        "data/day08.txt",              [1688, 410400];
    test1:  day08,        "data/day08_test1.txt",        [21, 8];
    main:   day08_speed,  "data/day08.txt",              [1688, 410400];
    test1:  day08_speed,  "data/day08_test1.txt",        [21, 8];
  );
}
