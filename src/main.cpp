#include "../include/cli/cli.hpp"
#include "../include/globals.hpp"

int main(int argc, char *argv[]) {
  logger::set_level(logger::Level::INFO);
  cli::run(cli::parse_args(argc, argv));
  return 0;
}
