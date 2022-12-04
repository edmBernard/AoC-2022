// #![allow(unused_variables)]
use std::path::Path;

use crate::Result;

pub fn day04(filename: &Path) -> Result<[u64; 2]> {
  let file_content = std::fs::read_to_string(filename)?;
  let part1 = file_content
    .lines()
    .map(|line| {
      let indexes: Vec<&str> = line.split(&['-', ',']).collect();
      let min_efl1 = indexes[0].parse::<u32>().unwrap_or(0);
      let max_efl1 = indexes[1].parse::<u32>().unwrap_or(0);
      let min_efl2 = indexes[2].parse::<u32>().unwrap_or(0);
      let max_efl2 = indexes[3].parse::<u32>().unwrap_or(0);
      if min_efl1 <= min_efl2 && max_efl1 >= max_efl2 {
        1
      } else if min_efl2 <= min_efl1 && max_efl2 >= max_efl1 {
        1
      } else {
        0
      }
    })
    .sum();
  let part2 = file_content
    .lines()
    .map(|line| {
      let indexes: Vec<&str> = line.split(&['-', ',']).collect();
      let min_efl1 = indexes[0].parse::<u32>().unwrap_or(0);
      let max_efl1 = indexes[1].parse::<u32>().unwrap_or(0);
      let min_efl2 = indexes[2].parse::<u32>().unwrap_or(0);
      let max_efl2 = indexes[3].parse::<u32>().unwrap_or(0);
      if min_efl1 <= min_efl2 && max_efl1 >= min_efl2 {
        1
      } else if min_efl2 <= min_efl1 && max_efl2 >= min_efl1 {
        1
      } else {
        0
      }
    })
    .sum();

  Ok([part1, part2])
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day04, "data/day04.txt",           [490, 921];
    test1:  day04, "data/day04_test1.txt",     [2, 4];
  );
}
