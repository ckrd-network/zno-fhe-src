#----------------------------------------------------------------
# Generated CMake target import file for configuration "Debug".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "helib" for configuration "Debug"
set_property(TARGET helib APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(helib PROPERTIES
  IMPORTED_LOCATION_DEBUG "${_IMPORT_PREFIX}/lib/libhelib.so.2.2.0"
  IMPORTED_SONAME_DEBUG "libhelib.so.2.2.0"
  )

list(APPEND _cmake_import_check_targets helib )
list(APPEND _cmake_import_check_files_for_helib "${_IMPORT_PREFIX}/lib/libhelib.so.2.2.0" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
