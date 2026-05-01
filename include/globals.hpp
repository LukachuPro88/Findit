#pragma once

#include <string>

namespace logger {

enum class Level { DEBUG, INFO, SUCCESS, ERROR, NONE };

inline Level current_level = Level::DEBUG;
inline Level previous_level = Level::DEBUG;

inline void toggle() {
  if (current_level == Level::NONE) {
    current_level = previous_level; // restore
  } else {
    previous_level = current_level; // save before muting
    current_level = Level::NONE;
  }
}

inline void set_level(Level level) {
  previous_level = current_level;
  current_level = level;
}
}; // namespace logger

// Provides ANSI escape codes for text color
namespace color {

inline const std::string RED = "\033[31m";
inline const std::string GREEN = "\033[32m";
inline const std::string YELLOW = "\033[33m";
inline const std::string BLUE = "\033[34m";

inline const std::string RESET = "\033[0m";

}; // namespace color
