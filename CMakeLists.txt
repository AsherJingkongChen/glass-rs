cmake_minimum_required (VERSION 3.5)
project(glass LANGUAGES CXX)

add_library(glass INTERFACE)
target_include_directories(glass INTERFACE .) 

set(CXX_STANDARD 20)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

SET(CMAKE_CXX_FLAGS " \
  -std=c++20 \
  -Wall -Wextra \
  -g \
  -lrt \
  -lembind \
  -fpic \
  -ftree-vectorize \
")

add_executable(main main.cc)
set_target_properties(main PROPERTIES LINK_FLAGS " \
  -s ENVIRONMENT=web \
  -s FETCH=1 \
  -s WASM=1 \
  -s MODULARIZE=1 \
  -s EXPORT_ES6=1 \
  --oformat=mjs \
")
target_link_libraries(main glass)
