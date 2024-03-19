# zno-fhe

Source code and system (FFI) crates for the ZnO-FHE crate.

This crate contains the logic to build HELib and SEAL (TODO: OpenFHE).
It is intended to be consumed by the `zno-fhe` crate.
In theory you should not need to interact with this repository.

Add these features to the FHE crate that exposes the libraries:

```toml
# Build and test all of the above.
all = ['tests', 'examples', 'utils', 'benchmarks']
# Build the HElib Google benchmark directory.
benchmarks = []
# Build and test the HElib examples directory.
examples = []
# Run the HElib Google tests.
tests = []
# Build and test the HElib utils directory.
utils = []
```

## Setting Up HElib Libraries for End-User Rust Binaries Using ZnO

When you're building a Rust application that depends on the ZnO library, it's
necessary to ensure that the required HElib static and shared libraries are
appropriately located. The application will look for these libraries in three
potential directories relative to the executable:

1. `$ORIGIN/../libs`: This is the directory one level above the binary and
   then in the `libs` folder.
2. `$ORIGIN/libs`: This is the `libs` directory at the same level as the binary.
3. `$ORIGIN`: This is the directory where the binary is located.

### Example: ZnO Integration Tests

For ZnO's integration test binaries, the libraries are arranged as follows:

- The test binaries are located in the `target/[target-triple]/debug/deps` directory.
- The required libraries are placed within the `libs` directory at the root of
  the project. This corresponds to the `$ORIGIN/libs` location mentioned
  above, as `$ORIGIN` refers to the directory containing the test binary,
  which is `target/[target-triple]/debug/deps` in this case.

This setup ensures that the test binaries can successfully locate and link to
the required libraries during execution.

## Tests

Tests are front and centre. We follow advice on their setup:

- [Delete Cargo Integration Tests][l001] these tips also reduce compile time:

  - > Large projects should have only one integration test crate with several
      modules. A nice side-effect of a single modularized integration test is
      that sharing the code between separate tests becomes trivial, you just
      pull it into a submodule. There’s no need to awkwardly repeat
      `mod common;` for each integration test.
  - For a public API on crates.io, avoid unit tests. Use a single integration
    test,  `tests/it.rs` or `tests/it/main.rs`. Integration tests use
    `it` library as an external crate. Using the public API results in
    better API design feedback.

## FHE Libraries

### HElib

To compile the source code and move artifacts to the system crate:

```shell
library=helib
crate_name="zno-${library}-src-test"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo build --lib --manifest-path "$src_dir/Cargo.toml" --target $target \
            --features "static package" --verbose 2>&1 \
            | tee cargo-build-${crate_name}.txt
```

#### FFI bindings

To generate the FFI bindings in the system crate:

```shell
library=helib
crate_name="zno-${library}-src-test"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo build --lib --manifest-path "$src_dir/Cargo.toml" --target $target \
            --verbose 2>&1 | tee cargo-build-${crate_name}.txt
```

Generate documentation:

```shell
library=helib
crate_name="zno-${library}-sys"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"

cargo doc --open --document-private-items --target $target \
          --manifest-path "$src_dir/Cargo.toml"
cargo expand --manifest-path "$src_dir/Cargo.toml" --target $target \
             -- --nocapture 2>&1 | tee cargo-expand-${crate_name}.txt
```

#### Tests

List and run tests using clang ([issue #681](https://github.com/microsoft/SEAL/issues/681)):

```shell
library=helib
crate_name="zno-${library}-sys"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"

cargo test --features "static helib" --target $target \
           --manifest-path "$src_dir/Cargo.toml" -- --list \
           | tee cargo-test-list-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --lib bgv::m:: --features "static helib" --target $target \
           --manifest-path "$src_dir/Cargo.toml" --verbose -- \
           bgv::context::tests::test_get_m_zero --exact --nocapture 2>&1 | \
           tee cargo-unit-test-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --lib --features "static helib" --target $target --verbose \
           --manifest-path $src_dir/Cargo.toml -- --nocapture 2>&1 \
           | tee cargo-unit-test-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test bgv::bits::tests::test_new_usize_isize_arch_dependent \
           --lib --features "static helib" --target $target --verbose \
           --manifest-path "$src_dir/Cargo.toml"  -- --nocapture 2>&1 \
           | tee cargo-unit-test-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --test ffi-context-bgv --features "static helib" --target $target \
           --verbose --manifest-path "$src_dir/Cargo.toml" -- --nocapture 2>&1 \
           | tee cargo-unit-test-${crate_name}.txt
```

#### Source

```shell
library=helib
crate_name="zno-${library}-src-test"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"

set -ex

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --manifest-path "$src_dir/Cargo.toml" --target $target --verbose \
           -- --nocapture 2>&1 | tee cargo-test-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --manifest-path "$src_dir/Cargo.toml" --target $target --verbose \
           --release -- --nocapture 2>&1 | tee cargo-test-${crate_name}-release.txt

set +ex

```

### SEAL

SEAL compiled with Clang++ has much better runtime performance than when
compiled with GNU G++.

To compile the source code and move artifacts to the system crate:

```shell
library=seal
crate_name="zno-${library}-src-test"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo build --lib --manifest-path "$src_dir/Cargo.toml" --target $target \
            --verbose 2>&1 | tee cargo-build-${crate_name}.txt
```

#### Tests

List and run tests using clang ([issue #681](https://github.com/microsoft/SEAL/issues/681)):

```shell
library=seal
crate_name="zno-${library}-sys"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"

cargo test --features "static helib" --target $target \
           --manifest-path "$src_dir/Cargo.toml" -- --list 2>&1 \
           | tee cargo-test-list-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --lib bgv::m:: --features "static helib" --target $target \
           --manifest-path "$src_dir/Cargo.toml" --verbose -- \
           bgv::context::tests::test_get_m_zero --exact --nocapture 2>&1 | \
           tee cargo-unit-test-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --lib --features "static helib" --target $target --verbose \
           --manifest-path $src_dir/Cargo.toml -- --nocapture 2>&1 \
           | tee cargo-unit-test-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test bgv::bits::tests::test_new_usize_isize_arch_dependent \
           --lib --features "static helib" --target $target --verbose \
           --manifest-path "$src_dir/Cargo.toml"  -- --nocapture 2>&1 \
           | tee cargo-unit-test-${crate_name}.txt

CXX=/usr/bin/clang++ RUST_BACKTRACE=full \
cargo test --test ffi-context-bgv --features "static helib" --target $target \
           --verbose --manifest-path "$src_dir/Cargo.toml" -- --nocapture 2>&1 \
           | tee cargo-unit-test-${crate_name}.txt
```

#### Source (SEAL)

```shell
library=seal
crate_name="zno-${library}-src-test"
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/${crate_name}"


set -ex

cargo test --manifest-path "$src_dir/Cargo.toml" --target $target --verbose \
           -- --nocapture 2>&1 | tee cargo-${crate_name}.txt

cargo test --manifest-path "$src_dir/Cargo.toml" --target $target --verbose \
           --release -- --nocapture 2>&1 | tee cargo-${crate_name}-release.txt

set +ex

```

## Versioning

This crate follows the minor and patch versions for each maintained major
version, according to:

- The HELib release strategy.
- The OpenFHE release strategy.
- The SEAL release strategy.

The crate versions follow the X.Y.Z+B pattern:

- The major version X is the upstream API/ABI compatibility version:
        3YZ for 3.Y.Z
- The minor Y and patch Z versions are incremented when making changes to the
  crate, either upstream update or internal changes.
- `B` contains the full upstream version, like 1.1.1k or 3.0.7. Note that this
  field is actually ignored in comparisons and only there for documentation.

## Upstream Sources

### System requirements

Install static libraries for `libstdc++`

```shell
apt search libstdc++ | grep dev
```

### HELib

```shell
git subrepo clone https://github.com/homenc/HElib.git zno-helib-src/helib \
                  --branch=v2.3.0 --method=rebase
```

## Parameter Selection

Install SageMath. Run script.

```shell
    <!-- markdownlint-disable MD013 -->
curl -L -O https://github.com/conda-forge/miniforge/releases/latest/download/Mambaforge-$(uname)-$(uname -m).sh
    <!-- markdownlint-enable MD013 -->
sh Mambaforge-$(uname)-$(uname -m).sh
mamba create -n sage sage python=3.11
pushd ~/src
  git clone https://github.com/Crypto-TII/fhegen.git
popd
~/mambaforge/envs/sage/bin/sage --python ~/src/fhegen/src/interactive.py
```

## Contributing

- [Fast Rust Builds](https://matklad.github.io/2021/09/04/fast-rust-builds.html)
- [Optimize Rust Compile Time](https://rustmagazine.org/issue-2/optimize-rust-comptime/)

## Development

The build tool is [`redo`](https://github.com/apenwarr/redo).

## Maintenance

https://blog.logrocket.com/comparing-rust-supply-chain-safety-tools/

```shell
cargo install cargo-edit
cargo upgrade --dry-run --verbose &>>cargo-upgrade.txt
cargo upgrade
# Check release notes for crates dry-run said are incompatible.
cargo upgrade --incompatible
cargo tree &>>cargo-tree.txt
```

Checking MSRV

```shell
podman pull docker.io/foresterre/cargo-msrv
podman run -t -v "$(pwd)/Cargo.toml":/app/Cargo.toml docker.io/foresterre/cargo-msrv
```

## References

[HElib](https://github.com/homenc/HElib)

```latex
@misc{helib,
    title = {{HELib}  (release 2.3.0)},
    howpublished = {\url{https://github.com/homenc/HElib}},
    month = July,
    year = 2023,
    note = {IBM Corp.},
    key = {HElib}
}
@misc{cryptoeprint:2020/1481,
      author = {Shai Halevi and Victor Shoup},
      title = {Design and implementation of HElib: a homomorphic encryption library},
      howpublished = {Cryptology ePrint Archive, Paper 2020/1481},
      year = {2020},
      note = {\url{https://eprint.iacr.org/2020/1481}},
      url = {https://eprint.iacr.org/2020/1481}
}
```

[SEAL](https://github.com/Microsoft/SEAL)

```latex
@misc{sealcrypto,
    title = {{M}icrosoft {SEAL} (release 4.1)},
    howpublished = {\url{https://github.com/Microsoft/SEAL}},
    month = jan,
    year = 2023,
    note = {Microsoft Research, Redmond, WA.},
    key = {SEAL}
}
```

[Parameter selection repository](https://github.com/Crypto-TII/fhegen)

```latex
@misc{cryptoeprint:2022/706,
      author = {
        <!-- cSpell:disable -->
          <!-- markdownlint-disable MD013 -->
                Johannes Mono and Chiara Marcolla and Georg Land and Tim Güneysu and Najwa Aaraj
          <!-- markdownlint-enable MD013 -->
        <!-- cSpell:enable -->
      },
      title = {Finding and Evaluating Parameters for {BGV}},
      howpublished = {Cryptology ePrint Archive, Paper 2022/706},
      year = {2022},
      note = {\url{https://eprint.iacr.org/2022/706}},
      url = {https://eprint.iacr.org/2022/706}
}
```

```latex
@misc{cryptoeprint:2023/600,
      <!-- markdownlint-disable MD013 -->
      author = {Beatrice Biasioli and Chiara Marcolla and Marco Calderini and Johannes Mono},
      title = {Improving and Automating BFV Parameters Selection: An Average-Case Approach},
      <!-- markdownlint-enable MD013 -->
      howpublished = {Cryptology ePrint Archive, Paper 2023/600},
      year = {2023},
      note = {\url{https://eprint.iacr.org/2023/600}},
      url = {https://eprint.iacr.org/2023/600}
}
```

[Comparison repository](https://github.com/iliailia/comparison-circuit-over-fq)

```latex
@misc{cryptoeprint:2021/315,
      author = {Ilia Iliashenko and Vincent Zucca},
      title = {Faster homomorphic comparison operations for BGV and BFV},
      howpublished = {Cryptology ePrint Archive, Paper 2021/315},
      year = {2021},
      note = {\url{https://eprint.iacr.org/2021/315}},
      url = {https://eprint.iacr.org/2021/315}
}
```

```latex
@article{Koki Morimura2023,
  title={Accelerating Polynomial Evaluation for Integer-wise Homomorphic Comparison and Division},
  author={Koki Morimura and Daisuke Maeda and Takashi Nishide},
  journal={Journal of Information Processing},
  volume={31},
  number={ },
  pages={288-298},
  year={2023},
  doi={10.2197/ipsjjip.31.288}
}
```

[l001]: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html
