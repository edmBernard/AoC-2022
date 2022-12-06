#![allow(unused_variables)]
use regex::Regex;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

enum State {
  ParseHeader,
  InterpreteHeader,
  ParseMovement,
}

pub fn day06(filename: &Path) -> Result<ReturnType> {
  let file_content = std::fs::read_to_string(filename)?;
  let content_iterator = file_content.lines();
  let mut header = Vec::new();
  let mut bin_size = 0;
  let mut state = State::ParseHeader;
  let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
  let mut board_part1: Vec<Vec<char>> = Vec::new();
  let mut board_part2: Vec<Vec<char>> = Vec::new();
  // Use a state machine because I was not able to make iterator continue during hot time
  // but lucky enough the switch between state allow to skip the empty line between header and movement
  for line in content_iterator {
    match state {
      State::ParseHeader => {
        // Parse header
        if line.chars().nth(1).unwrap_or('0') == '1' {
          bin_size = line
            .chars()
            .rev()
            .nth(0)
            .ok_or("Empty line")?
            .to_digit(10)
            .ok_or("Fail to parse digit")?;
          state = State::InterpreteHeader;
        } else {
          header.push(line);
        }
      }
      State::InterpreteHeader => {
        // Interprete header
        board_part1 = vec![Vec::new(); bin_size as usize];
        for header_line in header.iter().rev() {
          for (index, value) in header_line.chars().skip(1).step_by(4).enumerate() {
            if value != ' ' {
              board_part1[index].push(value);
            }
          }
        }
        board_part2 = board_part1.clone();
        state = State::ParseMovement;
      }
      State::ParseMovement => {
        // Parse and apply movement
        let cap = regex.captures(line).ok_or("Fail to capture")?;
        let quantity = cap.get(1).ok_or("Fail to capture 1")?.as_str().parse::<u64>()?;
        let src = cap.get(2).ok_or("Fail to capture 2")?.as_str().parse::<u64>()? - 1;
        let dst = cap.get(3).ok_or("Fail to capture 3")?.as_str().parse::<u64>()? - 1;
        let mut temp_part1 = Vec::new();
        let mut temp_part2 = Vec::new();
        for _ in 0..quantity {
          temp_part1.push(board_part1[src as usize].pop().ok_or("No enough value to pop")?);
          temp_part2.push(board_part2[src as usize].pop().ok_or("No enough value to pop")?);
        }
        for elem in temp_part1 {
          board_part1[dst as usize].push(elem);
        }
        for elem in temp_part2.into_iter().rev() {
          board_part2[dst as usize].push(elem);
        }
      }
    }
  }
  let part1 = board_part1
    .iter()
    .map(|stack| String::from(stack[stack.len() - 1]))
    .collect::<Vec<_>>()
    .join("");
  let part2 = board_part2
    .iter()
    .map(|stack| String::from(stack[stack.len() - 1]))
    .collect::<Vec<_>>()
    .join("");
  Ok(ReturnType::String(part1, part2))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day06,        "data/day06.txt",              ["ZWHVFWQWW", "HZFZCCWWV"];
    test1:  day06,        "data/day06_test1.txt",        ["CMZ", "MCD"];
  );
}
