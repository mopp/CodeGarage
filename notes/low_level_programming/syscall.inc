; vi:ft=nasm
bits 64
section .text

write:
    mov rax, 1
    syscall
    ret

; int open(const char *pathname, int flags);
; int open(const char *pathname, int flags, mode_t mode);
%define O_APPEND 0x1000
%define O_CREAT  0x0040
%define O_TRUNC  0x0200
%define O_RDWR   0x0002
%define O_WRONLY 0x0001
%define O_RDONLY 0x0000
open:
    mov rax, 2
    syscall
    ret


; int stat(const char *pathname, struct stat *buf);
stat:
    mov rax, 4
    syscall
    ret


; void* mmap(void* addr, size_t length, int prot, int flags, int fd, off_t offset)
%define PROT_EXEC  0x4
%define PROT_READ  0x1
%define PROT_WRITE 0x2
%define PROT_NONE  0x0
%define MAP_SHARED    0x01
%define MAP_PRIVATE   0x02
%define MAP_ANONYMOUS 0x20
%define MAP_FIXED     0x10
mmap:
    mov rax, 9
    syscall
    ret


; void exit(int status)
exit:
    mov rax, 60
    syscall

