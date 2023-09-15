bits 64

global _main
extern _puts
extern _getchar

section .text

_main:
    push rbx
    call _getchar
    mov rax, message
    mov rdi, message
    call _puts
    pop rbx
    mov rax, 0
    ret


section .data

message: resb 50