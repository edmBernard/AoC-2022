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

pub fn day07(filename: &Path) -> Result<ReturnType> {
  let mut tree_index: HashMap<String, usize> = HashMap::new();
  let mut tree_content: Vec<Entry> = Vec::new();

  // I can't make the HashMap<String, Option<Entry>> work on the second
  // iteration where we compute size because we borrow the key and modify value
  let mut current_idx = 0;
  let mut current_path = "/".to_string();
  tree_index.insert(current_path.clone(), current_idx);
  tree_content.push(Entry {
    kind: Type::Directory,
    size: 0,
    parent: None,
  });

  for full_command_with_result in std::fs::read_to_string(filename)?.split("$").skip(1) {
    let mut splitted_by_line = full_command_with_result.split("\n");
    // Check command
    let mut full_command = splitted_by_line.next().ok_or("Empty Line Found")?.trim().split(" ");
    let command = full_command.next().ok_or("Command not found")?;
    if command == "cd" {
      let folder_name = String::from(full_command.next().ok_or("Folder name not found")?);
      match folder_name.as_str() {
        "/" => {
          current_idx = 0;
          current_path = String::from("/");
        }
        ".." => {
          current_idx = tree_content[current_idx].parent.ok_or("No parent and try to go up with ..")?;
          let temp = current_path.split("/").collect::<Vec<_>>();
          current_path = temp[..temp.len() - 1].join("/");
        }
        _ => {
          let fullname = format!("{}/{}", current_path, folder_name);
          current_idx = match tree_index.get(&fullname) {
            Some(idx) => *idx,
            None => {
              let inserted_index = tree_content.len();
              tree_index.insert(folder_name, inserted_index);
              tree_content.push(Entry {
                kind: Type::Directory,
                size: 0,
                parent: Some(current_idx),
              });
              inserted_index
            },
          }
        }
      }
      continue;
    }
    // Check results
    for result in splitted_by_line {
      if result.is_empty() {
        continue;
      }
      let mut line = result.split(" ");
      let size_or_dir = line.next().ok_or("No Size or Dir found")?;
      let name = line.next().ok_or(format!("No Name found in {}", result))?;
      if size_or_dir != "dir" {
        let inserted_index = tree_content.len();
        tree_index.insert(name.to_string(), inserted_index);
        tree_content.push(Entry {
          kind: Type::File,
          size: size_or_dir.parse::<usize>()?,
          parent: Some(current_idx),
        });
      }
    }
  }

  // Populate dir size
  for original_index in 1..tree_content.len() {
    let mut current_idx = original_index;
    if tree_content[original_index].kind == Type::Directory {
      continue;
    }
    loop {
      let Some(parent_index) = tree_content[current_idx].parent else {
          break;
        };
      tree_content[parent_index].size += tree_content[original_index].size;
      current_idx = parent_index;
    }
  }
  // Compute part1
  let part1 = tree_content
    .iter()
    .filter_map(|entry| {
      if entry.kind == Type::Directory && entry.size <= 100000 {
        Some(entry.size)
      } else {
        None
      }
    })
    .sum::<usize>();

  let total_used_space = tree_content[0].size;
  let space_to_free = 30_000_000 - (70_000_000 - total_used_space);
  let mut part2_list = tree_content
    .iter()
    .filter_map(|entry| {
      if entry.kind == Type::Directory && entry.size >= space_to_free {
        Some(entry.size)
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
  part2_list.sort();
  let part2 = part2_list[0];
  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day07,        "data/day07.txt",              [1543140, 1117448];
    test1:  day07,        "data/day07_test1.txt",        [95437, 24933642];
  );
}
