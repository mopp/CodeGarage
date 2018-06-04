; vi:ft=nasm
bits 64

%include "words.inc"

section .text

extern find_word
extern print_string
extern string_length
extern print_uint
extern print_newline
extern read_word

global _start
_start:
    ; Input a key (max 255) from STDIN.
    ; NOTE: this read_word differs from assignment1.
    mov rdi, key_buffer
    mov rsi, key_buffer_size
    call read_word

    test rax, rax
    jz .invalid_input

    ; Keep the length of the given key.
    push rdx

    ; Find value by the given key.
    mov rdi, rax
    mov rsi, list_head
    call find_word

    pop rdx

    test rax, rax
    jz .not_found

.found:
    ; Calculate the address of the value in the entry.
    add rax, rdx
    add rax, 8 + 1 ; address size + null character.

    ; Print the value.
    mov rdi, rax
    call print_string
    call print_newline

    xor rdi, rdi
.exit:
    ; Exit
    mov rax, 60
    syscall

.invalid_input:
    mov rdi, msg_invalid_input
    call print_error
    mov rdi, 1
    jmp .exit

.not_found:
    mov rdi, msg_not_found
    call print_error
    mov rdi, 2
    jmp .exit


print_error:
    mov rsi, rdi
    call string_length

    mov rdi, 2
    mov rdx, rax
    mov rax, 1
    syscall

    ret

section .data
key_buffer:
    times 255 db 0
key_buffer_size equ $ - key_buffer

msg_invalid_input:
    db "Invalid input is given", 0x0A, 0
msg_not_found:
    db "The key is NOT found.", 0x0A, 0
