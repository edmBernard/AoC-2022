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
  fn get(&self, x: usize, y: usize) -> u8;
  fn get_mut(&mut self, x: usize, y: usize) -> &mut u8;
  fn get_height(&mut self) -> usize;
  fn get_offset(&self, x: usize, y: usize) -> usize;
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
        write!(f, "{} ", self.get(w, h))?
      }
    }
    Ok(())
  }
}

fn get_neighbor(x: usize, y: usize, width: usize, height: usize) -> [(usize, usize); 4] {
  return [
    (if x < width - 1 { x + 1 } else { x }, y),
    (if x > 0 { x - 1 } else { x }, y),
    (x, if y < height - 1 { y + 1 } else { y }),
    (x, if y > 0 { y - 1 } else { y }),
  ];
}

pub fn day12(filename: &Path) -> Result<ReturnType> {
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
    // TODO: make part1 and part2 in one loop
    let mut frontier = Vec::new();
    frontier.push((start, 0));
    let mut came_from = HashMap::new();
    // TODO: maybe we don't need came_from as we only need the len() not the path
    came_from.insert(start, None);
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some((current, _)) = frontier.pop() {
      if end == current {
        break;
      }

      for next in get_neighbor(current.0, current.1, board.width, board.get_height()) {
        if board.get(next.0, next.1) > board.get(current.0, current.1) + 1 {
          continue;
        }
        let current_cost = cost_so_far.get(&current).ok_or("Previous pos not found")?;

        let new_cost = if is_part1 {
          current_cost + 1
        } else {
          // if the elevation is 0 (aka. a) we put a cost of 0.
          if board.get(current.0, current.1) == 0 {
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

pub fn day12_speed(filename: &Path) -> Result<ReturnType> {
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

  // Dijkstra’s Algorithm
  // https://www.redblobgames.com/pathfinding/a-star/introduction.html
  let mut part1 = 0;
  let mut part2 = 0;
  let mut frontier = Vec::new();
  frontier.push((end, 0));
  let mut cost_so_far = HashMap::new();
  cost_so_far.insert(end, 0);

  while let Some((current, _)) = frontier.pop() {
    for next in get_neighbor(current.0, current.1, board.width, board.get_height()) {
      if board.get(next.0, next.1) < board.get(current.0, current.1) - 1 {
        continue;
      }
      let current_cost = cost_so_far.get(&current).ok_or("Previous pos not found")?;

      let new_cost = current_cost + 1;

      if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).ok_or("Previous pos not found")? {
        cost_so_far.insert(next, new_cost);
        frontier.push((next, new_cost));
      }
    }
    if part2 == 0 && board.get(current.0, current.1) == 0 {
      part2 = *cost_so_far.get(&current).ok_or("No start value")?;
    }
    if part1 == 0 && start == current {
      part1 = *cost_so_far.get(&current).ok_or("No start value")?;
      break;
    }
    frontier.sort_by(|a, b| b.1.cmp(&a.1));
  }

  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day12,        "data/day12.txt",       [437, 430];
    test1:  day12,        "data/day12_test1.txt", [31, 29];
  );
}
