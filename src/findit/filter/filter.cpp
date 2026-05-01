#include "../../../include/findit/crawler/crawler.hpp"
#include "../../../include/utils/logger.hpp"
#include <filesystem>
#include <string>
#include <vector>

namespace fs = std::filesystem;

namespace filter {

std::vector<fs::path> dir(std::string start_path, std::string dir_name) {
  std::vector<fs::path> dirs;

  std::vector<fs::path> searched = crawler::traverse_dirs(start_path);

  for (const auto &dir : searched) {
    if (dir.filename() == dir_name) {
      logger::info(std::string("Directory with name '") + dir_name +
                   "' found at path: " + dir.string());
      dirs.push_back(dir);
    }
  }

  return dirs;
}

std::vector<fs::path> file(std::string start_path, std::string file_name) {
  std::vector<fs::path> files;

  std::vector<fs::path> searched = crawler::traverse_files(start_path);

  for (const auto &file : searched) {
    if (file.filename() == file_name) {
      logger::info(std::string("File with name '") + file_name +
                   "' found at path: " + file.string());
      files.push_back(file);
    }
  }

  return files;
}
} // namespace filter
