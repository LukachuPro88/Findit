#include <filesystem>
#include <string>
#include <vector>

namespace fs = std::filesystem;

namespace crawler {

std::vector<fs::path> traverse_files(std::string start_path);
std::vector<fs::path> traverse_dirs(std::string start_path);
std::vector<std::string> open_file(const std::string &file_name);
} // namespace crawler
