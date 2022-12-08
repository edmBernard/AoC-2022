// #![allow(unused_variables)]
use std::hash::{Hash, Hasher};
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
  let file_content = std::fs::read_to_string(filename)?;
  let mut tree_index: HashMap<String, usize> = HashMap::new();
  let mut tree_content: Vec<Entry> = Vec::new();

  // I can't make the HashMap<String, Option<Entry>> work on the second
  // iteration where we compute size because we borrow the key and modify value
  let root = "/".to_string();
  let mut current_idx = 0;
  tree_index.insert(root, current_idx);
  tree_content.push(Entry{kind:Type::Directory, size:0, parent:None});

  file_content
    .split("$").skip(1)
    .for_each(|full_command_with_result| {
      let mut splitted = full_command_with_result.split("\n");
      // Check command
      let full_command = splitted.next().unwrap();
      let mut temp = full_command.trim().split(" ");
      let command = temp.next().unwrap();
      if command == "cd" {
        let folder_name = String::from(temp.next().unwrap());
        if folder_name == ".." {
          current_idx = tree_content[current_idx].parent.unwrap();
        } else {
          if !tree_index.contains_key(&folder_name) {
            let inserted_index = tree_content.len();
            tree_index.insert(folder_name, inserted_index);
            tree_content.push(Entry{kind:Type::Directory, size:0, parent:Some(current_idx)});
            current_idx = inserted_index;
          } else {
            current_idx = tree_index.get(&folder_name).unwrap().clone();
          }
        }
        return;
      }
      // Check results
      for result in splitted {
        let mut line = result.split(" ");
        let Some(size_or_dir) = line.next() else {
          continue;
        };
        let Some(name) = line.next() else {
          continue;
        };
        if size_or_dir != "dir" {
          let inserted_index = tree_content.len();
          tree_index.insert(name.to_string(), inserted_index);
          tree_content.push(Entry{kind:Type::File, size:size_or_dir.parse::<usize>().unwrap(), parent:Some(current_idx)});
        }
      }
    });

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
    let part1 = tree_content.iter().filter_map(|entry| if entry.kind == Type::Directory && entry.size <= 100000 {
      println!("{}", entry.size);
      Some(entry.size) } else { None }
    ).sum::<usize>();
    for (k, v) in &tree_index {
      println!("k:-{:?}- v:-{:?}-", k, tree_content[*v]);
    }

    // for (k, v) in tree_index {
    //   println!("k:-{:?}- v:-{:?}-", k, v);
    // }
  Ok(ReturnType::Numeric(part1 as u64, 2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day07,        "data/day07.txt",              [1282, 3513];
    test1:  day07,        "data/day07_test1.txt",        [95437, 19];
  );
}
