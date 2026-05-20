#pragma once

#include <filesystem>
#include <string>
#include <vector>

namespace fs = std::filesystem;

namespace filter {

std::vector<fs::path> dir(std::string start_path, std::string dir_name);
std::vector<fs::path> file(std::string start_path, std::string file_name);
std::vector<std::string> word(std::string file_name, std::string search_word);
} // namespace filter
