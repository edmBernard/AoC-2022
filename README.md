# AoC-2022

My solutions to [Advent of Code 2022](https://adventofcode.com/). This year, I'm going to try doing the challenges in completly in [Rust](https://www.rust-lang.org/)

- Github : [https://github.com/edmBernard/AoC-2022](https://github.com/edmBernard/AoC-2022)

## Getting Started

```bash
git clone git@github.com:edmBernard/AoC-2022.git
cd AoC-2022
cargo build --release
```

## Run

```bash
cargo run -- data
```

## Test

Run each part and expect result to be from my input
```bash
cargo test
```

The executable is named `AoC-2022`. Inputs path can be specified as argument or by default in the data folder.

## Problem

| Day   | Description                | Tips  |
|--     |--                          |--     |
| Day01 | Max of range               | -     |
| Day02 | Rock Paper Scissors        | -     |
| Day03 | Duplicate in compartments  | -     |
| Day04 | Check range overlapping    | -     |
| Day05 | Stack swap                 | -     |
| Day06 | Check block if duplicate   | -     |
| Day07 | Directory mapping          | -     |
| Day08 | Line of sight              | -     |

## Some Timing for `10'000` run :

```
In Rust
day01::day01_speed             in  676.64 ms : part1=70720      part2=207148
day02::day02_speed             in  828.58 ms : part1=11475      part2=16862
day03::day03_speed             in  759.69 ms : part1=8202       part2=2864
day04::day04_speed             in  622.73 ms : part1=490        part2=921
day05::day05_speed             in  866.24 ms : part1=ZWHVFWQWW  part2=HZFZCCWWV
day06::day06_speed             in  521.10 ms : part1=1282       part2=3513
day07::day07                   in 1807.73 ms : part1=1543140    part2=1117448
day08::day08_speed             in 2443.45 ms : part1=1688       part2=410400

In C++
day01                          in 665.876 ms : part1=70720      part2=207148
day02                          in 689.468 ms : part1=11475      part2=16862
day03                          in 573.196 ms : part1=8202       part2=2864
day04                          in 739.069 ms : part1=490        part2=921
```