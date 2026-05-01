#include "../../../include/utils/logger.hpp"
#include <exception>
#include <filesystem>
#include <vector>

namespace fs = std::filesystem;

namespace crawler {

std::vector<fs::path> traverse_files(std::string start_path) {
  std::vector<fs::path> files;

  try {
    if (!fs::exists(start_path)) {
      logger::error("Path does not exist: " + start_path);
      return {};
    }

    for (const auto &entry : fs::recursive_directory_iterator(start_path)) {
      // Check if entry is a regular file
      if (entry.is_regular_file()) {
        files.push_back(entry.path());
        logger::info("Regular file found: " + entry.path().string());
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

    for (const auto &entry : fs::recursive_directory_iterator(start_path)) {
      // Check if entry is a regular file
      if (entry.is_directory()) {
        dirs.push_back(entry.path());
        logger::info("Regular file found: " + entry.path().string());
      }
    }
  } catch (const fs::filesystem_error &e) {
    logger::error(std::string("Filesystem error: ") + e.what());
  } catch (const std::exception &e) {
    logger::error(std::string("Unexpected error: ") + e.what());
  }
  return dirs;
}
} // namespace crawler
