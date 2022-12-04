
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

// ./standalonecpp ../data/day03.txt
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

    part1 = 0;
    part2 = 0;

    // Reading directly the whole file and parsing each line is faster than getline+parsing
    for (auto line : IteratorOnLines(input_raw)) {

      const size_t posToken1 = line.find("-");
      const size_t posToken2 = line.find(",");
      const size_t posToken3 = line.find("-", posToken2);
      const uint64_t minElf1 = Parse<uint64_t>(std::string_view{line.data(), posToken1});
      const uint64_t maxElf1 = Parse<uint64_t>(std::string_view{line.data() + posToken1 + 1, posToken2 - posToken1 - 1});
      const uint64_t minElf2 = Parse<uint64_t>(std::string_view{line.data() + posToken2 + 1, posToken3 - posToken2 - 1});
      const uint64_t maxElf2 = Parse<uint64_t>(std::string_view{line.data() + posToken3 + 1, line.size() - posToken3 - 1});
      // part1
      if (minElf1 <= minElf2 && maxElf1 >= maxElf2) {
        part1 += 1;
      } else if (minElf2 <= minElf1 && maxElf2 >= maxElf1) {
        part1 += 1;
      };
      // part2
      if (minElf1 <= minElf2 && maxElf1 >= minElf2) {
        part2 += 1;
      } else if (minElf2 <= minElf1 && maxElf2 >= minElf1) {
        part2 += 1;
      }
    }
  }

  std::chrono::duration<double, std::milli> elapsed_temp = std::chrono::high_resolution_clock::now() - start_temp;
  std::cout << "day04 \t\t\tin " << elapsed_temp.count() << " ms : part1=" << part1 << " \tpart2=" << part2 << std::endl;
  return 0;
}

int main(int argc, char *argv[]) {
  main_speed_iter(argc, argv);
  return 0;
}