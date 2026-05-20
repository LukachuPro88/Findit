#include "../../../include/findit/crawler/crawler.hpp"
#include "../../../include/globals.hpp"
#include "../../../include/utils/files.hpp"
#include <filesystem>
#include <regex>
#include <string>
#include <vector>

namespace fs = std::filesystem;

namespace filter {

std::vector<fs::path> dir(std::string start_path, std::string dir_name) {
  std::vector<fs::path> dirs;
  std::vector<fs::path> searched = crawler::traverse_dirs(start_path);
  for (const auto &dir : searched) {
    if (dir.filename() == dir_name) {
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
      files.push_back(file);
    }
  }
  return files;
}

std::vector<std::string> word(std::string file_name, std::string search_word) {
  std::vector<std::string> words;
  std::vector<std::string> content = crawler::open_file(file_name);
  std::regex pattern("\\b" + search_word + "\\b");
  for (int i = 0; i < content.size(); i++) {
    if (std::regex_search(content[i], pattern)) {
      words.push_back("~ " + std::to_string(i + 1) + ": " + content[i]);
    }
  }
  return words;
}

} // namespace filter
