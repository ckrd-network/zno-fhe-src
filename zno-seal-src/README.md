# SEAL

## General prerequisites

- pthreads
- git >= 2.36 (required to build and run the SEAL test suite)

The Microsoft SEAL build system can download and build dependencies, or search the system directories for pre-installed dependencies.

he optional dependencies and their tested versions (other versions may work as well) are as follows:

| Optional dependency | Tested version | Use                                      |
|---------------------|----------------|------------------------------------------|
| Intel HEXL          | 1.2.5          | Acceleration of low-level kernels         |
| Microsoft GSL       | 4.0.0          | API extensions                           |
| ZLIB                | 1.2.13         | Compressed serialization                  |
| Zstandard           | 1.5.2          | Compressed serialization (much faster than ZLIB) |
| GoogleTest          | 1.12.1         | For running tests                         |
| GoogleBenchmark     | 1.7.1          | For running benchmarks                    |

**Default Linux environment:**

- Ubuntu 22.04 LTS
- Clang++ (>= 5.0) or GNU G++ (>= 6.0)
  - SEAL compiled with Clang++ has much better runtime performance than one compiled with GNU G++
- CMake (>= 3.13) available from [CMake](https://cmake.org/

**macOS environment:**

- Xcode Command Line Tools >= 9.3 (can be installed with the command `xcode-select
  --install` in a terminal)
- cmake >= 3.13 (available from [CMake](https://cmake.org/) or [MacPorts
  Project](https://www.macports.org/) and [Homebrew](https://brew.sh/) as
  packages)
