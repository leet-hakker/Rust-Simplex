# If pkg-config is not installed on the system, then the CHECK_INSTALL_DIR var
# must be set to the install location of Check. For example, on Windows, this
# may be: C:/Program Files/check
# set(CHECK_INSTALL_DIR "C:/Program Files/check")

cmake_minimum_required(VERSION 3.15)

project(Tests)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")

enable_testing()

# Change compiler
set(CMAKE_SYSTEM_NAME linux)
set(CMAKE_SYSTEM_PROCESSOR x86_64)
set(CMAKE_C_COMPILER gcc)
set(CMAKE_C_FLAGS "-Wall -Werror -g3 -O2")

find_package(Check REQUIRED)

include(CheckCSourceCompiles)
include(CheckCSourceRuns)
include(CheckFunctionExists)
include(CheckIncludeFile)
include(CheckIncludeFiles)
include(CheckLibraryExists)
include(CheckSymbolExists)
include(CheckTypeSize)

# Since Check uses Threads to paralelize the tests, it's mandatory
# add pthread as a dependency, alongside the Check libraries.
file(GLOB fraction_c RELATIVE "../src/lib" "fraction.c")
add_executable(test_fraction test_fraction.c ${fraction_c})
target_link_libraries(test_fraction ${CHECK_LIBRARIES} pthread)

# Create testing target and redirect its output to `tests` folder
add_test(NAME test_fraction COMMAND test_fraction WORKING_DIRECTORY ${CMAKE_BINARY_DIR}/tests)
set_tests_properties(test_fraction PROPERTIES TIMEOUT 30) 

# Additional Valgrind test to check memory leaks in code
include(FindValgrind)

add_custom_target(valgrind_test_fraction
  COMMAND ${VALGRIND_PROGRAM} --leak-check=full $<TARGET_FILE:test_fraction>
  DEPENDS test_fraction
)
