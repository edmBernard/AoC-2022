
#include <algorithm>
#include <charconv>
#include <chrono>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <string_view>
#include <vector>

namespace {

template <typename T>
inline T Parse(std::string_view original, int base = 10) {
  T result;
  const auto [ptr, ec] = std::from_chars(original.data(), original.data() + original.size(), result, base);
  if (ec != std::errc())
    throw std::runtime_error("Fail to parse");
  return result;
}

std::string ReadToString(const std::string &filename) {
  std::ifstream infile(filename, std::ios::in | std::ios::binary);
  if (!infile.is_open())
    throw std::runtime_error("File Not Found");

  // Obtain the size of the file.
  const auto sz = std::filesystem::file_size(filename);
  // Create a buffer.
  std::string input_raw(sz, '\0');
  // Read the whole file into the buffer.
  infile.read(input_raw.data(), sz);
  return input_raw;
}

// Iterator on a string splitted by a delimiter (default:'\n')
// Usage:
//   - range loop :
//       for (auto line : IteratorOnLines(input_raw)) {
//   - vector initialization :
//       std::vector<std::string_view>(IteratorOnLines(input_raw).begin(), IteratorOnLines(input_raw).end())
class IteratorOnLines {

public:
  using iterator_category = std::input_iterator_tag;
  using difference_type = std::ptrdiff_t;
  using value_type = std::string_view;
  using pointer = value_type;
  using reference = value_type &;

  IteratorOnLines(std::string_view input_raw, const char delimiter = '\n')
      : input_raw(input_raw), delimiter(delimiter) {
    next = input_raw.find(delimiter, start);
    line = std::string_view{input_raw.data() + start, next - start};
  }

  IteratorOnLines &begin() {
    return *this;
  }
  // The end iterator is currently just a trick
  IteratorOnLines &end() {
    return *this;
  }

  IteratorOnLines &operator++() {
    start = ++next;
    next = input_raw.find(delimiter, start);
    line = std::string_view{input_raw.data() + start, next - start};
    return *this;
  }

  reference operator*() {
    return line;
  }

  bool operator!=(const IteratorOnLines &b) {
    // Ugly trick, we assume the check condition will always be vs the end iterator.
    // So we don't need to create a real end iterator we just check vs string_view end index
    return this->next != std::string_view::npos;
  };

private:
  std::string_view input_raw;
  char delimiter;
  // We have to store the current line otherwise we can't give it as a reference
  std::string_view line;
  size_t start = 0;
  size_t next = 0;
};

} // namespace

// cat ../data/day01.txt | ./standalonecpp
int main_from_stdin() {
  std::vector<uint64_t> input_puzzle;
  uint64_t value = 0;
  for (std::string line; std::getline(std::cin, line);) {
    if (line.empty()) {
      input_puzzle.push_back(value);
      value = 0;
      continue;
    }
    value += Parse<uint64_t>(line);
  }
  // std::ranges::sort(input_puzzle);
  std::sort(input_puzzle.begin(), input_puzzle.end());

  std::cout << "part1: " << input_puzzle[input_puzzle.size() - 1] << "\n";
  std::cout << "part2: " << std::reduce(input_puzzle.begin() + input_puzzle.size() - 3, input_puzzle.end()) << "\n";

  return 0;
}

// ./standalonecpp ../data/day01.txt
int main_regular(int argc, char *argv[]) {
  if (argc <= 1)
    return 1;

  auto start_temp = std::chrono::high_resolution_clock::now();
  uint64_t part1 = 0;
  uint64_t part2 = 0;
  for (int i = 0; i < 10000; ++i) {

    std::ifstream infile(argv[1]);
    if (!infile.is_open())
      throw std::runtime_error("File Not Found");

    std::vector<uint64_t> input_puzzle;
    uint64_t value = 0;
    for (std::string line; std::getline(infile, line);) {
      if (line.empty()) {
        input_puzzle.push_back(value);
        value = 0;
        continue;
      }
      value += Parse<uint64_t>(line);
    }

    // std::ranges::sort(input_puzzle);
    std::sort(input_puzzle.begin(), input_puzzle.end());

    part1 = input_puzzle[input_puzzle.size() - 1];
    part2 = std::reduce(input_puzzle.begin() + input_puzzle.size() - 3, input_puzzle.end());
  }

  std::chrono::duration<double, std::micro> elapsed_temp = std::chrono::high_resolution_clock::now() - start_temp;
  std::cout << "day01_regular \tin " << elapsed_temp.count() / 10000. << " us : part1=" << part1 << " \tpart2=" << part2 << std::endl;

  return 0;
}

// ./standalonecpp ../data/day01.txt
// Design for speed but notation was a bit ugly
// We read the whole file in a string
int main_speed_raw(int argc, char *argv[]) {
  if (argc <= 1)
    return 1;

  auto start_temp = std::chrono::high_resolution_clock::now();
  uint64_t part1 = 0;
  uint64_t part2 = 0;

  for (int i = 0; i < 10000; ++i) {

    std::ifstream infile(argv[1], std::ios::in | std::ios::binary);
    if (!infile.is_open())
      throw std::runtime_error("File Not Found");

    // Obtain the size of the file.
    const auto sz = std::filesystem::file_size(argv[1]);
    // Create a buffer.
    std::string input_raw(sz, '\0');
    // Read the whole file into the buffer.
    infile.read(input_raw.data(), sz);

    std::vector<uint64_t> input_puzzle;
    uint64_t value = 0;

    // Reading directly the whole file and parsing each line is faster than getline+parsing
    for (size_t start = 0, next = input_raw.find('\n', start), parsed = 0;
         next != std::string_view::npos;
         start = ++next, next = input_raw.find('\n', start)) {

      if (next == start) {
        input_puzzle.push_back(value);
        value = 0;
        continue;
      }
      const auto [ptr, ec] = std::from_chars(input_raw.data() + start, input_raw.data() + next, parsed);
      value += parsed;
    };

    // std::ranges::sort(input_puzzle);
    std::sort(input_puzzle.begin(), input_puzzle.end());

    part1 = input_puzzle[input_puzzle.size() - 1];
    part2 = std::reduce(input_puzzle.begin() + input_puzzle.size() - 3, input_puzzle.end());
  }

  std::chrono::duration<double, std::micro> elapsed_temp = std::chrono::high_resolution_clock::now() - start_temp;
  std::cout << "day01_speed_raw \tin " << elapsed_temp.count() / 10000. << " us : part1=" << part1 << " \tpart2=" << part2 << std::endl;


  return 0;
}

// ./standalonecpp ../data/day01.txt
// We read the whole file in a string
// We use an iterator implementation that was a bit tricky/ugly
// but we have a nice syntaxe and full speed
int main_speed_iter(int argc, char *argv[]) {
  if (argc <= 1)
    return 1;

  auto start_temp = std::chrono::high_resolution_clock::now();
  uint64_t part1 = 0;
  uint64_t part2 = 0;

  for (int i = 0; i < 10000; ++i) {

    std::string input_raw = ReadToString(argv[1]);

    std::vector<uint64_t> input_puzzle;
    uint64_t value = 0;

    // Reading directly the whole file and parsing each line is faster than getline+parsing
    size_t parsed = 0;
    for (auto line : IteratorOnLines(input_raw)) {

      if (line.empty()) {
        input_puzzle.push_back(value);
        value = 0;
        continue;
      }
      value += Parse<uint64_t>(line);
    };

    // std::ranges::sort(input_puzzle);
    std::sort(input_puzzle.begin(), input_puzzle.end());

    part1 = input_puzzle[input_puzzle.size() - 1];
    part2 = std::reduce(input_puzzle.begin() + input_puzzle.size() - 3, input_puzzle.end());
  }

  std::chrono::duration<double, std::micro> elapsed_temp = std::chrono::high_resolution_clock::now() - start_temp;
  std::cout << "day01_speed_iter \tin " << elapsed_temp.count() / 10000. << " us : part1=" << part1 << " \tpart2=" << part2 << std::endl;

  return 0;
}

int main(int argc, char *argv[]) {
  main_regular(argc, argv);
  main_speed_raw(argc, argv);
  // Surprisingly the iterator version is slightly faster in release than raw
  main_speed_iter(argc, argv);
  return 0;
}