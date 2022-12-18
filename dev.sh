#!/bin/sh

clear

DAY=${1:-XX}
BIN="day${DAY}"

export RUST_BACKTRACE=1

cargo watch -c -w ./src -x "run --bin ${BIN}"
