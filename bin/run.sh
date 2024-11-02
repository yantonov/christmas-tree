#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")"

cd ..

cargo run show $@
