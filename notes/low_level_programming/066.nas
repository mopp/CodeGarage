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

    mov rdi, msg,
    call print_string

    mov rdi, [rsp],
    call print_uint
    call print_newline
    call print_newline

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

    ; Print the content.
    mov rdx, rsi
    mov rdi, STDOUT
    mov rsi, rax
    call write

    ; exit
    xor rdi, rdi
    call exit


section .data
filepath:
    db "memo.md", 0
msg:
    db "The file size is ", 0
