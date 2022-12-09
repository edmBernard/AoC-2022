// #![allow(unused_variables)]
use std::collections::HashSet;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

#[derive(Debug, Clone)]
struct Point {
  x: i32,
  y: i32
}

impl Point {
  fn distance(&self, pt2: &Point) -> i32 {
    (pt2.x - self.x).abs().max((pt2.y - self.y).abs())
  }
}

pub fn day09(filename: &Path) -> Result<ReturnType> {

  let mut tail_visited_position_part1: HashSet<(i32, i32)> = HashSet::new();
  let mut tail_visited_position_part2: HashSet<(i32, i32)> = HashSet::new();
  let mut rope = vec![Point{x:0, y:0}; 10];

  for line in std::fs::read_to_string(filename)?.lines() {
    let mut line_splitted = line.split(" ");
    let direction = line_splitted.next().unwrap();
    let quantity = line_splitted.next().unwrap().parse::<u32>()?;
    for _ in 0..quantity {
      let mut head = &mut rope[0];
      match direction {
        "U" => {
          head.y -= 1;
        },
        "D" => {
          head.y += 1;
        },
        "L" => {
          head.x -= 1;
        },
        "R" => {
          head.x += 1;
        },
        _ => panic!("Unsupported character {}", direction)
      }

      for index in 1..rope.len() {
        let (rope1, rope2) = rope.split_at_mut(index);
        let head = &rope1[index-1];
        let mut tail = &mut rope2[0];
        if tail.distance(&head) > 1 {
          tail.x += match head.x.cmp(&tail.x) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1
          };
          tail.y += match head.y.cmp(&tail.y) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1
          };
        }
      }
      tail_visited_position_part1.insert((rope[1].x, rope[1].y));
      tail_visited_position_part2.insert((rope[9].x, rope[9].y));
    }
  }
  let part1 = tail_visited_position_part1.len();
  let part2 = tail_visited_position_part2.len();
  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day09,        "data/day09.txt",              [6503, 2724];
    test1:  day09,        "data/day09_test1.txt",        [88, 36];
  );
}
