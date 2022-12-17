// #![allow(unused_variables)]
// #![allow(unused_imports)]

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
    &mut self.data[x + y * self.width]
  }
  fn get_height(&self) -> usize {
    self.data.len() / self.width
  }
}

impl std::fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    println!("size:({}, {})", self.width, self.get_height());
    for h in (0..(self.data.len() / self.width)).rev() {
      write!(f, "\n")?;
      for w in 0..self.width {
        write!(f, "{} ", self.get(&(w, h)))?
      }
    }
    Ok(())
  }
}


pub fn day17(filename: &Path) -> Result<ReturnType> {
  let mut movements = Vec::new();
  for line in std::fs::read_to_string(filename)?.lines() {
    movements = line.chars().collect::<Vec<_>>();
  }

  // Fill Board Part1
  let width = 7;
  let height = 10000;
  let mut board_part1 = Board {
    data: vec!['.'; width * height],
    width: width,
  };

  // println!("{}", board_part1);

  // Run part1 simulation
  // launch rocks
  let part1 = {
    let mut count = 0;
    let mut start_offset = (2, 4);
    for i in 0..2022 {
      let mut rocks = match i % 5 {
        0 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        1 => vec![(1, 2), (1, 0), (0, 1), (1, 1), (2, 1)],
        2 => vec![(2, 2), (2, 1), (2, 0), (1, 0), (0, 0)],
        3 => vec![(0, 3), (0, 2), (0, 1), (0, 0)],
        4 => vec![(0, 1), (1, 1), (1, 0), (0, 0)],
        _ => unreachable!(),
      };

      rocks.iter_mut().for_each(|(x, y)| {
        *x += start_offset.0;
        *y += start_offset.1;
      });

      loop {
        // fall
        rocks.iter_mut().for_each(|e| e.1 -= 1);

        // push
        match movements[count] {
          '<' => {
            if rocks
              .iter()
              .all(|(x, y)| *x > 0 && board_part1.get(&(x - 1, *y)) != '#')
            {
              rocks.iter_mut().for_each(|(x, _y)| *x -= 1);
            }
          }
          '>' => {
            if rocks
              .iter()
              .all(|(x, y)| *x < 6 && board_part1.get(&(x + 1, *y)) != '#')
            {
              rocks.iter_mut().for_each(|(x, _y)| *x += 1);
            }
          }
          _ => unreachable!(),
        }
        count = if count > movements.len() - 2 {
          0
        } else {
          count + 1
        };

        // stabilize
        let stabilized = 'stabilize_ckeck: {
          for rock in &rocks {
            if rock.1 == 0 || board_part1.get(&(rock.0, rock.1 - 1)) == '#' {
              break 'stabilize_ckeck true;
            }
          }
          false
        };

        if stabilized {
          for rock in &rocks {
            *board_part1.get_mut(rock.0, rock.1) = '#';
          }
          start_offset.1 = *rocks.iter().map(|(_x, y)| y).max().unwrap() + 5;
          break;
        }
      }
    }
    // println!("{}", board_part1);
    start_offset.1 - 5
  };

  Ok(ReturnType::Numeric(part1 as u64, 2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day17,        "data/day17.txt",       [1003, 25771];
    test1:  day17,        "data/day17_test1.txt", [24, 93];
  );
}
