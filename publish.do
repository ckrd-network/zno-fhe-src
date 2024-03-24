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

# FILEPATH: ./release.do
#
SELF=$(basename "${0##*/}" .do)

# if a crate foo depends on bar, a version of bar satisfying
# foo’s requirement must be available in crates.io before
# publishing foo

# dev-dependencies may introduce cyclic dependencies.
# Take the example again that the crate foo depends on bar.
# This time the test cases of bar depend on foo.
# Cargo can resolve these cyclic dependencies because it does not
# need the dev-dependencies to build both foo and bar,
# so it can build bar, foo, and then bar test cases in order.

But cargo publish requires both dependencies and dev-dependencies are available in crates.io, now cyclic dependencies will cause problems.

cargo machete

cargo publish --dry-run
