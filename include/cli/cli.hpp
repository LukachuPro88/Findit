#pragma once
#include <string>

namespace cli {

enum class Flag { DIR, FILE, WORD, UNKNOWN };

struct Args {
  Flag flag;
  std::string path;
  std::string word;
  std::string name;
};

Flag parse_flag(const std::string &flag);
Args parse_args(int argc, char *argv[]);
void print_usage();
void run(const Args &args);
} // namespace cli
