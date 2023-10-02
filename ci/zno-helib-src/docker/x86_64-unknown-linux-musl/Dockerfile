FROM ubuntu:18.04

# Install general HElib pre-requisites
RUN apt update && \
    apt dist-upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get update -y && apt-get install -y --no-install-recommends \
      build-essential  \
      ca-certificates \
      cmake  \
      diffutils  \
      gcc \
      git  \
      libc6-dev \
      libcurl4-openssl-dev  \
      m4  \
      make \
      musl-tools \
      parallel \
      patchelf \
      perl \
      python3  \
      wget  \
      zlib1g-dev

# Install bats-core (used for testing utils and examples)
RUN cd && \
    wget https://github.com/bats-core/bats-core/archive/v1.2.1.tar.gz && \
    tar xf v1.2.1.tar.gz && \
    cd bats-core-1.2.1 && \
    ./install.sh /usr/local

# Install Google Benchmarks (used for HElib benchmarks)
RUN cd && \
    wget https://github.com/google/benchmark/archive/v1.5.2.tar.gz && \
    tar xf v1.5.2.tar.gz && \
    cd benchmark-1.5.2 && \
    mkdir build && \
    cd build && \
    cmake -DBENCHMARK_ENABLE_GTEST_TESTS=OFF -DCMAKE_BUILD_TYPE=Release .. && \
    make -j4 && \
    make install

# Install NTL and GMP
RUN apt install -y libgmp-dev libntl-dev

# Script for building and testing HElib
COPY build_scripts/build_and_test_helib.sh /root

# Default script - Build and test HElib and all subprojects using package build
CMD ["/root/build_and_test_helib.sh", "-a"]
