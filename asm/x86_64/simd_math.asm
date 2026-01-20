; Super-C Runtime - x86-64 SIMD Math Operations
; NASM syntax for Windows x64
;
; AVX-optimized math for hot paths
; Windows x64 calling convention: RCX, RDX, R8, R9

default rel
section .text

global asm_dot_product_f32
global asm_reduce_sum_f32
global asm_reduce_max_f32
global asm_reduce_min_f32

; float asm_dot_product_f32(const float* a, const float* b, size_t n)
; Windows x64: RCX = a, RDX = b, R8 = n
; Returns result in XMM0
asm_dot_product_f32:
    push rbp
    mov rbp, rsp
    
    vxorps ymm0, ymm0, ymm0     ; accumulator = 0
    
    cmp r8, 8
    jl .scalar_loop
    
    mov rax, r8
    shr rax, 3                  ; rax = n / 8
    
.avx_loop:
    vmovups ymm1, [rcx]
    vmovups ymm2, [rdx]
    vfmadd231ps ymm0, ymm1, ymm2
    add rcx, 32
    add rdx, 32
    dec rax
    jnz .avx_loop
    
    ; Horizontal sum of ymm0
    vextractf128 xmm1, ymm0, 1
    vaddps xmm0, xmm0, xmm1
    vhaddps xmm0, xmm0, xmm0
    vhaddps xmm0, xmm0, xmm0
    
    and r8, 7
    jz .done
    
.scalar_loop:
    test r8, r8
    jz .done
    vmovss xmm1, [rcx]
    vmovss xmm2, [rdx]
    vfmadd231ss xmm0, xmm1, xmm2
    add rcx, 4
    add rdx, 4
    dec r8
    jnz .scalar_loop
    
.done:
    vzeroupper
    pop rbp
    ret

; float asm_reduce_sum_f32(const float* arr, size_t n)
; Windows x64: RCX = arr, RDX = n
; Returns sum in XMM0
asm_reduce_sum_f32:
    push rbp
    mov rbp, rsp
    
    vxorps ymm0, ymm0, ymm0     ; accumulator = 0
    
    cmp rdx, 8
    jl .sum_scalar
    
    mov rax, rdx
    shr rax, 3
    
.sum_avx_loop:
    vaddps ymm0, ymm0, [rcx]
    add rcx, 32
    dec rax
    jnz .sum_avx_loop
    
    ; Horizontal sum
    vextractf128 xmm1, ymm0, 1
    vaddps xmm0, xmm0, xmm1
    vhaddps xmm0, xmm0, xmm0
    vhaddps xmm0, xmm0, xmm0
    
    and rdx, 7
    jz .sum_done
    
.sum_scalar:
    test rdx, rdx
    jz .sum_done
    vaddss xmm0, xmm0, [rcx]
    add rcx, 4
    dec rdx
    jnz .sum_scalar
    
.sum_done:
    vzeroupper
    pop rbp
    ret

; float asm_reduce_max_f32(const float* arr, size_t n)
; Windows x64: RCX = arr, RDX = n
asm_reduce_max_f32:
    push rbp
    mov rbp, rsp
    
    ; Initialize with first element
    vbroadcastss ymm0, dword [rcx]
    
    cmp rdx, 8
    jl .max_scalar
    
    mov rax, rdx
    shr rax, 3
    
.max_avx_loop:
    vmaxps ymm0, ymm0, [rcx]
    add rcx, 32
    dec rax
    jnz .max_avx_loop
    
    ; Horizontal max
    vextractf128 xmm1, ymm0, 1
    vmaxps xmm0, xmm0, xmm1
    vshufps xmm1, xmm0, xmm0, 0x4E
    vmaxps xmm0, xmm0, xmm1
    vshufps xmm1, xmm0, xmm0, 0xB1
    vmaxps xmm0, xmm0, xmm1
    
    and rdx, 7
    jz .max_done
    
.max_scalar:
    test rdx, rdx
    jz .max_done
    vmaxss xmm0, xmm0, [rcx]
    add rcx, 4
    dec rdx
    jnz .max_scalar
    
.max_done:
    vzeroupper
    pop rbp
    ret

; float asm_reduce_min_f32(const float* arr, size_t n)
; Windows x64: RCX = arr, RDX = n
asm_reduce_min_f32:
    push rbp
    mov rbp, rsp
    
    ; Initialize with first element
    vbroadcastss ymm0, dword [rcx]
    
    cmp rdx, 8
    jl .min_scalar
    
    mov rax, rdx
    shr rax, 3
    
.min_avx_loop:
    vminps ymm0, ymm0, [rcx]
    add rcx, 32
    dec rax
    jnz .min_avx_loop
    
    ; Horizontal min
    vextractf128 xmm1, ymm0, 1
    vminps xmm0, xmm0, xmm1
    vshufps xmm1, xmm0, xmm0, 0x4E
    vminps xmm0, xmm0, xmm1
    vshufps xmm1, xmm0, xmm0, 0xB1
    vminps xmm0, xmm0, xmm1
    
    and rdx, 7
    jz .min_done
    
.min_scalar:
    test rdx, rdx
    jz .min_done
    vminss xmm0, xmm0, [rcx]
    add rcx, 4
    dec rdx
    jnz .min_scalar
    
.min_done:
    vzeroupper
    pop rbp
    ret
