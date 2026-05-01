cmake_minimum_required(VERSION 3.14)
project(Findit)

set(CMAKE_CXX_STANDARD 17)

include_directories(include)

add_executable(crawler
  src/main.cpp
  src/findit/crawler/crawler.cpp
  src/utils/logger.cpp
  src/findit/filter/filter.cpp
)

target_link_libraries(crawler PUBLIC stdc++fs)
