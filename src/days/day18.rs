// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::collections::HashSet;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

pub fn day18(filename: &Path) -> Result<ReturnType> {
  let mut cube_map = HashSet::new();
  let content =  std::fs::read_to_string(filename)?;
  for line in content.lines() {
    let coordinate = line.split(',').map(|e| e.parse::<i32>()).flatten().collect::<Vec<_>>();
    cube_map.insert((coordinate[0], coordinate[1], coordinate[2]));
  }

  println!("{:?}", cube_map);
  let mut part1 = 0;
  for (x, y, z) in &cube_map {
    let neighbor = [(x+1,*y,*z), (x-1,*y,*z), (*x,y+1,*z), (*x,y-1,*z), (*x,*y,z+1), (*x,*y,z-1)];
    part1 += neighbor.iter().filter(|&e| !cube_map.contains(e)).count();
  }
  Ok(ReturnType::Numeric(part1 as u64, 2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day18,        "data/day18.txt",       [3498, 3];
    test1:  day18,        "data/day18_test1.txt", [64, 3];
  );
}
