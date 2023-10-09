# Documentation

## Split-tests and Run-tests

- `split_tests.sh` splits GTest files and generates CMakeLists.txt files.
- `run_tests.sh` downloads HElib, builds it, and runs the generated tests.
- To use these scripts:
  - Make sure the scripts are executable (e.g., `chmod +x split_tests.sh run_tests.sh`).
  - Run `./split_tests.sh` to split the tests and generate CMakeLists.txt files.
  - Run `./run_tests.sh` to download HElib, build it, and run the generated tests.
- Set SOURCE_DIR and DEST_DIR in split_tests.sh as needed.
- Follow the README for any additional build instructions and requirements.
