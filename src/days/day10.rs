// #![allow(unused_variables)]
use std::collections::HashMap;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Type {
  Directory,
  File,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Entry {
  kind: Type,
  size: usize,
  parent: Option<usize>,
}

pub fn day10(filename: &Path) -> Result<ReturnType> {
  let mut register_value: Vec<i32> = Vec::new();
  register_value.reserve(220);
  let mut current_value = 1;
  register_value.push(current_value);
  for line in std::fs::read_to_string(filename)?.lines() {
    let mut full_command = line.split(" ");
    // Check command
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
  for (idx, value) in register_value.iter().enumerate() {
    println!("{} -> {}", idx, value);
  }
  let mut part1 = 0;
  for idx in (20..=220).step_by(40) {
    part1 += register_value[idx-1] * idx as i32;
    println!("{} -> {}", idx, register_value[idx-1]);

  }
  Ok(ReturnType::Numeric(part1 as u64, 2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day10,        "data/day10.txt",              [11720, 1117448];
    test1:  day10,        "data/day10_test1.txt",        [13140, 24933642];
  );
}
