#!/bin/sh

clear

DAY=${1:-XX}
BIN="day${DAY}"

cargo run --bin "${BIN}" > "./out/${BIN}.out"
