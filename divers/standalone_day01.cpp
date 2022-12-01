
#include <iostream>
#include <string>
#include <vector>
#include <charconv>
#include <algorithm>
#include <numeric>
#include <fstream>
#include <chrono>

template <typename T>
inline T parse(std::string_view original, int base = 10)
{
  T result;
  const auto [ptr, ec] = std::from_chars(original.data(), original.data() + original.size(), result, base);
  if (ec != std::errc())
    throw std::runtime_error("Fail to parse");
  return result;
}

// cat ../data/day01.txt | ./standalonecpp
int main_from_stdin()
{
  std::vector<uint64_t> input_puzzle;
  uint64_t value = 0;
  for (std::string line; std::getline(std::cin, line);)
  {
    if (line.empty())
    {
      input_puzzle.push_back(value);
      value = 0;
      continue;
    }
    value += parse<uint64_t>(line);
  }
  // std::ranges::sort(input_puzzle);
  std::sort(input_puzzle.begin(), input_puzzle.end());

  std::cout << "part1: " << input_puzzle[input_puzzle.size() - 1] << "\n";
  std::cout << "part2: " << std::reduce(input_puzzle.begin() + input_puzzle.size() - 3, input_puzzle.end()) << "\n";

  return 0;
}

// ./standalonecpp ../data/day01.txt
int main(int argc, char *argv[])
{
  if (argc <= 1)
    return 1;

  auto start_temp = std::chrono::high_resolution_clock::now();
  uint64_t part1 = 0;
  uint64_t part2 = 0;

  std::ifstream infile(argv[1]);
  if (!infile.is_open())
  {
    throw std::runtime_error("File Not Found");
  }

  std::vector<uint64_t> input_puzzle;
  uint64_t value = 0;
  for (std::string line; std::getline(infile, line);)
  {
    if (line.empty())
    {
      input_puzzle.push_back(value);
      value = 0;
      continue;
    }
    value += parse<uint64_t>(line);
  }
  // std::ranges::sort(input_puzzle);
  std::sort(input_puzzle.begin(), input_puzzle.end());

  part1 = input_puzzle[input_puzzle.size() - 1];
  part2 = std::reduce(input_puzzle.begin() + input_puzzle.size() - 3, input_puzzle.end());

  std::cout << "part1: " << part1 << "\n";
  std::cout << "part2: " << part2 << "\n";

  std::chrono::duration<double, std::milli> elapsed_temp = std::chrono::high_resolution_clock::now() - start_temp;
  std::cout << "total duration: " << elapsed_temp.count() << "ms \n";

  return 0;
}
