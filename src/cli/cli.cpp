#include "../../include/cli/cli.hpp"
#include "../../include/findit/filter/filter.hpp"
#include "../../include/globals.hpp"
#include "../../include/utils/logger.hpp"
#include <filesystem>
#include <iostream>

namespace fs = std::filesystem;

namespace cli {

Flag parse_flag(const std::string &flag) {
  if (flag == "--dir")
    return Flag::DIR;
  if (flag == "--file")
    return Flag::FILE;
  if (flag == "--word")
    return Flag::WORD;
  return Flag::UNKNOWN;
}

Args parse_args(int argc, char *argv[]) {
  Args args;

  if (argc < 3) {
    print_usage();
    exit(1);
  }

  args.flag = parse_flag(argv[1]);
  args.path = argv[2];

  if (args.flag == Flag::WORD) {
    if (argc < 4) {
      logger::error("--word requires two arguments: <file_path> <word>");
      print_usage();
      exit(1);
    }
    args.word = argv[3];
  }

  if (args.flag == Flag::DIR || args.flag == Flag::FILE) {
    if (argc < 4) {
      logger::error(
          "--dir / --file requires two arguments: <start_path> <name>");
      print_usage();
      exit(1);
    }
    args.name = argv[3];
  }

  if (args.flag == Flag::UNKNOWN) {
    logger::error("Unknown flag: " + std::string(argv[1]));
    print_usage();
    exit(1);
  }

  return args;
}

void print_usage() {
  std::cout
      << color::BOLD << "Usage:" << color::RESET << "\n"
      << "  findit --dir  <start_path> <name>              Search for a "
         "directory\n"
      << "  findit --file <start_path> <name>              Search for a file\n"
      << "  findit --word <file_path> <word>       Search for a word in a "
         "file\n";
}

void run(const Args &args) {
  fs::path input = args.path;

  switch (args.flag) {

  case Flag::DIR: {
    auto results = filter::dir(args.path, args.name);
    if (!results.empty()) {
      for (const auto &r : results)
        logger::success("Directory found at: " + r.string());
    } else {
      logger::error("Directory '" + args.name + "' not found.");
    }
    break;
  }

  case Flag::FILE: {
    auto results = filter::file(args.path, args.name);
    if (!results.empty()) {
      for (const auto &r : results)
        logger::success("File found at: " + r.string());
    } else {
      logger::error("File '" + args.name + "' not found.");
    }
    break;
  }

  case Flag::WORD: {
    auto results = filter::word(input.string(), args.word);

    if (!results.empty()) {
      logger::success("Word '" + args.word + "' found:");
      for (const auto &line : results)
        std::cout << "  " << color::YELLOW << line << color::RESET << "\n";
    } else {
      logger::error("Word '" + args.word + "' not found.");
    }
    break;
  }

  default:
    break;
  }
}

} // namespace cli
