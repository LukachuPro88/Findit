cmake_minimum_required(VERSION 3.14)
project(Findit)

set(CMAKE_CXX_STANDARD 17)

include_directories(include)

add_executable(findit
  src/main.cpp
  src/findit/crawler/crawler.cpp
  src/utils/logger.cpp
  src/findit/filter/filter.cpp
  src/cli/cli.cpp
)

target_link_libraries(findit PUBLIC stdc++fs)
