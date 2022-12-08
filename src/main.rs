// #![deny(warnings)]

use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::time::Instant;

mod utils;
use utils::ReturnType;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

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
  // for _ in 0..10000 {
  //   _ = command(filepath);
  // }
  match command(filepath) {
    Ok(result) => {
      let duration = now.elapsed().as_micros();
      let (part1, part2) = match result {
        ReturnType::Numeric(part1, part2) => (format!("{}", part1), format!("{}", part2)),
        ReturnType::String(part1, part2) => (format!("{}", part1), format!("{}", part2)),
      };
      println!(
        "{: <30} in {:>7.2} ms : part1={:<10} part2={:<10}",
        name,
        duration as f32 / 1000.,
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
    day01::day01,
    day01::day01_speed,
    day02::day02,
    day02::day02_speed,
    day03::day03,
    day03::day03_speed,
    day04::day04,
    day04::day04_speed,
    day05::day05,
    day05::day05_speed,
    day06::day06,
    day06::day06_speed,
    day07::day07,
    day08::day08,
    day08::day08_speed,
    day09::day09
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
