#!/bin/sh

clear

DAY=${1:-XX}
BIN="day${DAY}"

cargo watch -c -w ./src -x "run --bin ${BIN}"
