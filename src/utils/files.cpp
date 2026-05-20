#include "../../include/utils/logger.hpp"
#include <fstream>
#include <string>
#include <vector>

namespace fileIO {

std::vector<std::string> read_file(const std::string &file_name) {
  std::vector<std::string> contents;

  std::ifstream inFile(file_name);
  if (!inFile) {
    logger::error("Error opening file '" + file_name + "'");
    return contents;
  }

  std::string line;
  while (std::getline(inFile, line)) {
    contents.push_back(std::move(line));
  }

  return contents;
}
} // namespace fileIO
