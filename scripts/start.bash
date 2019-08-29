#!/usr/bin/env bash

main() {
  ./node_modules/.bin/webpack -dw &
  local webpack_pid=$!

  node server.js
  kill $webpack_pid
}


main
