; Super-C Runtime - x86-64 SIMD Math Operations
; NASM syntax
;
; AVX-optimized math for hot paths

section .text

global asm_dot_product_f32
global asm_vector_add_f32

; float asm_dot_product_f32(const float* a, const float* b, size_t n)
; RDI = a, RSI = b, RDX = n
; Returns result in XMM0
asm_dot_product_f32:
    push rbp
    mov rbp, rsp
    
    vxorps ymm0, ymm0, ymm0     ; accumulator = 0
    
    cmp rdx, 8
    jl .scalar_loop
    
    mov rcx, rdx
    shr rcx, 3                  ; rcx = n / 8
    
.avx_loop:
    vmovups ymm1, [rdi]
    vmovups ymm2, [rsi]
    vfmadd231ps ymm0, ymm1, ymm2
    add rdi, 32
    add rsi, 32
    dec rcx
    jnz .avx_loop
    
    ; Horizontal sum of ymm0
    vextractf128 xmm1, ymm0, 1
    vaddps xmm0, xmm0, xmm1
    vhaddps xmm0, xmm0, xmm0
    vhaddps xmm0, xmm0, xmm0
    
    and rdx, 7
    jz .done
    
.scalar_loop:
    test rdx, rdx
    jz .done
    vmovss xmm1, [rdi]
    vmovss xmm2, [rsi]
    vfmadd231ss xmm0, xmm1, xmm2
    add rdi, 4
    add rsi, 4
    dec rdx
    jnz .scalar_loop
    
.done:
    vzeroupper
    pop rbp
    ret

; void asm_vector_add_f32(float* dst, const float* a, const float* b, size_t n)
; RDI = dst, RSI = a, RDX = b, RCX = n
asm_vector_add_f32:
    push rbp
    mov rbp, rsp
    
    cmp rcx, 8
    jl .scalar_add
    
    mov r8, rcx
    shr r8, 3                   ; r8 = n / 8
    
.avx_add_loop:
    vmovups ymm0, [rsi]
    vmovups ymm1, [rdx]
    vaddps ymm0, ymm0, ymm1
    vmovups [rdi], ymm0
    add rdi, 32
    add rsi, 32
    add rdx, 32
    dec r8
    jnz .avx_add_loop
    
    and rcx, 7
    jz .add_done
    
.scalar_add:
    test rcx, rcx
    jz .add_done
    vmovss xmm0, [rsi]
    vaddss xmm0, xmm0, [rdx]
    vmovss [rdi], xmm0
    add rdi, 4
    add rsi, 4
    add rdx, 4
    dec rcx
    jnz .scalar_add
    
.add_done:
    vzeroupper
    pop rbp
    ret
