// #![allow(unused_variables)]

use regex::Regex;
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

    let sensor = (raw_position[0], raw_position[1]);
    let beacon = (raw_position[2], raw_position[3]);
    sensor_position.push((sensor, manhattan(sensor, beacon)));
    beacon_position.push(beacon);
  }

  // Dirty switch as test and regular input don't have same condition
  let line_index: i32 = if filename.to_str().unwrap().contains("test") { 10 } else { 2000000 };
  let search_dim: i32 = if filename.to_str().unwrap().contains("test") { 20 } else { 4000000 };

  // part1
  let mut line_to_check = Vec::new();
  let mut beacon_in_line = Vec::new();

  for ((sensor, radius), beacon) in zip(&sensor_position, &beacon_position) {
    if line_index - sensor.1 > *radius {
      continue;
    }
    for x in sensor.0 - radius..sensor.0 + radius {
      if manhattan(*sensor, (x, line_index)) <= *radius {
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
  let mut frontier_point = Vec::new();
  for (sensor, radius) in &sensor_position {
    for y in (sensor.1 - radius - 1).max(0)..(sensor.1 + radius + 1).min(search_dim) {
      let min_x = (sensor.0 - (radius + 1 - (sensor.1 - y).abs())).max(0);
      let max_x = (sensor.0 + (radius + 1 - (sensor.1 - y).abs())).min(search_dim);
      frontier_point.push((min_x, y));
      frontier_point.push((max_x, y));
    }
  }
  let part2 = 'block: {
    for (x, y) in frontier_point {
      let mut current = 0;
      for (sensor, radius) in &sensor_position {
        if manhattan(*sensor, (x, y)) <= *radius {
          current += 1;
          break;
        }
      }
      if current == 0 {
        break 'block x as u64 * 4000000 + y as u64;
      }
    }
    0
  };
  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

/// Given two ranges return 1 or two non overlaping range
fn merge_range(range1:(i32, i32), range2:(i32, i32)) -> ((i32, i32), (i32, i32))
{
  //   range1     0-------1
  //   range2              0-----------1
  if range1.1 < range2.0 - 1 || range2.1 < range1.0 - 1 {
    (range1, range2)
  } else {
    ((range1.0.min(range2.0), range1.1.max(range2.1)), (0, 0))
  }
}

pub fn day15_only_range(filename: &Path) -> Result<ReturnType> {
  let mut beacon_position = Vec::new();
  let mut sensor_position = Vec::new();
  let regex = Regex::new(r"(-?\d+)")?;
  for line in std::fs::read_to_string(filename)?.lines() {
    let raw_position = regex
      .captures_iter(line)
      .map(|elem| elem[0].parse::<i32>())
      .flatten()
      .collect::<Vec<_>>();

    let sensor = (raw_position[0], raw_position[1]);
    let beacon = (raw_position[2], raw_position[3]);
    sensor_position.push((sensor, manhattan(sensor, beacon)));
    beacon_position.push(beacon);
  }

  // Dirty switch as test and regular input don't have same condition
  let line_index: i32 = if filename.to_str().unwrap().contains("test") { 10 } else { 2000000 };
  let search_dim: i32 = if filename.to_str().unwrap().contains("test") { 20 } else { 4000000 };

  // part1
  let mut ranges = Vec::new();
  let mut beacon_in_line = Vec::new();
  for ((sensor, radius), beacon) in zip(&sensor_position, &beacon_position) {
    if (line_index - sensor.1).abs() > *radius {
      continue;
    }
    if beacon.1 == line_index {
      beacon_in_line.push(beacon.0);
    }
    let min_x = sensor.0 - (radius - (sensor.1 - line_index).abs()).abs();
    let max_x = sensor.0 + (radius - (sensor.1 - line_index).abs()).abs();
    assert!(min_x <= max_x);
    ranges.push((min_x, max_x));
  }
  // Merge stored range directly
  for _loop in 0..2 {
    for idx1 in 0..ranges.len()-1 {
      for idx2 in idx1+1..ranges.len() {
        let (range1, range2) = merge_range(ranges[idx1], ranges[idx2]);
        ranges[idx1] = range1;
        ranges[idx2] = range2;
      }
    }
  }
  beacon_in_line.sort();
  beacon_in_line.dedup();
  let part1 = (ranges[0].1 - ranges[0].0 + 1) as u64 - beacon_in_line.len() as u64;

  // part2.2
  let part2 = 'block: {
    for y in 0..search_dim {
      let mut ranges = Vec::new();
      for (sensor, radius) in &sensor_position {
        if (y - sensor.1).abs() > *radius {
          continue;
        }
        let min_x = sensor.0 - (radius - (sensor.1 - y).abs()).abs();
        let max_x = sensor.0 + (radius - (sensor.1 - y).abs()).abs();
        assert!(min_x <= max_x);
        let mut new_range = (min_x, max_x);
        for idx1 in 0..ranges.len() {
          let (range1, range2) = merge_range(ranges[idx1], new_range);
          ranges[idx1] = range1;
          new_range = range2;
        }
        if new_range != (0, 0) {
          ranges.push(new_range);
        }
      }
      // Merge stored range directly
      // Even if I merge range on the fly I still need this  second pass
      for _loop in 0..2 {
        for idx2 in 1..ranges.len() {
          let (range1, range2) = merge_range(ranges[0], ranges[idx2]);
          ranges[0] = range1;
          ranges[idx2] = range2;
        }
      }
      let part1 = ranges[0].1.min(search_dim-1) - ranges[0].0.max(0) + 1;
      if part1 != search_dim {
        break 'block (ranges[0].1 + 1) as u64 * 4000000 + y as u64;
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

    let sensor = (raw_position[0], raw_position[1]);
    let beacon = (raw_position[2], raw_position[3]);
    sensor_position.push((sensor, manhattan(sensor, beacon)));
    beacon_position.push(beacon);
  }

  // Dirty switch as test and regular input don't have same condition
  let line_index: i32 = if filename.to_str().unwrap().contains("test") { 10 } else { 2000000 };
  let search_dim: i32 = if filename.to_str().unwrap().contains("test") { 20 } else { 4000000 };

  // part1
  // For part1 we directly merge range
  let mut ranges = Vec::new();
  let mut beacon_in_line = Vec::new();
  for ((sensor, radius), beacon) in zip(&sensor_position, &beacon_position) {
    if (line_index - sensor.1).abs() > *radius {
      continue;
    }
    if beacon.1 == line_index {
      beacon_in_line.push(beacon.0);
    }
    let min_x = sensor.0 - (radius - (sensor.1 - line_index).abs()).abs();
    let max_x = sensor.0 + (radius - (sensor.1 - line_index).abs()).abs();
    assert!(min_x <= max_x);
    ranges.push((min_x, max_x));
  }
  // Merge stored range directly
  // We have to do several iteration to be sure we merge all possible case
  for _loop in 0..2 {
    for idx2 in 1..ranges.len() {
      let (range1, range2) = merge_range(ranges[0], ranges[idx2]);
      ranges[0] = range1;
      ranges[idx2] = range2;
    }
  }
  beacon_in_line.sort();
  beacon_in_line.dedup();
  let part1 = (ranges[0].1 - ranges[0].0 + 1) as u64 - beacon_in_line.len() as u64;

  // part2
  // for each sensor
  //   for each pixel on the border of the sensor
  //     for each sensor
  //       if the pixel in radius
  //          check
  //     if the pixel is not in range of any sensor
  //        we have the result
  let part2 = 'block: {
    for (sensor, radius) in &sensor_position {
      for y in (sensor.1 - radius - 1).max(0)..(sensor.1 + radius + 1).min(search_dim) {
        let min_x = (sensor.0 - (radius + 1 - (sensor.1 - y).abs())).max(0);
        let max_x = (sensor.0 + (radius + 1 - (sensor.1 - y).abs())).min(search_dim);
        // min_x
        let mut current = 0;
        for (sensor, radius) in &sensor_position {
          if manhattan(*sensor, (min_x, y)) <= *radius {
            current += 1;
            break;
          }
        }
        if current == 0 {
          break 'block min_x as u64 * 4000000 + y as u64;
        }
        // max_x
        let mut current = 0;
        for (sensor, radius) in &sensor_position {
          if manhattan(*sensor, (max_x, y)) <= *radius {
            current += 1;
            break;
          }
        }
        if current == 0 {
          break 'block max_x as u64 * 4000000 + y as u64;
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
    main:   day15,            "data/day15.txt",       [5525990, 11756174628223];
    test1:  day15,            "data/day15_test1.txt", [26, 56000011];
    main:   day15_speed,      "data/day15.txt",       [5525990, 11756174628223];
    test1:  day15_speed,      "data/day15_test1.txt", [26, 56000011];
    main:   day15_only_range, "data/day15.txt",       [5525990, 11756174628223];
    test1:  day15_only_range, "data/day15_test1.txt", [26, 56000011];
  );
}
