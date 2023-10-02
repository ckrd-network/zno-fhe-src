#!/bin/sh

target=$1

if [ ! -d ci/docker/$1 ]; then
  exec ci/zno-helib-src/run.sh $1
fi

set -ex

docker build \
  --rm \
  --tag ci-zno-helib-src \
  ci/zno-helib-src/docker/$1

docker run \
  --rm \
  --volume "$(rustc --print sysroot)":/rust:ro \
  --volume "$(pwd)":/usr/code:ro \
  --volume "$(pwd)"/target:/usr/code/target \
  --volume $HOME/.cargo:/cargo \
  --env CARGO_HOME=/cargo \
  --workdir /usr/code \
  ci-zno-helib-src \
  bash -c "PATH=\$PATH:/rust/bin ci/zno-helib-src/run.sh $target"
