// #![allow(unused_variables)]

use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

#[derive(Debug, Clone)]
struct Board {
  data: Vec<char>,
  width: usize,
}

impl Board {
  fn get(&self, pos: &(usize, usize)) -> char {
    self.data[pos.0 + pos.1 * self.width]
  }
  fn get_mut(&mut self, x: usize, y: usize) -> &mut char {
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

pub fn day14(filename: &Path) -> Result<ReturnType> {
  let mut rock_shapes = Vec::new();
  for line in std::fs::read_to_string(filename)?.lines() {
    rock_shapes.push(Vec::new());
    for point_str in line.split("->") {
      let mut point_split = point_str.split(",");
      let x = point_split.next().ok_or("Missing x for point")?.trim().parse::<i32>()?;
      let y = point_split.next().ok_or("Missing y for point")?.trim().parse::<i32>()?;
      let length = rock_shapes.len();
      rock_shapes[length - 1].push((x, y));
    }
  }
  let (min_x, max_x) = match rock_shapes.iter().flatten().map(|elem| elem.0).minmax() {
    NoElements => panic!("NoMinMax"),
    OneElement(x) => (x, x),
    MinMax(x, y) => (x, y),
  };
  let (min_y, max_y) = match rock_shapes.iter().flatten().map(|elem| elem.1).minmax() {
    NoElements => panic!("NoMinMax"),
    OneElement(x) => (x, x),
    MinMax(x, y) => (x, y),
  };

  let mut board = Board {
    data: vec!['.'; ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize],
    width: (max_x - min_x + 1) as usize,
  };

  for rock_shape in &rock_shapes {
    for ((x1, y1), (x2, y2)) in rock_shape.iter().tuple_windows::<(_, _)>() {
      let x_min = x1.min(x2);
      let y_min = y1.min(y2);

      let x_max = x1.max(x2);
      let y_max = y1.max(y2);

      let step_x = (x_max - x_min).min(1);
      let step_y = (y_max - y_min).min(1);
      if step_x == 0 {
        for y in (*y_min..*y_max).step_by(step_y as usize) {
          *board.get_mut((*x_min - min_x) as usize, (y - min_y) as usize) = '#';
        }
      }

      if step_y == 0 {
        for x in (*x_min..*x_max).step_by(step_x as usize) {
          *board.get_mut((x - min_x) as usize, (*y_min - min_y) as usize) = '#';
        }
      }
    }
  }
  for x in min_x..=max_x {
    for y in min_y..=max_y {}
  }
  println!("{}", board);

  let mut part1 = 0;
  let mut part2 = 0;
  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day14,        "data/day14.txt",       [437, 430];
    test1:  day14,        "data/day14_test1.txt", [31, 29];
  );
}
