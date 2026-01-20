/**
 * Super-C Runtime - Rust ↔ C Glue Layer
 * 
 * Bridge between Rust FFI and C implementations.
 */

#include "super_c.h"

// This file serves as the glue between Rust's extern "C" calls
// and the actual C implementations.
// 
// The functions declared in super_c.h are implemented in:
// - core/runtime.c (native_* functions)
// - dispatch/dispatcher.c (dispatch logic)
// - ../cuda/src/* (cuda_* functions)
