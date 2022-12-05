// #![allow(unused_variables)]
use std::path::Path;

use crate::Result;
use crate::utils::ReturnType;

pub fn day04(filename: &Path) -> Result<ReturnType> {
  let file_content = std::fs::read_to_string(filename)?;
  let part1 = file_content
    .lines()
    .map(|line| {
      let indexes: Vec<&str> = line.split(&['-', ',']).collect();
      let min_elf1 = indexes[0].parse::<u32>().unwrap_or(0);
      let max_elf1 = indexes[1].parse::<u32>().unwrap_or(0);
      let min_elf2 = indexes[2].parse::<u32>().unwrap_or(0);
      let max_elf2 = indexes[3].parse::<u32>().unwrap_or(0);
      if min_elf1 <= min_elf2 && max_elf1 >= max_elf2 {
        1
      } else if min_elf2 <= min_elf1 && max_elf2 >= max_elf1 {
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
      let min_elf1 = indexes[0].parse::<u32>().unwrap_or(0);
      let max_elf1 = indexes[1].parse::<u32>().unwrap_or(0);
      let min_elf2 = indexes[2].parse::<u32>().unwrap_or(0);
      let max_elf2 = indexes[3].parse::<u32>().unwrap_or(0);
      if min_elf1 <= min_elf2 && max_elf1 >= min_elf2 {
        1
      } else if min_elf2 <= min_elf1 && max_elf2 >= min_elf1 {
        1
      } else {
        0
      }
    })
    .sum();

  Ok(ReturnType::Numeric(part1, part2))
}

pub fn day04_speed(filename: &Path) -> Result<ReturnType> {
  let mut part1 = 0;
  let mut part2 = 0;
  for line in std::fs::read_to_string(filename)?.lines() {
    let mut iter = line.split(&['-', ',']);
    let min_elf1 = iter.next().expect("Not enough value").parse::<u64>()?;
    let max_elf1 = iter.next().expect("Not enough value").parse::<u64>()?;
    let min_elf2 = iter.next().expect("Not enough value").parse::<u64>()?;
    let max_elf2 = iter.next().expect("Not enough value").parse::<u64>()?;
    // part1
    part1 += if min_elf1 <= min_elf2 && max_elf1 >= max_elf2 {
      1
    } else if min_elf2 <= min_elf1 && max_elf2 >= max_elf1 {
      1
    } else {
      0
    };
    // part2
    part2 += if min_elf1 <= min_elf2 && max_elf1 >= min_elf2 {
      1
    } else if min_elf2 <= min_elf1 && max_elf2 >= min_elf1 {
      1
    } else {
      0
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
    main:   day04, "data/day04.txt",              [490, 921];
    test1:  day04, "data/day04_test1.txt",        [2, 4];
    main:   day04_speed, "data/day04.txt",        [490, 921];
    test1:  day04_speed, "data/day04_test1.txt",  [2, 4];
  );
}
