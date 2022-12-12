// #![allow(unused_variables)]

use std::collections::HashMap;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

#[derive(Debug, Clone)]
struct Board {
  data: Vec<u8>,
  width: usize,
}

trait BoardTrait {
  fn get(&self, pos: &(usize, usize)) -> u8;
  fn get_mut(&mut self, x: usize, y: usize) -> &mut u8;
  fn get_height(&mut self) -> usize;
  fn get_offset(&self, x: usize, y: usize) -> usize;
}

impl BoardTrait for Board {
  fn get(&self, pos: &(usize, usize)) -> u8 {
    self.data[pos.0 + pos.1 * self.width]
  }
  fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
    let w = self.width;
    &mut self.data[x + y * w]
  }
  fn get_height(&mut self) -> usize {
    self.data.len() / self.width
  }
  fn get_offset(&self, x: usize, y: usize) -> usize {
    x + y * self.width
  }
}

impl std::fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for h in 0..(self.data.len() / self.width) {
      if h != 0 {
        write!(f, "\n")?
      }
      for w in 0..self.width {
        write!(f, "{} ", self.get(&(w, h)))?
      }
    }
    Ok(())
  }
}

fn get_neighbor(pos : (usize, usize), width: usize, height: usize) -> [(usize, usize); 4] {
  return [
    (if pos.0 < width - 1 { pos.0 + 1 } else { pos.0 }, pos.1),
    (if pos.0 > 0 { pos.0 - 1 } else { pos.0 }, pos.1),
    (pos.0, if pos.1 < height - 1 { pos.1 + 1 } else { pos.1 }),
    (pos.0, if pos.1 > 0 { pos.1 - 1 } else { pos.1 }),
  ];
}

pub fn day13(filename: &Path) -> Result<ReturnType> {
  let mut board = Board {
    data: Vec::new(),
    width: 0,
  };
  let mut start = (0, 0);
  let mut end = (0, 0);
  for line in std::fs::read_to_string(filename)?.lines() {
    if board.width == 0 {
      board.width = line.chars().count();
    }
    for elevation in line.chars() {
      if elevation == 'S' {
        start = (board.data.len() % board.width, board.data.len() / board.width);
        board.data.push(0);
      } else if elevation == 'E' {
        end = (board.data.len() % board.width, board.data.len() / board.width);
        board.data.push(26);
      } else {
        board.data.push((elevation as u32 - 'a' as u32) as u8);
      }
    }
  }

  let mut part1 = 0;
  let mut part2 = 0;
  for is_part1 in [false, true] {
    let mut frontier = Vec::new();
    frontier.push((start, 0));
    let mut came_from = HashMap::new();
    came_from.insert(start, None);
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some((current, _)) = frontier.pop() {
      if end == current {
        break;
      }

      for next in get_neighbor(current, board.width, board.get_height()) {
        if board.get(&next) > board.get(&current) + 1 {
          continue;
        }
        let current_cost = cost_so_far.get(&current).ok_or("Previous pos not found")?;

        let new_cost = if is_part1 {
          current_cost + 1
        } else {
          // if the elevation is 0 (aka. a) we put a cost of 0.
          if board.get(&current) == 0 {
            0
          } else {
            current_cost + 1
          }
        };

        if !came_from.contains_key(&next) || new_cost < *cost_so_far.get(&next).ok_or("Previous pos not found")? {
          cost_so_far.insert(next, new_cost);
          frontier.push((next, new_cost));
          came_from.insert(next, Some(current));
        }
      }
      frontier.sort_by(|a, b| b.1.cmp(&a.1));
    }

    let mut current = end;
    let mut path = Vec::new();
    while current != start {
      path.push(current);
      let Some(&Some(temp)) = came_from.get(&current) else {
      panic!("No source found")
    };
      current = temp;
    }
    if is_part1 {
      part1 = path.len();
    } else {
      part2 = cost_so_far.get(&end).ok_or("No end value")? + 1;
    }
  }
  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day13,        "data/day13.txt",       [437, 430];
    test1:  day13,        "data/day13_test1.txt", [31, 29];
  );
}
