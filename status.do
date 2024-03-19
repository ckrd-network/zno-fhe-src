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
set -ex

readonly SELF=$(basename "${0##*/}" .do)
readonly DIR=$(pwd)

find . -type f -name '*.rs' -print0 | xargs --null redo-ifchange
find . -type f -name '*.toml' -print0 | xargs --null redo-ifchange

# Even if a *.rs file has changed (above), a redo script that monitors this,
# will see a change only if there has been a change across results.
cat benchmarks examples src tests | redo-stamp
