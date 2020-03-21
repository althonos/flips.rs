#!/bin/sh -e

. $(dirname $0)/functions.sh

# --- Test with coverage -----------------------------------------------------

log Measuring code coverage
cargo tarpaulin --release -v --out Xml --ciserver travis-ci
