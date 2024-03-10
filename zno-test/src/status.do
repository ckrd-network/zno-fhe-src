#!/usr/bin/env bash
#
# $1 is the target name, eg. pie.init
# $2 is the same as $1.
# $3 is the temporary output file to create.
#    If this script succeeds, redo replaces $1 with $3.
#    If this script fails, redo leaves $1 alone.

# shellcheck shell=bash
exec >&2
# Exit as soon as there is an error.
set -e

find . -type f -name '*.rs' -print0 | xargs --null redo-ifchange

# Run only the unit tests
cargo nextest test --no-fail-fast --lib &>"$3"
# Run the doc tests
cargo test --doc &>>"$3"
