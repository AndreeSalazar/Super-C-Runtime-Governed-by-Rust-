cmake_minimum_required(VERSION 3.16)
project(super_c_cuda CUDA CXX)

# Find CUDA
find_package(CUDAToolkit REQUIRED)

set(CMAKE_CUDA_STANDARD 14)
set(CMAKE_CXX_STANDARD 17)

# Include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../native/include)

# Source files
file(GLOB_RECURSE CUDA_SOURCES 
    "src/*.cu"
    "src/*.cpp"
)

# CUDA library
add_library(super_c_cuda STATIC ${CUDA_SOURCES})

# Link CUDA runtime
target_link_libraries(super_c_cuda CUDA::cudart)

# Compiler flags
set_target_properties(super_c_cuda PROPERTIES
    CUDA_SEPARABLE_COMPILATION ON
    CUDA_ARCHITECTURES "60;70;75;80;86"
)
