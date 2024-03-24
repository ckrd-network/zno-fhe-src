#!/usr/bin/env bash
#
# $1 is the target name, eg. pie.init
# $2 is the same as $1.
# $3 is the temporary output file to create.
#    If this script succeeds, redo replaces $1 with $3.
#    If this script fails, redo leaves $1 alone.
#
# References:
# - https://fasterthanli.me/articles/my-ideal-rust-workflow
#

# shellcheck shell=bash
exec >&2
# Exit as soon as there is an error.
set -e

# FILEPATH: ./release.do
#
SELF=$(basename "${0##*/}" .do)

# generate changelog files from the Git history
git-cliff

# generate release.yml, which implements the full pipeline of plan, build, host, publish, announce:
cargo dist init