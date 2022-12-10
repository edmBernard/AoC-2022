// #![allow(unused_variables)]
use std::path::Path;

use itertools::Itertools;

use crate::utils::ReturnType;
use crate::Result;

pub fn day10(filename: &Path) -> Result<ReturnType> {
  let mut register_value: Vec<i32> = Vec::new();
  const CRT_WIDTH: usize = 40;
  const CRT_HEIGHT: usize = 6;
  register_value.reserve(CRT_WIDTH * CRT_HEIGHT);
  let mut current_value = 1;
  register_value.push(current_value);  // to compensate zero indexing of array
  register_value.push(current_value);  // offset to compensate the fact the register is set at the end of cycle

  for line in std::fs::read_to_string(filename)?.lines() {
    let mut full_command = line.split(" ");
    let command = full_command.next().ok_or("Empty Line Found")?;
    match command {
      "noop" => register_value.push(current_value),
      "addx" => {
        register_value.push(current_value);
        current_value += full_command
        .next()
        .ok_or("No Increment found in addx command")?
        .parse::<i32>()?;
        register_value.push(current_value);
      }
      _ => Err("Unknown command")?,
    }
  }

  let mut screen: Vec<char> = vec!['.'; CRT_WIDTH * CRT_HEIGHT];
  let mut part1 = 0;
  for cycle in 1..screen.len() {
    if (20..=220).step_by(40).contains(&cycle) {
      part1 += register_value[cycle] * cycle as i32;
    }
    if ((cycle-1).rem_euclid(40) as i32 - register_value[cycle]).abs() <= 1 {
      screen[cycle-1] = '#';
    }
  }
  // for row in 0..CRT_HEIGHT {
  //   for col in 0..CRT_WIDTH {
  //     print!("{}", screen[col + row * CRT_WIDTH]);
  //   }
  //   println!("");
  // }
  // Visual solution
  // ####.###...##..###..####.###...##....##.
  // #....#..#.#..#.#..#.#....#..#.#..#....#.
  // ###..#..#.#....#..#.###..#..#.#.......#.
  // #....###..#....###..#....###..#.......#.
  // #....#.#..#..#.#.#..#....#....#..#.#..#.
  // ####.#..#..##..#..#.####.#.....##...##..
  Ok(ReturnType::Numeric(part1 as u64, 2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day10,        "data/day10.txt",              [11720, 2];
    test1:  day10,        "data/day10_test1.txt",        [13140, 2];
  );
}
