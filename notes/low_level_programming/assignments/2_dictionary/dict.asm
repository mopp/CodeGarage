; vi:ft=nasm
bits 64

section .text

extern string_equals

; rdi - pointer to null terminated string.
; rsi - pointer to last word in dictionary (list head).
; rax - return the address of the found entry, otherwise 0.
global find_word
find_word:
    push rdi
    push rsi

    add rsi, 8
    call string_equals

    pop rsi
    pop rdi

    test rax, rax
    jnz .found

    mov rsi, [rsi]
    test rsi, rsi
    jz .not_found

    jmp find_word
.found:
    mov rax, rsi
    ret
.not_found:
    xor rax, rax
    ret
