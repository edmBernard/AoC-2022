// #![allow(unused_variables)]

use std::path::Path;

use crate::Result;
use crate::utils::ReturnType;

pub fn day11(filename: &Path) -> Result<ReturnType> {

  Ok(ReturnType::Numeric(1, 2))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day11,        "data/day11.txt",       [1, 2];
    test1:  day11,        "data/day11_test1.txt", [1, 2];
  );
}
