#![allow(unused_variables)]
use std::path::Path;
use regex::Regex;

use crate::Result;

enum State {
  ParseHeader,
  InterpreteHeader,
  ParseMovement
}

pub fn day05(filename: &Path) -> Result<[u64; 2]> {
  let file_content = std::fs::read_to_string(filename)?;
  let content_iterator = file_content.lines();
  let mut header = Vec::new();
  let mut bin_size = 0;
  let mut state = State::ParseHeader;
  let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
  let mut board : Vec<Vec<char>> = Vec::new();
  // Ugly state machine because I don't find a way to continue the content_iterator in another loop
  for line in content_iterator {
    match state {
      State::ParseHeader => {
        // Parse header
        if line.chars().nth(1).unwrap_or('0') == '1' {
          bin_size = line.chars().rev().nth(0).ok_or("Empty line")?.to_digit(10).ok_or("Fail to parse digit")?;
          state = State::InterpreteHeader;
        } else {
          header.push(line);
        }
      }
      State::InterpreteHeader => {
        // Interprete header
        board = vec![Vec::new(); bin_size as usize];
        for header_line in header.iter().rev() {
          for (index, value) in header_line.chars().skip(1).step_by(4).enumerate() {
            if value != ' ' {
              board[index].push(value);
            }
          }
        }
        state = State::ParseMovement;
      }
      State::ParseMovement => {
        let cap = regex.captures(line).ok_or("Fail to capture")?;
        let quantity = cap.get(1).ok_or("Fail to capture 1")?.as_str().parse::<u64>()?;
        let src = cap.get(2).ok_or("Fail to capture 2")?.as_str().parse::<u64>()? - 1;
        let dst = cap.get(3).ok_or("Fail to capture 3")?.as_str().parse::<u64>()? - 1;
        let mut temp = Vec::new();
        for _ in 0..quantity {
          temp.push(board[src as usize].pop().ok_or("No enough value to pop")?);
        }
        for elem in temp {
          board[dst as usize].push(elem);
        }
      }
    }
  }
  let part1 = board.iter().map(|stack| String::from(stack[stack.len()-1])).collect::<Vec<_>>().join("");
  // Result of part1 is a string
  // println!("part1 {}", part1);
  Ok([0, 0])
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day05, "data/day05.txt",              [490, 921];
    test1:  day05, "data/day05_test1.txt",        [2, 4];
  );
}
