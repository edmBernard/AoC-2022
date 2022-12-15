// #![allow(unused_variables)]

use regex::Regex;
use std::collections::HashSet;
use std::iter::zip;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

#[inline(always)]
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

    sensor_position.push((raw_position[0], raw_position[1]));
    beacon_position.push((raw_position[2], raw_position[3]));
  }

  // Dirty switch as test and regular input don't have same condition
  let line_index: i32 = if filename.to_str().unwrap().contains("test") { 10 } else { 2000000 };
  let search_dim: usize = if filename.to_str().unwrap().contains("test") { 20 } else { 4000000 };

  // part1
  let mut line_to_check = HashSet::new();
  let mut beacon_in_line = HashSet::new();
  for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
    let radius = manhattan(*sensor, *beacon);
    if line_index - sensor.1 > radius {
      continue;
    }
    for x in sensor.0 - radius..sensor.0 + radius {
      if manhattan(*sensor, (x, line_index)) <= radius {
        line_to_check.insert(x);
      }
      if beacon.1 == line_index {
        beacon_in_line.insert(beacon.0);
      }
    }
  }
  let part1 = line_to_check.len() - beacon_in_line.len();

  // part2
  let mut frontier_point = Vec::new();
  for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
    let radius = manhattan(*sensor, *beacon);
    for y in (sensor.1 - radius - 1).max(0)..(sensor.1 + radius + 1).min(search_dim as i32) {
      let min_x = (sensor.0 - (radius + 1 - (sensor.1 - y).abs())).max(0) as usize;
      let max_x = (sensor.0 + (radius + 1 - (sensor.1 - y).abs())).min(search_dim as i32) as usize;
      frontier_point.push((min_x, y as usize));
      frontier_point.push((max_x, y as usize));
    }
  }
  let part2 = 'block: {
    for (x, y) in frontier_point {
      let mut current = 0;
      for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
        let radius = manhattan(*sensor, *beacon);
        if manhattan(*sensor, (x as i32, y as i32)) <= radius {
          current += 1;
          break;
        }
      }
      if current == 0 {
        break 'block x * 4000000 + y;
      }
    }
    0
  };
  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

pub fn day15_speed(filename: &Path) -> Result<ReturnType> {
  let mut beacon_position = Vec::new();
  let mut sensor_position = Vec::new();
  let regex = Regex::new(r"(-?\d+)")?;
  for line in std::fs::read_to_string(filename)?.lines() {
    let raw_position = regex
      .captures_iter(line)
      .map(|elem| elem[0].parse::<i32>())
      .flatten()
      .collect::<Vec<_>>();

    sensor_position.push((raw_position[0], raw_position[1]));
    beacon_position.push((raw_position[2], raw_position[3]));
  }

  // Dirty switch as test and regular input don't have same condition
  let line_index: i32 = if filename.to_str().unwrap().contains("test") { 10 } else { 2000000 };
  let search_dim: usize = if filename.to_str().unwrap().contains("test") { 20 } else { 4000000 };

  // part1
  let mut line_to_check = Vec::new();
  let mut beacon_in_line = Vec::new();
  for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
    let radius = manhattan(*sensor, *beacon);
    if line_index - sensor.1 > radius {
      continue;
    }
    for x in sensor.0 - radius..sensor.0 + radius {
      if manhattan(*sensor, (x, line_index)) <= radius {
        line_to_check.push(x);
      }
      if beacon.1 == line_index {
        beacon_in_line.push(beacon.0);
      }
    }
  }
  line_to_check.sort();
  line_to_check.dedup();
  beacon_in_line.sort();
  beacon_in_line.dedup();
  let part1 = line_to_check.len() - beacon_in_line.len();

  // part2
  let part2 = 'block: {
    for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
      let radius = manhattan(*sensor, *beacon);
      for y in (sensor.1 - radius - 1).max(0)..(sensor.1 + radius + 1).min(search_dim as i32) {
        let min_x = (sensor.0 - (radius + 1 - (sensor.1 - y).abs())).max(0) as usize;
        let max_x = (sensor.0 + (radius + 1 - (sensor.1 - y).abs())).min(search_dim as i32) as usize;
        // min_x
        let mut current = 0;
        for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
          let radius = manhattan(*sensor, *beacon);
          if manhattan(*sensor, (min_x as i32, y as i32)) <= radius {
            current += 1;
            break;
          }
        }
        if current == 0 {
          break 'block min_x * 4000000 + y as usize;
        }
        // max_x
        let mut current = 0;
        for (sensor, beacon) in zip(&sensor_position, &beacon_position) {
          let radius = manhattan(*sensor, *beacon);
          if manhattan(*sensor, (max_x as i32, y as i32)) <= radius {
            current += 1;
            break;
          }
        }
        if current == 0 {
          break 'block max_x * 4000000 + y as usize;
        }

      }
    }
    0
  };
  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day15,        "data/day15.txt",       [5525990, 11756174628223];
    test1:  day15,        "data/day15_test1.txt", [26, 56000011];
  );
}
