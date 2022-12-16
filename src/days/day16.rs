// #![allow(unused_variables)]

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::utils::ReturnType;
use crate::Result;

fn go_deeper(
  valve_graph: &HashMap<&str, Vec<&str>>,
  valve_flow: &HashMap<&str, i32>,
  current_id: &str,
  current_score: i32,
  current_flow_rate: i32,
  count: i32,
  valve_status: &HashSet<&str>,
  cache: &mut HashMap<(i32, String, i32), i32>,
) -> i32 {
  if count > 30 {
    return current_score;
  }

  let cache_key = (count, current_id.to_string(), current_flow_rate);
  if let Some(cached_value) = cache.get(&cache_key) {
    if *cached_value >= current_score {
      // println!("flow_rate:{} score:{} cached:{}", current_flow_rate, current_score, *cached_value);
      return current_score;
    }
  }
  cache.insert(cache_key, current_score);

  // println!("id:{}, count:{}, score:{}, flow_rate:{}", current_id, count, current_score, current_flow_rate);
  let new_score = current_score + current_flow_rate;

  let mut score = new_score;
  // choice 1: open valve
  let current_flow = *valve_flow.get(&current_id).unwrap();
  if !valve_status.contains(&current_id) && current_flow != 0 {
    let mut valve_status_clone = valve_status.clone();
    valve_status_clone.insert(&current_id);

    let new_flow_rate = current_flow_rate + current_flow;
    score = go_deeper(
      valve_graph,
      valve_flow,
      current_id,
      new_score,
      new_flow_rate,
      count + 1,
      &valve_status_clone,
      cache,
    );
  }

  // choice 2: next valve
  for valve in valve_graph.get(&current_id).unwrap() {
    score = go_deeper(
      valve_graph,
      valve_flow,
      valve,
      new_score,
      current_flow_rate,
      count + 1,
      valve_status,
      cache,
    ).max(score);
  }
  score
}

pub fn day16(filename: &Path) -> Result<ReturnType> {
  let mut valve_graph = HashMap::new();
  let mut valve_flow = HashMap::new();
  let mut valve_status = HashSet::new();
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
      .split(",")
      .map(|elem| elem.trim())
      .collect::<Vec<_>>();

    valve_graph.insert(current_valve, next_valve);
    valve_flow.insert(current_valve, flow_rate);
    // valve_status.insert(current_valve, false);
  }
  let content = std::fs::read_to_string(filename)?;
  for line in content.lines() {
    let caps = re.captures(line).ok_or("Fail to capture")?;
    let current_valve = caps.get(1).ok_or("Fail to capture valve id")?.as_str();

    let list = valve_graph.get(current_valve).unwrap().iter().join(", ");
    println!(
      "Valve {} has flow rate={}; tunnels lead to valves {}",
      current_valve,
      valve_flow.get(current_valve).unwrap(),
      list
    );
  }

  let mut cache = HashMap::new();
  let part1 = go_deeper(&valve_graph, &valve_flow, "AA", 0, 0, 1, &valve_status, &mut cache);

  println!("IT DON'T WORK, I WAS OFF BY 1 POINT");
  Ok(ReturnType::Numeric(part1 as u64, 2 as u64))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helper::add_test;

  #[rustfmt::skip::macros(add_test)]
  add_test!(
    main:   day16,        "data/day16.txt",       [1003, 25771];
    test1:  day16,        "data/day16_test1.txt", [24, 93];
  );
}
