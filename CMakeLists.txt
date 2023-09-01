cmake_minimum_required(VERSION 3.18)

project(glass LANGUAGES CXX)

set(CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# build/glass.js
add_executable(glass glass.cc)
set_target_properties(glass PROPERTIES CXX_STANDARD 20)
set_target_properties(glass PROPERTIES SUFFIX .js)
target_compile_options(glass PUBLIC
  -g
  -fPIC
  -Wall -Wextra
)
target_include_directories(glass PUBLIC .)
target_link_libraries(glass PUBLIC embind)
target_link_options(glass PUBLIC
  -sALLOW_MEMORY_GROWTH
  -sENVIRONMENT=web
  -sEXPORT_NAME=Glass
  -sFETCH
  -sMODULARIZE
  -sNO_DISABLE_EXCEPTION_CATCHING
  -sWASM
  --oformat=mjs
)

# build/glass.d.ts
add_custom_command(
  TARGET glass
  POST_BUILD
  COMMAND
    ${CMAKE_CXX_COMPILER}
    -Wall -Wextra
    -lembind
    -I${CMAKE_SOURCE_DIR}
    --embind-emit-tsd=${CMAKE_BINARY_DIR}/glass.d.ts
    -sEXPORT_NAME=Glass
    -sNO_EXPORT_ES6
    -sNO_WARN_ON_UNDEFINED_SYMBOLS
    ${CMAKE_SOURCE_DIR}/glass.cc
)
