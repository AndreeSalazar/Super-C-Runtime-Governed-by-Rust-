; Super-C Runtime - x86-64 Memory Operations
; NASM syntax
;
; Fast SIMD memory operations for hot paths

section .text

global asm_memcpy_fast
global asm_memset_fast

; void asm_memcpy_fast(void* dst, const void* src, size_t size)
; RDI = dst, RSI = src, RDX = size
asm_memcpy_fast:
    push rbp
    mov rbp, rsp
    
    ; Handle small copies with rep movsb
    cmp rdx, 64
    jl .small_copy
    
    ; AVX copy for larger blocks
    mov rcx, rdx
    shr rcx, 5          ; rcx = size / 32
    
.avx_loop:
    vmovdqu ymm0, [rsi]
    vmovdqu [rdi], ymm0
    add rsi, 32
    add rdi, 32
    dec rcx
    jnz .avx_loop
    
    ; Handle remaining bytes
    and rdx, 31
    jz .done
    
.small_copy:
    mov rcx, rdx
    rep movsb
    
.done:
    vzeroupper
    pop rbp
    ret

; void asm_memset_fast(void* dst, uint8_t value, size_t size)
; RDI = dst, RSI = value, RDX = size
asm_memset_fast:
    push rbp
    mov rbp, rsp
    
    ; Broadcast value to YMM register
    movzx eax, sil
    imul eax, 0x01010101
    vmovd xmm0, eax
    vpbroadcastd ymm0, xmm0
    
    cmp rdx, 64
    jl .small_set
    
    mov rcx, rdx
    shr rcx, 5          ; rcx = size / 32
    
.avx_set_loop:
    vmovdqu [rdi], ymm0
    add rdi, 32
    dec rcx
    jnz .avx_set_loop
    
    and rdx, 31
    jz .set_done
    
.small_set:
    mov rcx, rdx
    mov al, sil
    rep stosb
    
.set_done:
    vzeroupper
    pop rbp
    ret
