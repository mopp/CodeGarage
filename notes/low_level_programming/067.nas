bits 64
section .text

%include "assignments/1_io_library/lib.inc"
%include "syscall.inc"

global _start
_start:
    ; Fetch the file size.
    sub rsp, 128
    mov rdi, filepath
    mov rsi, rsp
    call stat
    mov rdi, [rsi + 48]
    add rsp, 128
    push rdi

    ; Open the file.
    mov rdi, filepath
    mov rsi, O_RDONLY
    call open

    ; Map the file to memory.
    mov rdi, 0
    pop rsi
    mov rdx, PROT_READ
    mov r8, rax
    mov r9, 0
    mov r10, MAP_PRIVATE
    call mmap

    mov rdi, rax
    call parse_uint

    push rax
    mov rdi, msg,
    call print_string
    mov rdi, [rsp]
    call print_uint
    call print_newline

    ; 1. x!
    mov rdi, [rsp]
    call factorial
    mov rdi, rax
    call print_uint
    call print_newline

    ; 2.
    mov rdi, [rsp]
    call is_prime
    mov rdi, rax
    call print_uint
    call print_newline

    ; 3.
    mov rdi, [rsp]
    call sum_digit
    mov rdi, rax
    call print_uint
    call print_newline

    ; 4.
    mov rdi, [rsp]
    call fibonacci
    mov rdi, rax
    call print_uint
    call print_newline

    ; 5.
    mov rdi, [rsp]
    call is_fibonacci
    mov rdi, rax
    call print_uint
    call print_newline

    pop rax

    ; exit
    xor rdi, rdi
    call exit

; size_t factorial(size_t).
factorial:
    test rdi, rdi
    jz .zero

    xor rdx, rdx
    mov rax, 1
.loop:
    mul rdi
    dec rdi
    cmp rdi, 1
    jne .loop

    ret
.zero:
    mov rax, 1
    ret


; bool is_prime(size_t).
is_prime:
    cmp rdi, 1
    je .true
    mov rcx, 2
.loop:
    xor rdx, rdx
    mov rax, rdi
    div rcx
    inc rcx
    test rdx, rdx
    jnz .loop

.r_zero:
    dec rcx
    cmp rdi, rcx
    je .true
    xor rax, rax
    ret
.true:
    mov rax, 1
    ret


; size_t sum_digit(size_t)
sum_digit:
    mov rcx, 10
    mov rax, rdi
    xor rsi, rsi
.loop:
    xor rdx, rdx
    div rcx
    add rsi, rdx
    test rax, rax
    jnz .loop

    mov rax, rsi
    ret


; size_t fibonacci(size_t)
fibonacci:
    test rdi, rdi
    jz .zero
    cmp rdi, 1
    jz .one

    dec rdi
    xor r8, r8
    mov r9, 1
.loop:
    xor rax, rax
    add rax, r8
    add rax, r9
    mov r8, r9
    mov r9, rax
    dec rdi
    jnz .loop
    ret
.zero:
    xor rax, rax
    ret
.one:
    mov rax, 1
    ret

; bool is_fibonacci(size_t)
is_fibonacci:
    test rdi, rdi
    jz .true
    cmp rdi, 1
    jz .true

    mov rcx, 1
    xor r8, r8
    mov r9, 1
.loop:
    xor rax, rax
    add rax, r8
    add rax, r9
    mov r8, r9
    mov r9, rax
    inc rcx
    cmp rax, rdi
    jl .loop
    cmp rax, rdi
    je .true
    xor rax, rax
    ret
.true:
    mov rax, 1
    ret


section .data
filepath:
    db "067.txt", 0
msg:
    db "input: ", 0
