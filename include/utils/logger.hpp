#pragma once

// pargma: keep -- Clang worrying about nothing
#include <iostream> // IWYU pragma: keep

namespace logger {

void error(const std::string &msg);
void debug(const std::string &msg);
void success(const std::string &msg);
void info(const std::string &msg);
} // namespace logger
