# zno-fhe-src

Source code crates for the ZnO-FHE crate.

This crate contains the logic to build HELib, OpenFHE and SEAL.
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

## FHE Libraries

### HElib

```shell
target=x86_64-unknown-linux-gnu
src_dir="$(pwd)/zno-helib-src-test"
sys_dir="$(pwd)/zno-helib-sys-test"

cargo build --manifest-path "$src_dir/Cargo.toml" --target $target -vvv &>log-src.txt
cargo build --manifest-path "$sys_dir/Cargo.toml" --feature "vendored" --target $target -vvv &>log-sys.txt

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
