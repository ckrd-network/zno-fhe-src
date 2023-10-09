#!/bin/bash
#!/bin/bash
set -exuo pipefail

# # GTest Splitter Script

# This script splits Google Test (gtest) files into individual test files, appends a test counter to each new file, and sets up the necessary includes and dependencies.

# ## Usage
# 1. Run this script to split GTest files.
# 2. Run `run_tests.sh` (separate script) to download HElib source, build HElib, and run the generated tests.

# ## Instructions
# - Set SOURCE_DIR and DEST_DIR variables to specify source and destination directories.
# - Ensure that source files are protected from changes while working with them.
# - Follow the README for additional build instructions.

# Source and destination directories
SOURCE_DIR="$(pwd)/zno-helib-src/helib/tests"
DEST_DIR="$(pwd)/zno-helib-src/scripts/tests"

# Ensure the destination directory exists
mkdir -p "$DEST_DIR"

# Loop over all test files in the source directory
for TEST_FILE in "$SOURCE_DIR"/GTest*.cpp; do
  # Extract the base name of the test file without the extension
  BASE_NAME=$(basename "$TEST_FILE" .cpp)

  # Create a copy of the original test file for modification
  cp "$TEST_FILE" "$DEST_DIR/$BASE_NAME"_temp.cpp

  # Initialize a test counter
  TEST_COUNTER=0

  # Split the test file into individual test files using csplit
  csplit -s -f "$DEST_DIR/$BASE_NAME" "$DEST_DIR/$BASE_NAME"_temp.cpp '/TEST(/' '{*}'

  # Remove the temporary file
  rm "$DEST_DIR/$BASE_NAME"_temp.cpp

  # Create CMakeLists.txt file for each generated test file
  echo "project(${BASE_NAME}Tests)" > "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
  for GENERATED_TEST_FILE in "$DEST_DIR/$BASE_NAME"*; do
    if [ -f "$GENERATED_TEST_FILE" ]; then
      # Extract the test name from the generated test file
      TEST_NAME=$(basename "$GENERATED_TEST_FILE" .cpp)
      # Append test counter to the test name
      NEW_TEST_NAME="${BASE_NAME}_Test${TEST_COUNTER}"

      # Rename the generated test file
      mv "$GENERATED_TEST_FILE" "$DEST_DIR/$NEW_TEST_NAME.cpp"

      # Increment the test counter
      TEST_COUNTER=$((TEST_COUNTER + 1))

      # Add test target to CMakeLists.txt
      echo "add_executable(${NEW_TEST_NAME} ${NEW_TEST_NAME}.cpp)" >> "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
      echo "target_link_libraries(${NEW_TEST_NAME} gtest_main)" >> "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
    fi
  done
done

echo "Test files separated and moved to $DEST_DIR"

# # This script does the following:

# #     Copies the source test files to the destination directory to avoid modifying the original files.
# #     Splits the test files into individual test files using csplit based on the TEST() macro.
# #     Renames the generated test files by appending a test counter.
# #     Creates CMakeLists.txt files for each test suite to set up the necessary includes and dependencies for building the tests separately.

# set -e  # Enable error reporting

# # Define the source and destination directories
# SOURCE_DIR="$(pwd)/zno-helib-src/helib/tests"
# DEST_DIR="$(pwd)/zno-helib-src/scripts/tests"

# # Ensure the destination directory exists
# mkdir -p "$DEST_DIR"

# # Loop over all test files in the source directory
# for TEST_FILE in "$SOURCE_DIR"/*.cpp; do
#   # Extract the base name of the test file without the extension
#   BASE_NAME=$(basename "$TEST_FILE" .cpp)

#   # Create a copy of the original test file for modification
#   cp "$TEST_FILE" "$DEST_DIR/$BASE_NAME"_temp.cpp

#   # Split the test file into individual test files using csplit
#   csplit -s -f "$DEST_DIR/$BASE_NAME" "$DEST_DIR/$BASE_NAME"_temp.cpp '/TEST(/' '{*}'

#   # Remove the temporary file
#   rm "$DEST_DIR/$BASE_NAME"_temp.cpp

#   # Create a CMakeLists.txt file for each generated test file
#   echo "project(${BASE_NAME}Test)" > "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
#   for GENERATED_TEST_FILE in "$DEST_DIR/$BASE_NAME"*; do
#     if [ -f "$GENERATED_TEST_FILE" ]; then
#       echo "add_executable(${BASE_NAME}Test_${GENERATED_TEST_FILE##*_} $GENERATED_TEST_FILE)" >> "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
#       echo "target_link_libraries(${BASE_NAME}Test_${GENERATED_TEST_FILE##*_} gtest_main)" >> "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
#     fi
#   done
# done

# echo "Test files separated and moved to $DEST_DIR"

# # set -e  # Enable error reporting

# # # Define the source and destination directories
# # SOURCE_DIR="$(pwd)/zno-helib-src/helib/tests"
# # DEST_DIR="$(pwd)/zno-helib-src/scripts/tests"

# # # Ensure the destination directory exists
# # mkdir -p "$DEST_DIR"

# # # Loop over all test files in the source directory
# # for TEST_FILE in "$SOURCE_DIR"/*.cpp; do
# #   # Extract the base name of the test file without the extension
# #   BASE_NAME=$(basename "$TEST_FILE" .cpp)

# #   # Create a copy of the original test file for modification
# #   cp "$TEST_FILE" "$DEST_DIR/$BASE_NAME"_temp.cpp

# #   # Use awk to split the test file into individual test files
# #   awk '/TEST(/,/{if ($0 ~ "TEST(") {if (NR > 1) {print "}" > "'"$DEST_DIR/$BASE_NAME"'_"++i".cpp"}; i=0}; {print > "'"$DEST_DIR/$BASE_NAME"'_"++i".cpp"}' "$DEST_DIR/$BASE_NAME"_temp.cpp

# #   # Remove the temporary file
# #   rm "$DEST_DIR/$BASE_NAME"_temp.cpp

# #   # Create a CMakeLists.txt file for each generated test file
# #   echo "project(${BASE_NAME}Test)" > "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
# #   echo "add_executable(${BASE_NAME}Test ${BASE_NAME}_1.cpp)" >> "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
# #   echo "target_link_libraries(${BASE_NAME}Test gtest_main)" >> "$DEST_DIR/$BASE_NAME"_CMakeLists.txt
# # done

# # echo "Test files separated and moved to $DEST_DIR"
