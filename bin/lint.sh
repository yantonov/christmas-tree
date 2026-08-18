#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")"

cd ..

cargo fmt --all -- --check

cargo clippy --all-targets -- -D warnings
