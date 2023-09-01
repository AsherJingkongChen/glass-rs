#include <emscripten/bind.h>

#include "glass/builder.hpp"
#include "glass/hnsw/hnsw.hpp"
#include "glass/searcher.hpp"

using namespace emscripten;
const int efC = 42;
EMSCRIPTEN_BINDINGS(uwu) {
  constant("efC", efC);
}
