; Super-C Runtime - x86-64 Memory Operations
; NASM syntax for Windows x64
;
; Fast SIMD memory operations for hot paths
; Windows x64 calling convention: RCX, RDX, R8, R9

default rel
section .text

global asm_memcpy_fast
global asm_memset_fast
global asm_vector_add_f32
global asm_vector_mul_f32
global asm_vector_scale_f32

; void asm_memcpy_fast(void* dst, const void* src, size_t size)
; Windows x64: RCX = dst, RDX = src, R8 = size
asm_memcpy_fast:
    push rbp
    mov rbp, rsp
    push rdi
    push rsi
    
    ; Setup for rep movsb (uses RDI, RSI, RCX)
    mov rdi, rcx        ; dst
    mov rsi, rdx        ; src
    mov rcx, r8         ; size
    
    ; Handle small copies with rep movsb
    cmp rcx, 64
    jl .small_copy
    
    ; Save size for remainder
    mov r9, rcx
    shr rcx, 5          ; rcx = size / 32
    
.avx_loop:
    vmovdqu ymm0, [rsi]
    vmovdqu [rdi], ymm0
    add rsi, 32
    add rdi, 32
    dec rcx
    jnz .avx_loop
    
    ; Handle remaining bytes
    mov rcx, r9
    and rcx, 31
    jz .done
    
.small_copy:
    rep movsb
    
.done:
    vzeroupper
    pop rsi
    pop rdi
    pop rbp
    ret

; void asm_memset_fast(void* dst, uint8_t value, size_t size)
; Windows x64: RCX = dst, RDX = value, R8 = size
asm_memset_fast:
    push rbp
    mov rbp, rsp
    push rdi
    
    mov rdi, rcx        ; dst
    
    ; Broadcast value to YMM register
    movzx eax, dl
    imul eax, 0x01010101
    vmovd xmm0, eax
    vpbroadcastd ymm0, xmm0
    
    mov rcx, r8         ; size
    cmp rcx, 64
    jl .small_set
    
    mov r9, rcx
    shr rcx, 5          ; rcx = size / 32
    
.avx_set_loop:
    vmovdqu [rdi], ymm0
    add rdi, 32
    dec rcx
    jnz .avx_set_loop
    
    mov rcx, r9
    and rcx, 31
    jz .set_done
    
.small_set:
    mov al, dl
    rep stosb
    
.set_done:
    vzeroupper
    pop rdi
    pop rbp
    ret

; void asm_vector_add_f32(float* dst, const float* a, const float* b, size_t count)
; Windows x64: RCX = dst, RDX = a, R8 = b, R9 = count
asm_vector_add_f32:
    push rbp
    mov rbp, rsp
    
    ; Process 8 floats at a time (256-bit AVX)
    mov rax, r9
    shr rax, 3          ; count / 8
    jz .add_remainder
    
.add_avx_loop:
    vmovups ymm0, [rdx]
    vmovups ymm1, [r8]
    vaddps ymm2, ymm0, ymm1
    vmovups [rcx], ymm2
    add rcx, 32
    add rdx, 32
    add r8, 32
    dec rax
    jnz .add_avx_loop
    
.add_remainder:
    ; Handle remaining elements
    and r9, 7
    jz .add_done
    
.add_scalar_loop:
    vmovss xmm0, [rdx]
    vaddss xmm0, xmm0, [r8]
    vmovss [rcx], xmm0
    add rcx, 4
    add rdx, 4
    add r8, 4
    dec r9
    jnz .add_scalar_loop
    
.add_done:
    vzeroupper
    pop rbp
    ret

; void asm_vector_mul_f32(float* dst, const float* a, const float* b, size_t count)
; Windows x64: RCX = dst, RDX = a, R8 = b, R9 = count
asm_vector_mul_f32:
    push rbp
    mov rbp, rsp
    
    mov rax, r9
    shr rax, 3
    jz .mul_remainder
    
.mul_avx_loop:
    vmovups ymm0, [rdx]
    vmovups ymm1, [r8]
    vmulps ymm2, ymm0, ymm1
    vmovups [rcx], ymm2
    add rcx, 32
    add rdx, 32
    add r8, 32
    dec rax
    jnz .mul_avx_loop
    
.mul_remainder:
    and r9, 7
    jz .mul_done
    
.mul_scalar_loop:
    vmovss xmm0, [rdx]
    vmulss xmm0, xmm0, [r8]
    vmovss [rcx], xmm0
    add rcx, 4
    add rdx, 4
    add r8, 4
    dec r9
    jnz .mul_scalar_loop
    
.mul_done:
    vzeroupper
    pop rbp
    ret

; void asm_vector_scale_f32(float* dst, const float* src, float scale, size_t count)
; Windows x64: RCX = dst, RDX = src, XMM2 = scale, R9 = count
asm_vector_scale_f32:
    push rbp
    mov rbp, rsp
    
    ; Broadcast scale to ymm1
    vbroadcastss ymm1, xmm2
    
    mov rax, r9
    shr rax, 3
    jz .scale_remainder
    
.scale_avx_loop:
    vmovups ymm0, [rdx]
    vmulps ymm2, ymm0, ymm1
    vmovups [rcx], ymm2
    add rcx, 32
    add rdx, 32
    dec rax
    jnz .scale_avx_loop
    
.scale_remainder:
    and r9, 7
    jz .scale_done
    
    ; Restore scalar scale
    vbroadcastss xmm1, xmm2
    
.scale_scalar_loop:
    vmovss xmm0, [rdx]
    vmulss xmm0, xmm0, xmm1
    vmovss [rcx], xmm0
    add rcx, 4
    add rdx, 4
    dec r9
    jnz .scale_scalar_loop
    
.scale_done:
    vzeroupper
    pop rbp
    ret
