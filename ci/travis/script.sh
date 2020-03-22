#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Compile without features -----------------------------------------------

log Compiling without default features
cargo check --no-default-features

# --- Test with coverage -----------------------------------------------------

log Measuring code coverage
cargo tarpaulin --release -v --out Xml --ciserver travis-ci --all
