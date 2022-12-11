// #![allow(unused_variables)]
use regex::Regex;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;
// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
// for function member check : https://stackoverflow.com/a/52934680
//   - can't use : Fn(usize) -> usize because each monkey will have different lambda
//   - can't use : fn(usize) -> usize because I need to capture
//   - can't use : &'a dyn Fn(usize) -> usize because the lambda don't live long enough
struct Monkey {
  operation: Box<dyn Fn(u64) -> u64>,
  div_test: u64,
  monkey_if_true: usize,
  monkey_if_false: usize,
}

/// like split_at_mut but we give 3 elems instead of 2 slice
fn take3_at_mut(
  values: &mut [Vec<u64>],
  index1: usize,
  index2: usize,
  index3: usize,
) -> (&mut Vec<u64>, &mut Vec<u64>, &mut Vec<u64>) {
  let len = values.len();
  let ptr = values.as_mut_ptr();

  assert!(index1 <= len);
  assert!(index2 <= len);
  assert!(index3 <= len);
  assert!(index1 != index2);
  assert!(index1 != index3);
  assert!(index2 != index3);

  unsafe { (&mut *ptr.add(index1), &mut *ptr.add(index2), &mut *ptr.add(index3)) }
}

pub fn day11(filename: &Path) -> Result<ReturnType> {
  let mut monkeys: Vec<Monkey> = Vec::new();
  let mut ppcm = 1;
  let content = std::fs::read_to_string(filename)?;
  let lines = &mut content.lines();

  // Monkey 0:
  let line_header = Regex::new(r"Monkey \d:")?;
  //   Starting items: 79, 98
  let line_items = Regex::new(r"((,? \d+)+)")?;
  //   Operation: new = old * 19
  let line_operation = Regex::new(r"new = old ([+\-*]) (old|\d+)")?;
  //   Test: divisible by 23
  //     If true: throw to monkey 2
  //     If false: throw to monkey 3
  let line_test = Regex::new(r"(\d+)")?;

  let mut items_part1 = Vec::new();
  while let Some(line) = lines.next() {
    if !line_header.is_match(line) {
      continue;
    }
    // Parse items
    let line = lines.next().ok_or("Missing items line")?;
    let cap = line_items.captures(line).ok_or("Fail to capture")?;
    let items = cap
      .get(1)
      .ok_or("Fail to capture items")?
      .as_str()
      .trim()
      .split(", ")
      .map(|elem| elem.parse::<u64>())
      .flatten()
      .collect::<Vec<_>>();
    items_part1.push(items);

    // Parse operation
    let line = lines.next().ok_or("Missing operation line")?;
    let cap = line_operation.captures(line).ok_or("Fail to capture operation")?;
    let operation: Box<dyn Fn(u64, u64) -> u64> = match cap.get(1).ok_or("Fail to capture operation")?.as_str() {
      "*" => Box::new(|a, b| a * b),
      "+" => Box::new(|a, b| a + b),
      "-" => Box::new(|a, b| a - b),
      _ => panic!("Unsupported operation"),
    };
    let second_term = cap.get(2).ok_or("Fail to capture second terme")?.as_str();
    let lambda: Box<dyn Fn(u64) -> u64> = match second_term {
      "old" => Box::new(move |a| operation(a, a)),
      term => {
        let number = term.parse::<u64>()?;
        Box::new(move |a| operation(a, number))
      }
    };
    // Parse test
    let line = lines.next().ok_or("Missing test line")?;
    let cap = line_test.captures(line).ok_or("Fail to capture test")?;
    let div_test = cap.get(1).ok_or("Fail to capture test")?.as_str().parse::<u64>()?;
    ppcm *= div_test;

    let line = lines.next().ok_or("Missing true line")?;
    let cap = line_test.captures(line).ok_or("Fail to capture test true")?;
    let monkey_if_true = cap.get(1).ok_or("Fail to capture test")?.as_str().parse::<usize>()?;

    let line = lines.next().ok_or("Missing false line")?;
    let cap = line_test.captures(line).ok_or("Fail to capture test false")?;
    let monkey_if_false = cap.get(1).ok_or("Fail to capture test")?.as_str().parse::<usize>()?;

    monkeys.push(Monkey {
      operation: lambda,
      div_test,
      monkey_if_true,
      monkey_if_false,
    })
  }
  let mut items_part2 = items_part1.clone();

  let mut monkey_inspection = vec![0; monkeys.len()];
  for _round in 0..20 {
    for idx in 0..monkeys.len() {
      let current_items = items_part1[idx].clone();
      // the order in which we read item don't matter
      // either current_items or current_items.iter().rev() give the same result
      for item in current_items {
        monkey_inspection[idx] += 1;
        let worry_level = (monkeys[idx].operation)(item);
        let after_bored = worry_level / 3;
        if after_bored % monkeys[idx].div_test == 0 {
          let monkey_index = monkeys[idx].monkey_if_true;
          items_part1[monkey_index].push(after_bored);
        } else {
          let monkey_index = monkeys[idx].monkey_if_false;
          items_part1[monkey_index].push(after_bored);
        }
      }
      items_part1[idx].clear();
    }
  }
  monkey_inspection.sort();
  monkey_inspection.reverse();
  let part1 = monkey_inspection[0] * monkey_inspection[1];

  let mut monkey_inspection = vec![0; monkeys.len()];
  for _round in 0..10000 {
    for idx in 0..monkeys.len() {
      // take3_at_mut is two time faster than the trick we need to work around it
      // the part1 still use the trick
      let (current, if_true, if_false) = take3_at_mut(
        &mut items_part2,
        idx,
        monkeys[idx].monkey_if_true,
        monkeys[idx].monkey_if_false,
      );
      for item in current {
        monkey_inspection[idx] += 1;
        let worry_level = (monkeys[idx].operation)(*item);
        let after_bored = worry_level % ppcm;
        if after_bored % monkeys[idx].div_test == 0 {
          if_true.push(after_bored);
        } else {
          if_false.push(after_bored);
        }
      }
      items_part2[idx].clear();
    }
  }
  monkey_inspection.sort();
  monkey_inspection.reverse();
  let part2 = monkey_inspection[0] * monkey_inspection[1];

  Ok(ReturnType::Numeric(part1, part2))
}

enum Operation {
  Mul,
  Add,
  Square,
}

struct MonkeySpeed {
  operation: Operation,
  operand: u64,
  div_test: u64,
  monkey_if_true: usize,
  monkey_if_false: usize,
}

pub fn day11_speed(filename: &Path) -> Result<ReturnType> {
  let mut monkeys: Vec<MonkeySpeed> = Vec::new();
  let mut ppcm = 1;
  let content = std::fs::read_to_string(filename)?;
  let lines = &mut content.lines();

  // Monkey 0:
  let line_header = Regex::new(r"Monkey \d:")?;
  //   Starting items: 79, 98
  let line_items = Regex::new(r"((,? \d+)+)")?;
  //   Operation: new = old * 19
  let line_operation = Regex::new(r"new = old ([+\-*]) (old|\d+)")?;
  //   Test: divisible by 23
  //     If true: throw to monkey 2
  //     If false: throw to monkey 3
  let line_test = Regex::new(r"(\d+)")?;

  let mut items_part1 = Vec::new();
  while let Some(line) = lines.next() {
    if !line_header.is_match(line) {
      continue;
    }
    // Parse items
    let line = lines.next().ok_or("Missing items line")?;
    let cap = line_items.captures(line).ok_or("Fail to capture")?;
    let items = cap
      .get(1)
      .ok_or("Fail to capture items")?
      .as_str()
      .trim()
      .split(", ")
      .map(|elem| elem.parse::<u64>())
      .flatten()
      .collect::<Vec<_>>();
    items_part1.push(items);

    // Parse operation
    let line = lines.next().ok_or("Missing operation line")?;
    let cap = line_operation.captures(line).ok_or("Fail to capture operation")?;
    let operation_str = cap.get(1).ok_or("Fail to capture operation")?.as_str();
    let second_term_str = cap.get(2).ok_or("Fail to capture second terme")?.as_str();
    let (operation, operand): (Operation, u64) = match (operation_str, second_term_str) {
      ("*", "old") => (Operation::Square, 0),
      ("*", term) => (Operation::Mul, term.parse::<u64>()?),
      ("+", term) => (Operation::Add, term.parse::<u64>()?),
      _ => panic!("Unsupported operation"),
    };

    // Parse test
    let line = lines.next().ok_or("Missing test line")?;
    let cap = line_test.captures(line).ok_or("Fail to capture test")?;
    let div_test = cap.get(1).ok_or("Fail to capture test")?.as_str().parse::<u64>()?;
    ppcm *= div_test;

    let line = lines.next().ok_or("Missing true line")?;
    let cap = line_test.captures(line).ok_or("Fail to capture test true")?;
    let monkey_if_true = cap.get(1).ok_or("Fail to capture test")?.as_str().parse::<usize>()?;

    let line = lines.next().ok_or("Missing false line")?;
    let cap = line_test.captures(line).ok_or("Fail to capture test false")?;
    let monkey_if_false = cap.get(1).ok_or("Fail to capture test")?.as_str().parse::<usize>()?;

    monkeys.push(MonkeySpeed {
      operation,
      operand,
      div_test,
      monkey_if_true,
      monkey_if_false,
    })
  }
  let mut items_part2 = items_part1.clone();

  let mut monkey_inspection = vec![0; monkeys.len()];
  for _round in 0..20 {
    for idx in 0..monkeys.len() {
      let current_items = items_part1[idx].clone();
      // the order in which we read item don't matter
      // either current_items or current_items.iter().rev() give the same result
      monkey_inspection[idx] += current_items.len();
      for item in current_items {
        let worry_level = match monkeys[idx].operation {
          Operation::Add => item + monkeys[idx].operand,
          Operation::Mul => item * monkeys[idx].operand,
          Operation::Square => item * item,
        };
        let after_bored = worry_level / 3;
        if after_bored % monkeys[idx].div_test == 0 {
          let monkey_index = monkeys[idx].monkey_if_true;
          items_part1[monkey_index].push(after_bored);
        } else {
          let monkey_index = monkeys[idx].monkey_if_false;
          items_part1[monkey_index].push(after_bored);
        }
      }
      items_part1[idx].clear();
    }
  }
  monkey_inspection.sort();
  monkey_inspection.reverse();
  let part1 = monkey_inspection[0] * monkey_inspection[1];

  let mut monkey_inspection = vec![0; monkeys.len()];
  for _round in 0..10000 {
    for idx in 0..monkeys.len() {
      // take3_at_mut is two time faster than the trick we need to work around it
      // the part1 still use the trick
      let (current, if_true, if_false) = take3_at_mut(
        &mut items_part2,
        idx,
        monkeys[idx].monkey_if_true,
        monkeys[idx].monkey_if_false,
      );
      monkey_inspection[idx] += current.len();
      for item in current {
        let worry_level = match monkeys[idx].operation {
          Operation::Add => *item + monkeys[idx].operand,
          Operation::Mul => *item * monkeys[idx].operand,
          Operation::Square => *item * *item,
        };
        let after_bored = worry_level % ppcm;
        if after_bored % monkeys[idx].div_test == 0 {
          if_true.push(after_bored);
        } else {
          if_false.push(after_bored);
        }
      }
      items_part2[idx].clear();
    }
  }
  monkey_inspection.sort();
  monkey_inspection.reverse();
  let part2 = monkey_inspection[0] * monkey_inspection[1];

  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day11,        "data/day11.txt",       [117624, 16792940265];
    test1:  day11,        "data/day11_test1.txt", [10605, 2713310158];
  );
}
