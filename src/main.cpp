#include "../include/findit/filter/filter.hpp"
#include "../include/utils/logger.hpp"
#include <filesystem>
#include <string>
#include <vector>

int main() {
  logger::debug("Program started");

  logger::debug("Search for directory 'Findit' started");
  std::vector<fs::path> dir = filter::dir("/home/js/Projects", "Findit");

  std::vector<fs::path> file = filter::file("/home/js/Projects", "main.cpp");

  if (!dir.empty()) {
    logger::success(std::string("Directory found at path: ") + dir[0].string());
  } else {
    logger::error("Directory 'Findit' not found.");
  }

  if (!file.empty()) {
    logger::success(std::string("File found at path: ") + file[0].string());
  } else {
    logger::error("File 'main.cpp' not found.");
  }

  return 0;
}
