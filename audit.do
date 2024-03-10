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

# FILEPATH: ./audit.do
#
SELF=$(basename "${0##*/}" .do)

# # Scan the binary for vulnerabilities
# cargo audit bin target/release/zno_fhe
# # Build your container image with the binary
# docker build -t test-rust -f - . <<EOF
# FROM scratch
# COPY target/release/your_binary .
# EOF
# # Scan the image
# trivy image --security-checks vuln test-rust
# 