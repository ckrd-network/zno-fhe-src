# zno-fhe-src

Source code and FFI binding crates for the ZnO-FHE crate.

This crate contains the logic to build HELib, (TODO: OpenFHE and SEAL).
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

When you're building a Rust application that depends on the ZnO library, it's necessary to ensure that the required HElib static and shared libraries are appropriately located. The application will look for these libraries in three potential directories relative to the executable:

1. `$ORIGIN/../libs`: This is the directory one level above the binary and then in the `libs` folder.
2. `$ORIGIN/libs`: This is the `libs` directory at the same level as the binary.
3. `$ORIGIN`: This is the directory where the binary is located.

### Example: ZnO Integration Tests

For ZnO's integration test binaries, the libraries are arranged as follows:

- The test binaries are located in the `target/[target-triple]/debug/deps` directory.
- The required libraries are placed within the `libs` directory at the root of the project. This corresponds to the `$ORIGIN/libs` location mentioned above, as `$ORIGIN` refers to the directory containing the test binary, which is `target/[target-triple]/debug/deps` in this case.

This setup ensures that the test binaries can successfully locate and link to the required libraries during execution.

## Tests

Tests are front and centre. We follow advice on their setup:

- [Delete Cargo Integration Tests](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html) these tips also reduce compile time:

  - > Large projects should have only one integration test crate with several modules. A nice side-effect of a single modularized integration test is that sharing the code between separate tests becomes trivial, you just pull it into a submodule. There’s no need to awkwardly repeat mod common; for each integration test.
  - For a public API on crates.io, avoid unit tests. Use a single integration test,  `tests/it.rs` or `tests/it/main.rs`. Integration tests use `it` library as an external crate. Using the public API results in better API design feedback.

## FHE Libraries

### HElib

To compile the source code and move artifacts to the system crate:

```shell
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/zno-helib-src-test"

RUST_BACKTRACE=1 cargo build --bin package --features "static package" --manifest-path "$src_dir/Cargo.toml" --target $target -vvv 2>&1 | tee cargo-build-src-test.txt
```

To generate the FFI bindings in the system crate

```shell
target=x86_64-unknown-linux-gnu
sys_dir="$(pwd)/zno-helib-sys"

RUST_BACKTRACE=1 cargo build --lib --manifest-path "$sys_dir/Cargo.toml" --target $target -vvv 2>&1 | tee cargo-build-sys.txt
cargo doc --open --document-private-items --manifest-path "$sys_dir/Cargo.toml" --target $target
cargo expand --manifest-path "$sys_dir/Cargo.toml" --target $target -- --nocapture 2>&1 | tee cargo-expand-sys.txt

# export LD_LIBRARY_PATH="$(pwd)/zno-helib-sys/src/helib_pack/lib:$LD_LIBRARY_PATH"
cargo test --test ffi-context-bgv --features "static helib" --manifest-path "$sys_dir/Cargo.toml" --target $target --verbose -- --nocapture 2>&1 | tee cargo-test-sys.txt
```

#### Test

```shell
target=x86_64-unknown-linux-gnu
test_dir="$(pwd)/zno-helib-src-test"

set -ex

cargo test --manifest-path "$test_dir/Cargo.toml" --target $target -vvv &>log-test.txt
cargo test --manifest-path "$test_dir/Cargo.toml" --target $target -vvv --release &>log-test-release.txt

set +ex

```

## Versioning

This crate follows the minor and patch versions for each maintained major version, according to:

- The HELib release strategy.
- The OpenFHE release strategy.
- The SEAL release strategy.

The crate versions follow the X.Y.Z+B pattern:

- The major version X is the upstream API/ABI compatibility version:
        3YZ for 3.Y.Z
- The minor Y and patch Z versions are incremented when making changes to the crate, either upstream update or internal changes.
- `B` contains the full upstream version, like 1.1.1k or 3.0.7. Note that this field is actually ignored in comparisons and only there for documentation.

## Upstream Sources

### System requirements

Install static libraries for `libstdc++`

```shell
apt search libstdc++ | grep dev
```

### HELib

```shell
git subrepo clone https://github.com/homenc/HElib.git zno-helib-src/helib --branch=v2.3.0 --method=rebase
```

## Parameter Selection

Install SageMath. Run script.

```shell
curl -L -O https://github.com/conda-forge/miniforge/releases/latest/download/Mambaforge-$(uname)-$(uname -m).sh
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

## Maintenance

```shell
cargo install cargo-edit
cargo upgrade --dry-run --verbose &>>cargo-upgrade.txt
cargo upgrade
# Check release notes for crates dry-run said are incompatible.
cargo upgrade --incompatible
cargo tree &>>cargo-tree.txt
```

## References

[Parameter selection repository](https://github.com/Crypto-TII/fhegen)

```latex
@misc{cryptoeprint:2022/706,
      author = {Johannes Mono and Chiara Marcolla and Georg Land and Tim Güneysu and Najwa Aaraj},
      title = {Finding and Evaluating Parameters for {BGV}},
      howpublished = {Cryptology ePrint Archive, Paper 2022/706},
      year = {2022},
      note = {\url{https://eprint.iacr.org/2022/706}},
      url = {https://eprint.iacr.org/2022/706}
}
```

```latex
@misc{cryptoeprint:2023/600,
      author = {Beatrice Biasioli and Chiara Marcolla and Marco Calderini and Johannes Mono},
      title = {Improving and Automating BFV Parameters Selection: An Average-Case Approach},
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
