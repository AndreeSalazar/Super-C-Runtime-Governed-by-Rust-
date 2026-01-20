cmake_minimum_required(VERSION 3.16)
project(super_c_asm C ASM_NASM)

# Detect architecture
if(CMAKE_SYSTEM_PROCESSOR MATCHES "x86_64|AMD64")
    set(ASM_ARCH "x86_64")
    set(ASM_FILES
        x86_64/memops.asm
        x86_64/simd_math.asm
    )
elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "aarch64|ARM64")
    set(ASM_ARCH "aarch64")
    set(ASM_FILES
        aarch64/simd_math.s
    )
else()
    message(WARNING "Unsupported architecture: ${CMAKE_SYSTEM_PROCESSOR}")
    set(ASM_FILES "")
endif()

# Include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# ASM library (if we have files)
if(ASM_FILES)
    add_library(super_c_asm STATIC ${ASM_FILES})
    
    if(ASM_ARCH STREQUAL "x86_64")
        if(WIN32)
            set(CMAKE_ASM_NASM_FLAGS "-f win64")
        else()
            set(CMAKE_ASM_NASM_FLAGS "-f elf64")
        endif()
    endif()
endif()
