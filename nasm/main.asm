bits 64

global _main
extern _puts
extern _putchar
extern _getchar
; extern _free

; extern _string.new

; section .text

extern _calloc
extern _free

section .text

_string:

.new: ; creates space for a new string, pointer to memory is passed through rdi, this memory should be 8 bytes and will contain the string's allocated length, rsi should contain the desired length of the string
    cmp rsi, 0 ; see if length given is zero
    jne .new.yeslen
    mov rsi, 2
    .new.yeslen:
    mov [rdi], rsi
    mov rdi, rsi
    mov rsi, 1
    call _calloc
    ret

; calls _puts then prints a carriage return
prints:
    call _puts
    mov rdi, 13
    call _putchar
    ret

debprnt:
    mov rdi, debmsg
    call prints
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

    mov rdi, strlen

    call _string.new

    call debprnt

    cmp rax, 0
    je err

    mov cl, 0x61
    ; mov [rax], cl
    mov rbx, rax

    mov rdi, rax
    ; call prints

    mov rdi, rbx

    call _free

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
strlen: db 0,0,0,0,0,0,0,0
debmsg: db "DBMSG",0