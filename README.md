# zink-fhe-src

Source code crate for the Zink-FHE system crate.

This crate contains the logic to build HELib, OpenFHE and SEAL.
It is intended to be consumed by the `zink-fhe-sys` crate.
In theory you aren't interacting with this too much!

## Source

- HELib

```shell
git subrepo clone https://github.com/openssl/openssl.git helib-src/openssl --branch 7b649c7 --method rebase
```

git config user.email "mark@ckrd.io"
git config user.name "Mark Van de Vyver"

## Versioning

This crate follows the minor and patch versions for each maintained major version, according to:

- The HELib release strategy.
- The OpenFHE release strategy.
- The SEAL release strategy.

The crate versions follow the X.Y.Z+B pattern:

- The major version X is the upstream OpenSSL API/ABI compatibility version:
        300 for 3.Y.Z
- The minor Y and patch Z versions are incremented when making changes to the crate, either upstream update or internal changes.
- `B` contains the full upstream version, like 1.1.1k or 3.0.7. Note that this field is actually ignored in comparisons and only there for documentation.

## References

[Parameter Repository](https://github.com/Crypto-TII/fhegen)

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

[Comparison Repository](https://github.com/iliailia/comparison-circuit-over-fq)

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
