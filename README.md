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
| Day01 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/1 ) [<sup>solution</sup>](src/days/day01.rs) | Max of range                 | -     |
| Day02 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/2 ) [<sup>solution</sup>](src/days/day02.rs) | Rock Paper Scissors          | -     |
| Day03 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/3 ) [<sup>solution</sup>](src/days/day03.rs) | Duplicate in compartments    | -     |
| Day04 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/4 ) [<sup>solution</sup>](src/days/day04.rs) | Check range overlapping      | -     |
| Day05 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/5 ) [<sup>solution</sup>](src/days/day05.rs) | Stack swap                   | -     |
| Day06 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/6 ) [<sup>solution</sup>](src/days/day06.rs) | Check block if duplicate     | -     |
| Day07 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/7 ) [<sup>solution</sup>](src/days/day07.rs) | Directory mapping            | -     |
| Day08 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/8 ) [<sup>solution</sup>](src/days/day08.rs) | Line of sight                | -     |
| Day09 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/9 ) [<sup>solution</sup>](src/days/day09.rs) | Move rope                    | Using a big vec instead of HashSet |
| Day10 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/10) [<sup>solution</sup>](src/days/day10.rs) | cycle synchronisation in CRT | part2 is a visual solution  |



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
day09::day09_speed             in 3783.43 ms : part1=6503       part2=2724
day10::day10                   in  439.23 ms : part1=11720      part2=2

In C++
day01                          in 665.876 ms : part1=70720      part2=207148
day02                          in 689.468 ms : part1=11475      part2=16862
day03                          in 573.196 ms : part1=8202       part2=2864
day04                          in 739.069 ms : part1=490        part2=921
```