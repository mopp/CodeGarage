bits 64

%define prime_limit 1280
section .data

; Original version.
prime_sieve_bytes:
    db 0, 0, 1
%assign n 3
%rep prime_limit
    %assign current 1
    %assign i 1
        %rep n / 2
           %assign i i + 1
           %if (n % i) = 0
                %assign current 0
                %exitrep
           %endif
        %endrep
db current ; n
    %assign n n+1
%endrep

; Bit table version.
prime_sieve_bits:
    db 0b10101100
%assign count_bytes prime_limit / 8
%assign cnt 8
%rep count_bytes
    %assign bit 0
    %assign mask 1
    %rep 8 ; 8bits
        %assign i 1
        %assign is_p 1
        %rep cnt / 2
           %assign i i + 1
           %if (cnt % i) = 0
                %assign is_p 0
                %exitrep
           %endif
        %endrep

        %if is_p = 1
            %assign bit bit | mask
        %endif
        %assign mask mask << 1

        %assign cnt cnt + 1
    %endrep
    db bit
%endrep

msg_succeed:
    db 0x0A, "Succeed !", 0x0A, 0

msg_failed:
    db 0x0A, "Failed...", 0x0A, 0



section .text
global _start
_start:
    push rbx
    xor rbx, rbx

.loop:
    cmp rbx, prime_limit
    je .break

    ; Result 1
    mov rdi, rbx
    call is_prime_bytes
    mov r8, rax

    ; Result 2
    mov rdi, rbx
    call is_prime_bits
    mov r9, rax

    cmp rax, 1
    jne .skip_print_prime

    ; Print prime number.
    mov rdi, rbx
    call print_uint
    mov rdi, ','
    call print_char

.skip_print_prime:
    inc rbx

    ; Compare the two results.
    cmp r8, r9
    je .loop

    mov rdi, msg_failed
    call print_string

.break:
    pop rbx

    mov rdi, msg_succeed
    call print_string

    ; Exit
    mov rax, 60
    xor rdi, rdi
    syscall


; bool is_prime_bytes(size_t n).
is_prime_bytes:
    xor rax, rax
    mov al, [prime_sieve_bytes + rdi]
    ret


; bool is_prime_bytes(size_t n).
is_prime_bits:
    mov rax, rdi
    shr rax, 3

    mov al, [prime_sieve_bits + rax]

    mov rcx, rdi
    and rcx, 0b111
    shr al, cl
    and rax, 0x1
    ret


string_length:
    mov rax, rdi

.loop:
    cmp byte [rax], 0
    je .end

    inc rax
    jmp .loop

.end:
    sub rax, rdi
    ret


print_string:
    mov rsi, rdi
    call string_length

    mov rdi, 1
    mov rdx, rax
    mov rax, 1
    syscall

    ret


print_newline:
    mov rdi, 0x0A


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
