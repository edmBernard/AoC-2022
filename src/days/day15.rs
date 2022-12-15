// #![allow(unused_variables)]

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::zip;
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

fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
  (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn day15(filename: &Path) -> Result<ReturnType> {
  let mut beacon_position = Vec::new();
  let mut sensor_position = Vec::new();
  let regex = Regex::new(r"(-?\d+)")?;
  for line in std::fs::read_to_string(filename)?.lines() {
    let raw_position = regex
      .captures_iter(line)
      .map(|elem| elem[0].parse::<i32>())
      .flatten()
      .collect::<Vec<_>>();

    // let raw_position = line.split('=').filter_map(|elem| elem.parse::<i32>().ok()).collect::<Vec<_>>();
    sensor_position.push((raw_position[0], raw_position[1]));
    beacon_position.push((raw_position[2], raw_position[3]));
  }

  // part1
  const LINE_INDEX: i32 = 10;
  // const LINE_INDEX: i32 = 2000000;
  let mut line_to_check = HashSet::new();
  let mut beacon_in_line = HashSet::new();
  for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
    let radius = manhattan(*sensor, *beacon);
    if LINE_INDEX - sensor.1 > radius {
      continue;
    }
    for x in sensor.0 - radius..sensor.0 + radius {
      if manhattan(*sensor, (x, LINE_INDEX)) <= radius {
        line_to_check.insert(x);
      }
      if beacon.1 == LINE_INDEX {
        beacon_in_line.insert(beacon.0);
      }
    }
  }
  let part1 = line_to_check.len() - beacon_in_line.len();

  // part2
  const SEARCH_DIM : usize = 20;
  // const SEARCH_DIM: usize = 4000000;
  let mut board = Board {
    data: vec![0; SEARCH_DIM * SEARCH_DIM as usize],
    width: SEARCH_DIM,
  };
  for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
    let radius = manhattan(*sensor, *beacon);
    for y in (sensor.1 - radius).max(0)..(sensor.1 + radius).min(SEARCH_DIM as i32) {
      let min_x = (sensor.0 - (radius - (sensor.1 - y).abs())).max(0) as usize;
      let max_x = (sensor.0 + (radius + 1 - (sensor.1 - y).abs())).min(SEARCH_DIM as i32) as usize;
      let min_offset = board.get_offset(min_x, y as usize);
      let max_offset = board.get_offset(max_x, y as usize);
      let slice = &mut board.data[min_offset..max_offset];
      slice.fill(1);
      // for x in (sensor.0 - (radius - (sensor.1 - y).abs())).max(0)..(sensor.0 + (radius + 1 - (sensor.1 - y).abs())).min(SEARCH_DIM as i32) {
      //   // if board.get(&(x as usize, y as usize)) == 1 {
      //   //   continue;
      //   // }
      //   *board.get_mut(x as usize, y as usize) = 1;
      // }
    }
  }

  let index = board
    .data
    .iter()
    .enumerate()
    .filter_map(|(size, elem)| if *elem == 0 { Some(size) } else { None })
    .next()
    .ok_or("No free space in range")?;
  let part2 = index % SEARCH_DIM * 4000000 + index / SEARCH_DIM;

  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day15,        "data/day15.txt",       [5525990, 430];
    test1:  day15,        "data/day15_test1.txt", [26, 56000011];
  );
}
