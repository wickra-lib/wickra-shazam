# Build and run the C and C++ examples against the wickra-shazam C ABI.
#
#   cargo build --release -p wickra-shazam-c
#   cmake -S examples/c -B examples/c/build
#   cmake --build examples/c/build --config Release
#   ctest --test-dir examples/c/build -C Release --output-on-failure
#
# WICKRA_SHAZAM_LIB_DIR defaults to the workspace target/release directory and
# can be overridden for out-of-tree builds.

cmake_minimum_required(VERSION 3.15)
project(wickra_shazam_examples C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(WICKRA_SHAZAM_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    CACHE PATH "Directory containing the built wickra-shazam C ABI library")

set(WICKRA_SHAZAM_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/../../bindings/c/include")

find_library(WICKRA_SHAZAM_LIB
    NAMES wickra_shazam libwickra_shazam
    HINTS "${WICKRA_SHAZAM_LIB_DIR}"
    REQUIRED)

add_executable(match match.c)
target_include_directories(match PRIVATE "${WICKRA_SHAZAM_INCLUDE}")
target_link_libraries(match PRIVATE "${WICKRA_SHAZAM_LIB}")

add_executable(matchpp match.cpp)
target_include_directories(matchpp PRIVATE "${WICKRA_SHAZAM_INCLUDE}")
target_link_libraries(matchpp PRIVATE "${WICKRA_SHAZAM_LIB}")

# On Windows the runtime DLL must sit next to the executable (there is no rpath);
# without it the loader pops a modal "DLL not found" dialog that hangs ctest. Copy
# it next to each example after build. (Unix resolves the .so/.dylib via the
# linker's embedded path, so this is Windows-only.)
if(WIN32)
    foreach(example match matchpp)
        add_custom_command(TARGET ${example} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
                "${WICKRA_SHAZAM_LIB_DIR}/wickra_shazam.dll"
                "$<TARGET_FILE_DIR:${example}>"
            VERBATIM)
    endforeach()
endif()

enable_testing()
add_test(NAME match COMMAND match)
add_test(NAME matchpp COMMAND matchpp)
# A missing runtime dependency must fail fast, never block the runner for hours.
set_tests_properties(match matchpp PROPERTIES TIMEOUT 60)
