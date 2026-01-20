cmake_minimum_required(VERSION 3.16)
project(super_c_native C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# Include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# Source files
file(GLOB_RECURSE NATIVE_SOURCES 
    "src/*.c"
    "src/*.cpp"
)

# Static library
add_library(super_c_native STATIC ${NATIVE_SOURCES})

# Shared library
add_library(super_c_native_shared SHARED ${NATIVE_SOURCES})
set_target_properties(super_c_native_shared PROPERTIES OUTPUT_NAME super_c_native)

# Compiler flags
if(MSVC)
    target_compile_options(super_c_native PRIVATE /W4 /O2)
    target_compile_options(super_c_native_shared PRIVATE /W4 /O2)
else()
    target_compile_options(super_c_native PRIVATE -Wall -Wextra -O3)
    target_compile_options(super_c_native_shared PRIVATE -Wall -Wextra -O3)
endif()

# Link ASM library if available
if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/../asm")
    add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../asm asm_build)
    target_link_libraries(super_c_native super_c_asm)
    target_link_libraries(super_c_native_shared super_c_asm)
endif()
