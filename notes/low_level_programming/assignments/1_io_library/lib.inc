bits 64
section .text

%define STDIN 0
%define STDOUT 1

; Count the length of the given string.
; rdi - a pointer to string.
; eax - the count of the string.
string_length:
    xor rax, rax
.loop:
    cmp byte [rdi], 0
    je .end

    inc rdi
    inc eax
    jmp .loop

.end:
    ret


; Print the given string to stdout.
; rdi - a pointer to string.
print_string:
    mov rsi, rdi
    call string_length

    mov rdi, 1
    mov rdx, rax
    mov rax, 1
    syscall

    ret


; Print a given character.
; rdi - a character.
print_char:
    push rdi

    mov rax, 1
    mov rdi, 1
    mov rsi, rsp
    mov rdx, 1
    syscall

    pop rdi

    xor rax, rax
    ret

print_newline:
    xor rax, rax
    ret


; rdi points to a string
; returns rax: number, rdx : length
parse_int:
    mov al, [rdi]
    cmp al, '-'
    je .negetive

    call parse_uint
    ret

.negetive:
    inc rdi
    call parse_uint

    test rdx, rdx
    jz .tmp

    imul rax, rax, -1
    inc rdx
    ret

.tmp:
    xor rax, rax
    ret


; Print the given 8-bytes signed integer as decimal.
; rdi - signed integer.
print_int:
    test rdi, rdi
    jns print_uint

    push rdi
    mov rdi, '-'
    call print_char
    pop rdi
    imul rdi, rdi, -1

; Print the given 8-bytes unsigned integer as decimal.
; rdi - unsigned integer.
print_uint:
    mov rax, rdi
    mov rcx, 10

    ; Push null character.
    push 0
    add rsp, 7
    mov r10, 1

.loop:
    xor rdx, rdx
    inc r10

    div rcx
    add rdx, 0x30

    ; Cannot push 8bit into stack directory.
    shl rdx, 8
    push dx
    inc rsp

    test rax, rax
    jnz .loop

    mov rdi, rsp
    call print_string

    add rsp, r10

    xor rax, rax
    ret

string_equals:
    xor rax, rax
    ret

; Read a character from STDIN.
; rax - a character
read_char:
    dec rsp

    xor rax, rax
    mov rdi, STDIN
    mov rsi, rsp
    mov rdx, 1
    syscall

    mov al, [rsp]
    inc rsp

    ; if the character is .
    cmp al, 0x04
    je .eot

    ret
.eot:
    xor rax, rax
    ret


; Read a word from STDIN.
; rdi - a pointer to buffer.
; rsi - the length of the buffer.
; rax - the pointer to buffer or 0 if the word length exceeds the buffer length.
; rdx - the length of the word.
read_word:
    push rdi
    push rsi

    xor rax, rax
    mov rdx, rsi
    mov rsi, rdi
    mov rdi, STDIN
    syscall

    pop rsi
    pop rdi
    xor rdx, rdx

.loop:
    cmp rdx, rsi
    je .too_long
    cmp rdx, rax
    je .break

    mov r11b, [rdi + rdx]
    cmp r11b, 0x00
    je .break
    cmp r11b, 0x20
    je .break
    cmp r11b, 0x09
    je .break
    cmp r11b, 0x0A
    je .break
    cmp r11b, 0x0D
    je .break

    inc rdx
    jmp .loop

.break:
    mov byte [rdi + rdx], 0x00
    mov rax, rdi
    ret
.too_long:
    xor rax, rax
    xor rdx, rdx
    ret


; rdi points to a string
; returns rax: number, rdx : length
parse_uint:
    mov rsi, rdi
    ; Count numbers in the string.
    xor rdx, rdx
    mov r8, 10 ; Base number.
    mov rax, 1
.loop0:
    mov r9b, [rdi]
    cmp r9b, 0x30
    jl .end_count
    cmp r9b, 0x39
    jg .end_count

    mul r8
    inc rdi
    jmp .loop0

.end_count:
    cmp rdi, rsi
    je .none

    div r8

    mov r9, rax

    mov rcx, rdi
    sub rcx, rsi

    xor r10, r10
.loop1:

    ; Parse number.
    xor rax, rax
    mov al, [rsi]
    sub al, 0x30

    inc rsi

    mul r9
    add r10, rax

    xchg r9, rax
    div r8
    xchg r9, rax

    cmp rdi, rsi
    jne .loop1

    mov rax, r10
    mov rdx, rcx
    ret
.none:
    xor rdx, rdx
    ret

; Copy N characters from source to destination.
; rdi - a pointer to the given string.
; rsi - the destination buffer address
; rdx - the length of the buffer.
; rax - the buffer address if copying is succeed, otherwise 0.
string_copy:
    ; Count the length.
    mov r10, rdi
    call string_length
    mov rdi, r10

    ; for movsb
    xchg rdi, rsi
    mov r10, rdi

    ; Increment for null char.
    inc rax

    cmp rax, rdx
    jle .loop
    xor rax, rax
    ret

.loop:
    movsb
    dec rax
    jz .end
    jmp .loop

.end:
    mov rax, r10
    ret
