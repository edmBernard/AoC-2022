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
| Day11 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/11) [<sup>solution</sup>](src/days/day11.rs) | Monkey pass | part2 overflow so we use Least common multiple-like to reduce worry level at each pass |
| Day12 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/12) [<sup>solution</sup>](src/days/day12.rs) | Search path algorithm (BFS or Dijkstra) | Start from the End and different stop condition for part1 and part2 |
| Day14 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/14) [<sup>solution</sup>](src/days/day14.rs) | Sandfall | - |
| Day15 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/15) [<sup>solution</sup>](src/days/day15.rs) | Blind spot in Sensor Area | Check only points in frontier of the detected area |
| Day16 [<sup>puzzle</sup>](https://adventofcode.com/2022/day/16) [<sup>solution</sup>](src/days/day16.rs) | Graph search with valve | Floyd-Warshall and memoization |

## Some Timing on `10'000` run :

```
In Rust
days::day01::day01_speed       in    67.84 us : part1=70720      part2=207148
days::day02::day02_speed       in    81.44 us : part1=11475      part2=16862
days::day03::day03_speed       in    73.59 us : part1=8202       part2=2864
days::day04::day04_speed       in    62.08 us : part1=490        part2=921
days::day05::day05_speed       in    86.36 us : part1=ZWHVFWQWW  part2=HZFZCCWWV
days::day06::day06_speed       in    52.09 us : part1=1282       part2=3513
days::day07::day07             in   276.58 us : part1=1543140    part2=1117448
days::day08::day08_speed       in   257.59 us : part1=1688       part2=410400
days::day09::day09_speed       in   383.57 us : part1=6503       part2=2724
days::day10::day10             in    43.77 us : part1=11720      part2=2
days::day11::day11_speed       in  1999.36 us : part1=117624     part2=16792940265
days::day12::day12_speed       in  1567.76 us : part1=437        part2=430
days::day14::day14             in  3489.00 us : part1=1003       part2=25771
days::day15::day15_speed       in 14748.00 us : part1=5525990    part2=11756174628223

In C++
day01_speed_iter               in 66.7159 us : part1=70720      part2=207148
day02                          in 67.3343 us : part1=11475      part2=16862
day03                          in 55.5822 us : part1=8202       part2=2864
day04                          in 72.6224 us : part1=490        part2=921
day05                          in 78.4286 us : part1=ZWHVFWQWW  part2=HZFZCCWWV
```