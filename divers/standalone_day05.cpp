
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
  // IteratorOnLines(const IteratorOnLines&) = delete;
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

enum class State {
  ParseHeader,
  InterpreteHeader,
  ParseMovement,
};

// ./standalonecpp ../data/day03.txt
// We read the whole file in a string
// We use an iterator implementation that was a bit tricky/ugly
// but we have a nice syntaxe and full speed
int main_speed_iter(int argc, char *argv[]) {
  if (argc <= 1)
    return 1;

  auto start_temp = std::chrono::high_resolution_clock::now();
  std::string part1;
  std::string part2;

  for (int benchRun = 0; benchRun < 10000; ++benchRun) {

    std::string input_raw = ReadToString(argv[1]);

    part1 = "";
    part2 = "";

    // Reading directly the whole file and parsing each line is faster than getline+parsing
    std::vector<std::string_view> header;
    size_t binSize = 0;
    State state = State::ParseHeader;
    std::vector<std::vector<char>> boardPart1;
    std::vector<std::vector<char>> boardPart2;
    std::vector<char> tempPart1;
    std::vector<char> tempPart2;
    for (auto &line : IteratorOnLines(input_raw)) {
      switch (state) {
      case State::ParseHeader: {
        if (line[1] == '1') {
          binSize = line[line.size() - 1] - '0';
          state = State::InterpreteHeader;
        } else {
          header.push_back(line);
        }
      } break;
      case State::InterpreteHeader: {
        boardPart1 = std::vector<std::vector<char>>(binSize);
        for (int lineIndex = header.size() - 1; lineIndex >= 0; --lineIndex) {
          for (int stackOffset = 1, stackIndex = 0; stackOffset < header[lineIndex].size(); stackOffset += 4, ++stackIndex) {
            if (header[lineIndex][stackOffset] != ' ') {
              boardPart1[stackIndex].push_back(header[lineIndex][stackOffset]);
            }
          }
        }
        boardPart2 = boardPart1;
        state = State::ParseMovement;
      } break;
      case State::ParseMovement: {
        // Movement line are of the form "move (\d+) from (\d+) to (\d+)"
        auto splitter = IteratorOnLines(line, ' ');
        ++splitter;
        const uint64_t quantity = Parse<uint64_t>(*splitter);
        ++++splitter;
        const uint64_t src = Parse<uint64_t>(*splitter) - 1;
        ++++splitter;
        const uint64_t dst = Parse<uint64_t>(*splitter) - 1;

        for (int i = 0; i < quantity; ++i) {
          tempPart1.push_back(boardPart1[src].back());
          boardPart1[src].pop_back();
          tempPart2.push_back(boardPart2[src].back());
          boardPart2[src].pop_back();
        }
        for (int i = tempPart2.size() - 1; i >= 0; --i) {
          boardPart2[dst].push_back(tempPart2[i]);
        }
        for (int i = 0; i < tempPart1.size(); ++i) {
          boardPart1[dst].push_back(tempPart1[i]);
        }
        tempPart1.clear();
        tempPart2.clear();
      } break;
      }
    }

    for (auto &stack : boardPart1)
      part1.push_back(stack.back());

    for (auto &stack : boardPart2)
      part2.push_back(stack.back());
  }

  std::chrono::duration<double, std::milli> elapsed_temp = std::chrono::high_resolution_clock::now() - start_temp;
  std::cout << "day05 \t\t\tin " << elapsed_temp.count() << " ms : part1=" << part1 << " \tpart2=" << part2 << std::endl;
  return 0;
}

int main(int argc, char *argv[]) {
  main_speed_iter(argc, argv);
  return 0;
}