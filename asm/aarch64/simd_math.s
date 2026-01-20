// Super-C Runtime - AArch64 SIMD Math Operations
// ARM64 Assembly
//
// NEON-optimized math for hot paths

.global asm_dot_product_f32
.global asm_vector_add_f32

.text

// float asm_dot_product_f32(const float* a, const float* b, size_t n)
// x0 = a, x1 = b, x2 = n
// Returns result in s0
asm_dot_product_f32:
    movi    v0.4s, #0           // accumulator = 0
    
    cmp     x2, #4
    b.lt    .Lscalar_dot
    
    lsr     x3, x2, #2          // x3 = n / 4
    
.Lneon_dot_loop:
    ld1     {v1.4s}, [x0], #16
    ld1     {v2.4s}, [x1], #16
    fmla    v0.4s, v1.4s, v2.4s
    subs    x3, x3, #1
    b.ne    .Lneon_dot_loop
    
    // Horizontal sum
    faddp   v0.4s, v0.4s, v0.4s
    faddp   s0, v0.2s
    
    and     x2, x2, #3
    cbz     x2, .Ldot_done
    
.Lscalar_dot:
    cbz     x2, .Ldot_done
    ldr     s1, [x0], #4
    ldr     s2, [x1], #4
    fmadd   s0, s1, s2, s0
    subs    x2, x2, #1
    b.ne    .Lscalar_dot
    
.Ldot_done:
    ret

// void asm_vector_add_f32(float* dst, const float* a, const float* b, size_t n)
// x0 = dst, x1 = a, x2 = b, x3 = n
asm_vector_add_f32:
    cmp     x3, #4
    b.lt    .Lscalar_add
    
    lsr     x4, x3, #2          // x4 = n / 4
    
.Lneon_add_loop:
    ld1     {v0.4s}, [x1], #16
    ld1     {v1.4s}, [x2], #16
    fadd    v0.4s, v0.4s, v1.4s
    st1     {v0.4s}, [x0], #16
    subs    x4, x4, #1
    b.ne    .Lneon_add_loop
    
    and     x3, x3, #3
    cbz     x3, .Ladd_done
    
.Lscalar_add:
    cbz     x3, .Ladd_done
    ldr     s0, [x1], #4
    ldr     s1, [x2], #4
    fadd    s0, s0, s1
    str     s0, [x0], #4
    subs    x3, x3, #1
    b.ne    .Lscalar_add
    
.Ladd_done:
    ret
