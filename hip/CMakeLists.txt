cmake_minimum_required(VERSION 3.16)
project(super_c_hip CXX)

set(CMAKE_CXX_STANDARD 17)

# ============================================================================
# HIP Detection and Configuration
# ============================================================================

# Option to force HIP-CPU mode (for systems without AMD GPU)
option(USE_HIP_CPU "Use HIP-CPU for CPU fallback execution" OFF)

# Try to find HIP
find_package(hip QUIET)

if(hip_FOUND)
    message(STATUS "HIP found: ${hip_VERSION}")
    set(HIP_AVAILABLE ON)
else()
    message(STATUS "HIP not found, checking for HIP-CPU...")
    
    # Check for HIP-CPU
    if(DEFINED ENV{HIP_CPU_PATH})
        set(HIP_CPU_PATH $ENV{HIP_CPU_PATH})
    elseif(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/../external/hip-cpu")
        set(HIP_CPU_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../external/hip-cpu")
    endif()
    
    if(HIP_CPU_PATH)
        message(STATUS "Using HIP-CPU from: ${HIP_CPU_PATH}")
        set(USE_HIP_CPU ON)
        set(HIP_AVAILABLE ON)
    else()
        message(WARNING "Neither HIP nor HIP-CPU found. HIP support disabled.")
        set(HIP_AVAILABLE OFF)
    endif()
endif()

# ============================================================================
# Source Files
# ============================================================================

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../native/include)

file(GLOB_RECURSE HIP_SOURCES 
    "src/*.cpp"
)

# ============================================================================
# Build Library
# ============================================================================

if(HIP_AVAILABLE)
    add_library(super_c_hip STATIC ${HIP_SOURCES})
    
    if(USE_HIP_CPU)
        # HIP-CPU configuration
        target_include_directories(super_c_hip PRIVATE ${HIP_CPU_PATH}/include)
        target_compile_definitions(super_c_hip PRIVATE 
            HIP_CPU_RUNTIME
            __HIP_PLATFORM_CPU__
        )
        
        # HIP-CPU requires threading
        find_package(Threads REQUIRED)
        target_link_libraries(super_c_hip Threads::Threads)
        
        # TBB for parallel execution (optional but recommended)
        find_package(TBB QUIET)
        if(TBB_FOUND)
            target_link_libraries(super_c_hip TBB::tbb)
            target_compile_definitions(super_c_hip PRIVATE HIP_CPU_USE_TBB)
        endif()
    else()
        # Native HIP (AMD GPU)
        target_link_libraries(super_c_hip hip::device)
        target_compile_definitions(super_c_hip PRIVATE __HIP_PLATFORM_AMD__)
    endif()
    
    # Common flags
    if(MSVC)
        target_compile_options(super_c_hip PRIVATE /W4 /O2)
    else()
        target_compile_options(super_c_hip PRIVATE -Wall -Wextra -O3)
    endif()
endif()
