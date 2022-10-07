// #![deny(warnings)]

use clap::Parser;
use std::path::Path;
use std::time::Instant;

mod day01;

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

#[macro_export]
macro_rules! register_command {
    ( $( $func:expr),+ ) => {
        {
          // Intermediate variable to force type. otherwise function type is not generic
          let reg: Vec<(&str, fn(&Path) -> Result<[u64; 2], std::io::Error>)> = vec![
            $((stringify!($func), $func),)*
          ];
          reg
        }
    };
}

fn main() {
  let args = Args::parse();
  let input_filename = args.input.unwrap_or(String::from("data"));
  let input_path = Path::new(&input_filename);

  if !input_path.exists() {
    eprintln!("Error: Input filename not found. {}", input_path.display());
    return;
  }

  let register = register_command!(day01::day01, day01::day01functional);

  let mut total_time = 0u128;
  if input_path.is_file() {
    println!("{}", input_path.display());
    for (name, command) in register.into_iter() {
      // using if let ... && name.contains(...) is not yet ready : https://github.com/rust-lang/rust/issues/53667
      // same for is_some_and https://github.com/rust-lang/rust/issues/93050
      if let Some(filter) = args.filter_inclusion.clone() {
        if !name.contains(&filter) {
          continue;
        }
      }

      // using if let ... && name.contains(...) is not yet ready : https://github.com/rust-lang/rust/issues/53667
      // same for is_some_and https://github.com/rust-lang/rust/issues/93050
      if let Some(filter) = args.filter_exclusion.clone() {
        if name.contains(&filter) {
          continue;
        }
      }

      let now = Instant::now();
      match command(input_path) {
        Ok([part1, part2]) => {
          let duration = now.elapsed().as_micros();
          total_time += duration;
          println!(
            "{: <30} in {:>7.2} ms : part1={:<10} part2={:<10}",
            name,
            duration as f32 / 1000.,
            part1,
            part2
          )
        }
        Err(msg) => eprintln!("Error: in {}: {}", name, msg),
      };
    }
  }

  println!("Total time : {:>7.2} ms", total_time as f32 / 1000.)
}
