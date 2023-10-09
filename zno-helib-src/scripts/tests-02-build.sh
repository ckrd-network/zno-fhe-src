#!/bin/bash
set -exuo pipefail

# Script to build HElib from source and download its dependencies

# Set up temporary build directory
BUILD_DIR="/tmp/helib_build"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Check if HElib source code already exists, if not, clone it
HELIB_SOURCE_DIR="$BUILD_DIR/HElib"
if [ ! -d "$HELIB_SOURCE_DIR" ]; then
  git clone https://github.com/homenc/HElib.git "$HELIB_SOURCE_DIR"
fi

# Check if NTL source code already exists, if not, download and build it
NTL_DIR="$BUILD_DIR/ntl"
if [ ! -d "$NTL_DIR" ]; then
  wget https://libntl.org/ntl-11.4.3.tar.gz
  tar -xf ntl-11.4.3.tar.gz
  mv ntl-11.4.3 "$NTL_DIR"
  cd "$NTL_DIR/src"
  ./configure
  make
fi

# Check if GMP is installed, if not, install it
if ! command -v gmp > /dev/null; then
  sudo apt-get install libgmp-dev
fi

# Check if JSON-C is installed, if not, install it
if ! command -v json-c > /dev/null; then
  sudo apt-get install libjson-c-dev
fi

# Specify NTL include and lib paths
export NTL_INCLUDE_PATH="$NTL_DIR/include"
export NTL_LIB_PATH="$NTL_DIR/src"

# Build HElib
cd "$HELIB_SOURCE_DIR"

mkdir build
cd "$HELIB_SOURCE_DIR/build"

cmake -DPACKAGE_BUILD=ON -DENABLE_TEST=ON -DCMAKE_INSTALL_PREFIX=../ ..

# Run HElib tests
make -j16

ctest

cd ..

# Restore the current directory
cd -
