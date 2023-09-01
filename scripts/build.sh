#! /usr/bin/env sh

emcmake cmake -B./build/web/ .;
emmake make --directory=./build/web/;
echo '{
  "name": "glass",
  "version": "0.0.0",
  "type": "module",
  "module": "glass.js",
  "types": "glass.d.ts",
}' > build/web/package.json;