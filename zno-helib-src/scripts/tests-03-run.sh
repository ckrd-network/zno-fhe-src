#!/bin/bash
set -euo pipefail

# Script to run individual test cases and integrate them with ctest

# Source and destination directories
HELIB_BUILD_DIR="/tmp/helib_build/HElib/build"
HELIB_TEST_BUILD_DIR="/tmp/helib_build/HElib/build/tests"
XUNIT_TEST_RESULT_DIR="/tmp/helib_build/HElib/build/xunit_test_result"

# Ensure the xunit test result directory exists
mkdir -p "$XUNIT_TEST_RESULT_DIR"

# Loop over test files in the build directory
for test_case_file in "$HELIB_TEST_BUILD_DIR"/*.cpp; do
    if [ -f "$test_case_file" ]; then
        test_case_name=$(basename "$test_case_file" .cpp)
        test_result_file="$XUNIT_TEST_RESULT_DIR/TestResult-$test_case_name.xml"

        # Run the individual test case and capture the result
        "$HELIB_BUILD_DIR/dependencies/Build/helib_external/bin/runTests" "--gtest_output=xml:$test_result_file" "--gtest_filter=*${test_case_name}*"

        # Integrate the test case with ctest using CTestTestfile.cmake
        echo "add_test($test_case_name \"runTests\" \"--gtest_output=xml:$test_result_file\" \"--gtest_filter=*${test_case_name}*\")" >> "$HELIB_TEST_BUILD_DIR/CTestTestfile.cmake"
        echo "set_tests_properties($test_case_name PROPERTIES WORKING_DIRECTORY \"$HELIB_BUILD_DIR/dependencies/Build/helib_external/bin\" _BACKTRACE_TRIPLES \"$HELIB_TEST_BUILD_DIR/CMakeLists.txt;0;\")" >> "$HELIB_TEST_BUILD_DIR/CTestTestfile.cmake"
    fi
done

# Change to the HELIB_BUILD_DIR
cd "$HELIB_BUILD_DIR"

# Run ctest using the updated CTestTestfile.cmake
ctest -S "$HELIB_TEST_BUILD_DIR/CTestTestfile.cmake"

# #!/bin/bash
# set -euo pipefail

# # Shell script to copy and integrate tests into the build directory

# # Source and destination directories
# DEST_DIR="$(pwd)/zno-helib-src/scripts/tests"
# HELIB_TEST_BUILD_DIR="/tmp/helib_build/HElib/build/tests"

# # Copy the extracted tests to the HELIB_TEST_BUILD_DIR
# cp -r "$DEST_DIR"/* "$HELIB_TEST_BUILD_DIR"

# # Change to the HELIB_TEST_BUILD_DIR
# cd "$HELIB_TEST_BUILD_DIR"

# # Integrate the tests into CMakeLists.txt
# for test_file in *.cpp; do
#     # Add each test file to the CMakeLists.txt file
#     echo "add_executable(${test_file%.*} ${test_file})" >> CMakeLists.txt
# done

# # Run the tests
# make -j "$(nproc)" # Replace with your build command (e.g., make, cmake, etc.)

# # Loop over the new test files and run each one
# for test_file in *.cpp; do
#     ./"${test_file%.*}"
# done

# # #!/bin/bash
# # set -exuo pipefail

# # # Script to run individual HElib tests

# # # Set up temporary build directory
# # BUILD_DIR="/tmp/helib_build"
# # HELIB_SOURCE_DIR="$BUILD_DIR/HElib"
# # TEST_SRC_DIR="$(pwd)/zno-helib-src/scripts/tests"
# # TEST_BUILD_DIR="$HELIB_SOURCE_DIR/tests/"

# # cp -rf "$TEST_SRC_DIR/*.cpp"
# # find "$TEST_SRC_DIR" -name "*.cpp" -exec cp {} "$BUILD_DIR"/tests/ \;

# # cd "$HELIB_SOURCE_DIR"

# # # Loop over the test directories
# # for test_dir in "$BUILD_DIR"/tests/*; do
# #   if [ -d "$test_dir" ]; then
# #     # Build and run the test
# #     test_filename=$(basename "$test_dir")
# #     make "$test_filename"  # Replace with the actual build command

# #     # Run the test
# #     ./"$test_filename"  # Replace with the actual run command
# #   fi
# # done

# # # Restore the current directory
# # cd -

# # # : '
# # # # HElib Test Runner Script

# # # This script downloads the HElib source code to a temporary folder, builds HElib, and runs the generated tests.

# # # ## Usage
# # # 1. Run this script to download and build HElib and run the generated tests.

# # # ## Instructions
# # # - Ensure the script is executable (chmod +x run_tests.sh).
# # # - Follow the README for additional build instructions and requirements.

# # # '

# # # # HElib source directory
# # # HELIB_SOURCE_DIR="/tmp/helib_source"

# # # # Clean up any previous HElib source if it exists
# # # rm -rf "$HELIB_SOURCE_DIR"

# # # # Clone the HElib repository from GitHub
# # # git clone https://github.com/homenc/HElib.git "$HELIB_SOURCE_DIR"

# # # # Navigate to the HElib source directory
# # # cd "$HELIB_SOURCE_DIR"

# # # # Build HElib (Follow HElib's build instructions)

# # # # Run the generated test scripts
# # # for TEST_SCRIPT in $(find "$(pwd)/zno-helib-src/scripts/tests" -name "*Test*"); do
# # #   if [ -x "$TEST_SCRIPT" ]; then
# # #     echo "Running $TEST_SCRIPT"
# # #     "$TEST_SCRIPT"
# # #   else
# # #     echo "Skipping $TEST_SCRIPT (not executable)"
# # #   fi
# # # done

# # # # Clean up the temporary HElib source
# # # rm -rf "$HELIB_SOURCE_DIR"
