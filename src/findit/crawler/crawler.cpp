#include "../../../include/globals.hpp"
#include "../../../include/utils/files.hpp"
#include "../../../include/utils/logger.hpp"
#include <exception>
#include <filesystem>
#include <fstream>
#include <string>
#include <vector>

namespace fs = std::filesystem;

static bool should_ignore(const fs::path &path) {
  std::vector<std::string> contents =
      fileIO::read_file(config::ignore_file_path);

  for (size_t i = 0; i < contents.size(); i++) {
    if (path.filename().string() == contents[i]) {
      return true;
    }
  }

  return false;
}

namespace crawler {

std::vector<fs::path> traverse_files(std::string start_path) {
  std::vector<fs::path> files;
  try {
    if (!fs::exists(start_path)) {
      logger::error("Path does not exist: " + start_path);
      return {};
    }

    for (auto it = fs::recursive_directory_iterator(start_path);
         it != fs::recursive_directory_iterator(); ++it) {
      if (should_ignore(it->path())) {
        if (it->is_directory()) {
          it.disable_recursion_pending();
        }
        continue;
      }

      if (it->is_regular_file()) {
        files.push_back(it->path());
      }
    }
  } catch (const fs::filesystem_error &e) {
    logger::error(std::string("Filesystem error: ") + e.what());
  } catch (const std::exception &e) {
    logger::error(std::string("Unexpected error: ") + e.what());
  }
  return files;
}

std::vector<fs::path> traverse_dirs(std::string start_path) {
  std::vector<fs::path> dirs;
  try {
    if (!fs::exists(start_path)) {
      logger::error("Path does not exist: " + start_path);
      return {};
    }

    for (auto it = fs::recursive_directory_iterator(start_path);
         it != fs::recursive_directory_iterator(); ++it) {
      if (should_ignore(it->path())) {
        if (it->is_directory()) {
          it.disable_recursion_pending();
        }
        continue;
      }

      if (it->is_directory()) {
        dirs.push_back(it->path());
      }
    }
  } catch (const fs::filesystem_error &e) {
    logger::error(std::string("Filesystem error: ") + e.what());
  } catch (const std::exception &e) {
    logger::error(std::string("Unexpected error: ") + e.what());
  }
  return dirs;
}

std::vector<std::string> open_file(const std::string &file_name) {
  if (should_ignore(file_name)) {
    return {};
  }

  std::ifstream file(file_name);
  std::string line;
  std::vector<std::string> content;
  if (!file.is_open()) {
    logger::error("Could not open file: " + file_name);
    return {};
  }
  while (std::getline(file, line)) {
    content.push_back(line);
  }
  return content;
}
} // namespace crawler
