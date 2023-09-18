bits 64

global _main
extern _puts
extern _putchar
extern _getchar

section .text

; calls _puts then prints a carriage return
prints:
    call _puts
    mov rdi, 13
    call _putchar
    ret

; prints a newline and carriage return
printnewline:
    mov rdi, 10
    call _putchar
    mov rdi, 13
    call _putchar
    ret

_main:
    push rbx

    ; counter, using rbx because rbx is required to be preserved through calls
    mov rbx, 0
    .top:
    call _getchar ; get user input
    mov rdi, rax ; store in rdi for printing and r14 for newline detection as r14 is call safe
    mov r14, rax
    call _putchar
    cmp r14, 10 ; compare
    jne .nogetcharlf ; skip line feed code
    mov rdi, 13 ; print carriage return
    call _putchar
    jmp .end ; end early
    .nogetcharlf:
    mov rcx, message ; store character
    mov [rcx+rbx], al
    inc rbx ; increment counter
    cmp rbx, 3 ; end after three iterations
    jne .top
    call printnewline
    .end:
    mov rdi, message ; echo input
    call prints
    mov rax, 0 ; exit with code 0
.exit:
    pop rbx
    ret

; for errors
err:
    call printnewline
    mov rax, 1
    jmp _main.exit

section .data

message: db 0,0,0,0