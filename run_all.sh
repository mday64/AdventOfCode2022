#!/bin/zsh

run_day () {
    echo "== Day $1 =="
    time $2
    echo
}
cargo build --release
for x in 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25; do
    run_day $x ./target/release/day$x
done
