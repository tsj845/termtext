bits 64

extern _putchar

global _printnum

section .text

_printnum: ; prints a number to stdout in decimal, number is provided through rdi
    push rbx
    push r12
    mov rbx, rdi ; preserve input
    mov r12, digits ; digit memory
    xor r10, r10 ; current decimal degit being checked
    .loop_head:
    inc r10 ; increment digit counter
    mov rcx, 10 ; set up divisor
    mov rax, rbx ; move dividend to rax for division
    xor rdx, rdx ; ensure rdx is cleared
    div rcx ; divide rdx:rax by rcx
    ; sub rbx, rdx ; get rid of remainder to ensure no non-int division later
    mov r8, rax ; remember quotient
    mov rax, rdx ; set up division
    xor rdx, rdx ; clear rdx
    div rcx ; normalize remainder to be 0 - 10
    mov r11, [byte r12+rax] ; get digit char
    push qword r11 ; push to stack so number is printed in correct order
    mov rbx, r8 ; use quotient
    cmp rbx, rbx ; check if whole number has been converted
    jmp .end
    jnz .loop_head
    mov r12, r10 ; move number of deci digits into a callee-save register
    .lh2:
    dec r12
    pop rdi
    call _putchar ; print char
    cmp r12, r12
    jnz .lh2
    .end:
    pop r12
    pop rbx
    xor rax, rax ; return value
    ret

section .data

digits: db "0123456789",0,0,0,0,0,0,0,0,0,0,0,0