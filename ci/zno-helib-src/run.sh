#!/bin/sh
target=$1
test_dir="$(pwd)/zno-helib-src-test"

set -ex

export CARGO_TARGET_AARCH64_APPLE_DARWIN_RUNNER=echo
export CARGO_TARGET_WASM32_WASI_RUNNER=wasmtime

cargo test --manifest-path "$test_dir/Cargo.toml" --target "$target" -vv
cargo test --manifest-path "$test_dir/Cargo.toml" --target "$target" -vv --release

if [ "$target" = "x86_64-unknown-linux-gnu" ] ; then
  cargo test --manifest-path "$test_dir/Cargo.toml" --target $target -vvv --all-features

  # Run a few tests here:
  #
  # * Make sure the packaged crate file isn't bigger than 10MB which is
  #   crate.io's limit.
  # * Make sure that the package crate itself works.
  #
  # HElib's source code can be excluded on crates.io if it makes the
  # crate file too large. The test here should inform us if we're
  # missing anything actually required to build HElib.
  rm -rf target/ci
  cargo package --allow-dirty --target-dir target/ci
  crate=$(ls target/ci/package/*.crate)
  filesize=$(stat -c%s "$crate")
  echo "tarball is $filesize bytes"
  if (( filesize > 10000000 )); then
    echo "file size too big"
    exit 1
  fi
  rm "$crate"
  cd target/ci/package/helib-src-*
  cp -r "$test_dir" .
  cargo test --manifest-path "$test_dir/Cargo.toml" --target "$target" -vv
fi
