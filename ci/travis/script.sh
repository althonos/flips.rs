#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Compile without features -----------------------------------------------

log Testing without default features
cargo test --no-default-features

# --- Test with coverage -----------------------------------------------------

log Measuring code coverage
cargo tarpaulin --release -v --out Xml --ciserver travis-ci --all
