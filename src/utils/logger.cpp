#include "../../include/utils/logger.hpp"
#include "../../include/globals.hpp"

#include <iostream>

namespace logger {

void error(const std::string &msg) {
  if (current_level > Level::ERROR)
    return;
  std::cout << color::RED << "[ERROR]: " << msg << color::RESET << '\n';
}

void debug(const std::string &msg) {
  if (current_level > Level::DEBUG)
    std::cout << color::BLUE << "[DEBUG]: " << msg << color::RESET << '\n';
}

void success(const std::string &msg) {
  if (current_level > Level::SUCCESS)
    return;
  std::cout << color::GREEN << "[SUCCESS]: " << msg << color::RESET << '\n';
}

void info(const std::string &msg) {
  if (current_level > Level::INFO)
    return;
  std::cout << color::YELLOW << "[INFO]: " << msg << color::RESET << '\n';
}
} // namespace logger
