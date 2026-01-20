/**
 * Super-C Runtime - ASM Hot Paths Header
 * 
 * C declarations for assembly implementations.
 * ASM → C → Rust (never direct ASM → Rust)
 */

#ifndef ASM_OPS_H
#define ASM_OPS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Memory Operations (SIMD optimized)
 * ============================================================================ */

/**
 * Fast memory copy using SIMD
 * @param dst Destination pointer (16-byte aligned)
 * @param src Source pointer (16-byte aligned)
 * @param size Bytes to copy
 */
void asm_memcpy_fast(void* dst, const void* src, size_t size);

/**
 * Fast memory set using SIMD
 * @param dst Destination pointer (16-byte aligned)
 * @param value Value to set
 * @param size Bytes to set
 */
void asm_memset_fast(void* dst, uint8_t value, size_t size);

/* ============================================================================
 * Hashing (optimized)
 * ============================================================================ */

/**
 * Fast 64-bit hash
 * @param data Input data
 * @param size Data size
 * @return 64-bit hash value
 */
uint64_t asm_hash64(const void* data, size_t size);

/* ============================================================================
 * Math Operations (SIMD)
 * ============================================================================ */

/**
 * Vector dot product (float32, AVX)
 * @param a First vector
 * @param b Second vector
 * @param n Vector length
 * @return Dot product result
 */
float asm_dot_product_f32(const float* a, const float* b, size_t n);

/**
 * Vector add (float32, AVX)
 * @param dst Destination
 * @param a First vector
 * @param b Second vector
 * @param n Vector length
 */
void asm_vector_add_f32(float* dst, const float* a, const float* b, size_t n);

#ifdef __cplusplus
}
#endif

#endif /* ASM_OPS_H */
