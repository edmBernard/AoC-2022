// #![deny(warnings)]

use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::time::Instant;

mod utils;
use utils::ReturnType;

mod days;

#[cfg(test)]
mod test_helper;

#[doc(hidden)]
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

/// Command signature
/// # Argument
/// * `filename` - filename containing problem input
type CommandFunction = fn(filename: &Path) -> Result<ReturnType>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// Filename or directory containing input files
  input: Option<String>,

  /// Filter to include only match command. No filter if missing.
  #[clap(short = 'i', long = "include")]
  filter_inclusion: Option<String>,

  /// Filter to exclude matching command. No filter if missing.
  #[clap(short = 'e', long = "exclude")]
  filter_exclusion: Option<String>,
}

/// Macro to register command with
/// # Arguments
/// * `func` - function that take file and output part1 and part2 result
/// # Example
/// ```
/// let register = register_command!(day01::day01, day01::day01functional);
/// ```
#[macro_export]
macro_rules! register_command {
    ( $( $func:expr),+ ) => {
        {
          // Intermediate variable to force type. otherwise function type is not generic
          let reg: Vec<(&str, CommandFunction)> = vec![
            $((stringify!($func), $func),)*
          ];
          reg
        }
    };
}

/// Launch and time the command execution
/// # Arguments
/// * `command` - command to execute
/// * `filepath` - filename passed to the command function
/// * `name` - name of the command
fn measure_command_execution(command: &CommandFunction, filepath: &Path, name: &str) -> Option<u128> {
  let now = Instant::now();
  const NRUN: u32 = 10000;
  for _ in 0..NRUN-1 {
    _ = command(filepath);
  }
  match command(filepath) {
    Ok(result) => {
      let duration = now.elapsed().as_micros();
      let (part1, part2) = match result {
        ReturnType::Numeric(part1, part2) => (format!("{}", part1), format!("{}", part2)),
        ReturnType::String(part1, part2) => (format!("{}", part1), format!("{}", part2)),
      };
      println!(
        "{: <30} in {:>7.2} us : part1={:<10} part2={:<10}",
        name,
        duration as f32 / NRUN as f32,
        part1,
        part2
      );
      Some(duration)
    }
    Err(e) => {
      eprintln!("Error: in {}: {}", name, e);
      None
    }
  }
}

fn main() {
  let register = register_command!(
    days::day01::day01,
    days::day01::day01_speed,
    days::day02::day02,
    days::day02::day02_speed,
    days::day03::day03,
    days::day03::day03_speed,
    days::day04::day04,
    days::day04::day04_speed,
    days::day05::day05,
    days::day05::day05_speed,
    days::day06::day06,
    days::day06::day06_speed,
    days::day07::day07,
    days::day08::day08,
    days::day08::day08_speed,
    days::day09::day09,
    days::day09::day09_speed,
    days::day10::day10,
    days::day11::day11,
    days::day11::day11_speed,
    days::day12::day12,
    days::day12::day12_speed
    // days::day13::day13,
    // days::day14::day14,
    // days::day15::day15,
    // days::day16::day16,
    // days::day17::day17,
    // days::day18::day18,
    // days::day19::day19,
    // days::day20::day20,
    // days::day21::day21,
    // days::day22::day22,
    // days::day23::day23,
    // days::day24::day24,
    // days::day25::day25
  );

  let args = Args::parse();
  let input_filename = args.input.unwrap_or(String::from("data"));
  let input_path = Path::new(&input_filename);

  if !input_path.exists() {
    eprintln!("Error: Input filename not found. {}", input_path.display());
    return;
  }

  let mut total_time = 0u128;

  // Apply commands to given file
  if input_path.is_file() {
    for (name, command) in register.iter() {
      if let Some(filter) = args.filter_inclusion.clone() {
        if !name.contains(&filter) {
          continue;
        }
      }

      if let Some(filter) = args.filter_exclusion.clone() {
        if name.contains(&filter) {
          continue;
        }
      }

      if let Some(duration) = measure_command_execution(command, input_path, name) {
        total_time += duration;
      }
    }
  }

  // Apply commands all files in directory
  // respecting the match between day:
  //   day01.txt -> fn day01()
  //   day02.txt -> fn day02()
  //   etc ...
  if input_path.is_dir() {
    let re = Regex::new(r"(day\d{2})").expect("Failed to parse regex");

    for (name, command) in register.iter() {
      if let Some(filter) = args.filter_inclusion.clone() {
        if !name.contains(&filter) {
          continue;
        }
      }

      if let Some(filter) = args.filter_exclusion.clone() {
        if name.contains(&filter) {
          continue;
        }
      }

      // Iteration on files in directory
      if let Ok(dir) = fs::read_dir(input_path) {
        dir
          .filter_map(|filepath_result| {
            let filepath = filepath_result.ok()?;
            let filename = filepath.file_name();
            let caps = re.captures(filename.to_str()?)?;
            let captured = caps.get(1).map_or("", |m| m.as_str());
            if captured.is_empty() {
              return None;
            }
            if !name.contains(&captured) {
              return None;
            }
            Some(filepath)
          })
          .for_each(|filepath| {
            if let Some(duration) = measure_command_execution(command, &filepath.path(), name) {
              total_time += duration;
            }
          });
      }
    }
  }

  println!("Total time : {:>7.2} ms", total_time as f32 / 1000.)
}
