bits 64

extern _putchar

global _printnum

section .text

_printnum: ; prints a number to stdout in hex, number is provided through rdi
    push rbx ; preserve registers
    push r12
    mov rbx, rdi ; keep input
    xor r12, r12 ; curr digit
    .loop_head:
    mov rax, 4 ; get bit shift amount
    xor rdx, rdx
    mul r12 ; result in rax
    
    mov rdx, 60 ; start with high bits to put in big endian order
    sub rdx, rax
    mov cl, dl

    mov rax, rbx
    shr rax, cl ; shift desired 4 bits into least significant bits

    and rax, 0x000000000000000f ; mask out all but last 4 bits

    mov r8, digits
    add r8, rax
    
    mov rdi, [r8] ; print char
    call _putchar

    inc r12
    cmp r12, 16
    jne .loop_head
    .end:
    mov rdi, 10
    call _putchar
    mov rdi, 13
    call _putchar
    pop r12
    pop rbx
    ret

; _printnumO: ; prints a number to stdout in decimal, number is provided through rdi
;     push rbx
;     push r12
;     mov rbx, rdi ; preserve input
;     mov r12, digits ; digit memory
;     xor r10, r10 ; current decimal degit being checked
;     .loop_head:
;     inc r10 ; increment digit counter
;     mov rcx, 10 ; set up divisor
;     mov rax, rbx ; move dividend to rax for division
;     xor rdx, rdx ; ensure rdx is cleared
;     div rcx ; divide rdx:rax by rcx
;     ; sub rbx, rdx ; get rid of remainder to ensure no non-int division later
;     mov r8, rax ; remember quotient
;     mov rax, rdx ; set up division
;     xor rdx, rdx ; clear rdx
;     div rcx ; normalize remainder to be 0 - 10
;     add rax, r12
;     mov r11, [byte rax] ; get digit char
;     mov rdi, r11
;     call _putchar
;     mov rdi, 10
;     call _putchar
;     jmp .end
;     push r11 ; push to stack so number is printed in correct order
;     mov rbx, r8 ; use quotient
;     cmp rbx, rbx ; check if whole number has been converted
;     jmp .end
;     jnz .loop_head
;     mov r12, r10 ; move number of deci digits into a callee-save register
;     .lh2:
;     dec r12
;     pop rdi
;     call _putchar ; print char
;     cmp r12, r12
;     jnz .lh2
;     .end:
;     pop r12
;     pop rbx
;     xor rax, rax ; return value
;     ret

section .data

digits: db "0123456789abcdef",0,0,0,0,0,0,0,0,0,0,0,0