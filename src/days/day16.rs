// #![allow(unused_variables)]

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

fn dfs(
  valve_index: usize,
  remaining_time: i32,
  remaning_valve: Vec<usize>,
  valve_flow: &Vec<i32>,
  adjacent_matrix: &Vec<Vec<i32>>,
  cache: &mut HashMap<(usize, i32, Vec<usize>), i32>,
) -> i32 {
  let cache_key = (valve_index, remaining_time, remaning_valve.clone());
  if let Some(cached_value) = cache.get(&cache_key) {
    return *cached_value;
  }

  let mut score = 0;
  for idx in 0..remaning_valve.len() {
    let next = remaning_valve[idx];
    if adjacent_matrix[valve_index][next] >= remaining_time {
      continue;
    }
    let mut remain = remaning_valve.clone();
    remain.remove(idx);
    let time = remaining_time - adjacent_matrix[valve_index][next] - 1;
    score = score.max(
      valve_flow[next] * (remaining_time - adjacent_matrix[valve_index][next] - 1)
        + dfs(next, time, remain, valve_flow, adjacent_matrix, cache),
    );
  }
  cache.insert(cache_key, score);
  score
}

// caching part2 result is negligible because we still use caching of dfs
fn dfs2(
  valve_index: usize,
  remaining_time: i32,
  remaning_valve: Vec<usize>,
  aa_index: usize,
  valve_flow: &Vec<i32>,
  adjacent_matrix: &Vec<Vec<i32>>,
  cache1: &mut HashMap<(usize, i32, Vec<usize>), i32>,
  cache2: &mut HashMap<(usize, i32, Vec<usize>), i32>,
) -> i32 {

  let cache_key = (valve_index, remaining_time, remaning_valve.clone());
  if let Some(cached_value) = cache2.get(&cache_key) {
    return *cached_value;
  }

  let mut score = 0;
  for idx in 0..remaning_valve.len() {
    let next = remaning_valve[idx];
    if adjacent_matrix[valve_index][next] >= remaining_time {
      continue;
    }
    let mut remain = remaning_valve.clone();
    remain.remove(idx);
    let time = remaining_time - adjacent_matrix[valve_index][next] - 1;
    score = score.max(valve_flow[next] * (remaining_time - adjacent_matrix[valve_index][next] - 1)
                    + dfs2(next, time, remain, aa_index, valve_flow, adjacent_matrix, cache1, cache2));
  }

  let temp_score = dfs(aa_index, 26, remaning_valve.clone(), valve_flow, adjacent_matrix, cache1);
  cache2.insert(cache_key, i32::max(temp_score, score));
  i32::max(temp_score, score)
}

// For part1 my solution was wrong only on the real input, I was on off by 1 and haven't found why
// Here is a translation in rust of https://github.com/betaveros/advent-of-code-2022/blob/main/p16.noul
// that give the right result
pub fn day16(filename: &Path) -> Result<ReturnType> {
  let mut valve_index = Vec::new();
  let mut valve_connection = Vec::new();
  let mut valve_flow = Vec::new();

  // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
  let re = Regex::new(r"([A-Z]{2}).*=(\d+).+?((?:,? [A-Z]{2})+)")?;
  let content = std::fs::read_to_string(filename)?;
  for line in content.lines() {
    let caps = re.captures(line).ok_or("Fail to capture")?;
    let current_valve = caps.get(1).ok_or("Fail to capture valve id")?.as_str();
    let flow_rate = caps
      .get(2)
      .ok_or("Fail to capture flow rate")?
      .as_str()
      .parse::<i32>()?;
    let next_valve = caps
      .get(3)
      .ok_or("Fail to capture next valves id")?
      .as_str()
      .split(',')
      .map(|elem| elem.trim())
      .collect::<Vec<_>>();

    valve_index.push(current_valve);
    valve_connection.push(next_valve);
    valve_flow.push(flow_rate);
  }

  let mut adjacent_matrix = vec![vec![99; valve_index.len()]; valve_index.len()];
  for (idx, connections) in valve_connection.iter().enumerate() {
    for connection in connections {
      adjacent_matrix[idx][valve_index.iter().position(|e| e == connection).unwrap()] = 1;
    }
  }

  // Floyd-Warshall
  for k in 0..valve_index.len() {
    for i in 0..valve_index.len() {
      for j in 0..valve_index.len() {
        adjacent_matrix[i][j] = adjacent_matrix[i][j].min(adjacent_matrix[i][k] + adjacent_matrix[k][j]);
      }
    }
  }

  let valve_with_flow = valve_flow
    .iter()
    .enumerate()
    .filter_map(|(i, &f)| if f > 0 { Some(i) } else { None })
    .collect::<Vec<_>>();

  let mut cache1 = HashMap::new();
  let mut cache2 = HashMap::new();
  let aa_index = valve_index.iter().position(|&e| e == "AA").unwrap();
  let part1 = dfs(
    aa_index,
    30,
    valve_with_flow.clone(),
    &valve_flow,
    &adjacent_matrix,
    &mut cache1,
  );
  let part2 = dfs2(
    aa_index,
    26,
    valve_with_flow,
    aa_index,
    &valve_flow,
    &adjacent_matrix,
    &mut cache1,
    &mut cache2
  );

  Ok(ReturnType::Numeric(part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day16,        "data/day16.txt",       [1754, 2474];
    test1:  day16,        "data/day16_test1.txt", [24, 1707];
  );
}
