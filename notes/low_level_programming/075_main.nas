bits 64
section .text

extern string_length
extern print_string
extern print_newline
extern print_char
extern parse_int
extern print_int
extern print_uint
extern string_equals
extern read_char
extern read_word
extern parse_uint
extern string_copy


global _start
_start:

    mov rdi, msg
    call print_string
    call print_newline

    xor rdi, rdi
    mov rax, 60
    syscall


section .data
msg:
    db "hi !", 0
