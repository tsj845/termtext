bits 64

global _main
; extern _puts
; extern _putchar
; extern _getchar
; extern _free

extern __exit

extern _printnum

extern _io_init
extern __writechar
common _stdout 8

extern _string.new
extern _string.drop

; section .text

; extern _calloc
; extern _free

section .text

; _string:

; .new: ; creates space for a new string, pointer to memory is passed through rdi, this memory should be 8 bytes and will contain the string's allocated length, rsi should contain the desired length of the string
;     cmp rsi, 1 ; see if length given is zero
;     jg .new.yeslen
;     mov rsi, 2
;     .new.yeslen:
;     mov [rdi], rsi
;     mov rdi, rsi
;     mov rsi, 1
;     call _calloc
;     ret

; calls _puts then prints a carriage return
prints:
    call _puts
    mov rdi, 13
    mov rsi, _stdout
    call __writechar
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

    mov rdi, 15
    call _printnum

    mov rdi, 0
    call __exit

    call _io_init

    cmp rax, 10

    jne .initgood
    pop rbx
    ret
    .initgood:

    mov rdi, 0x61
    mov rsi, _stdout

    call __writechar

    call debprnt

    mov rdi, strprops

    mov rsi, 2

    call _string.new

    cmp rax, 0
    jnz err

    mov rdx, strprops
    mov rax, [rdx+16]

    mov cl, 0x61
    mov [rax], cl
    mov rbx, rax

    mov rdi, rax
    call prints

    mov rdi, rbx
    mov rsi, strprops

    call _string.drop

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
strprops:
    .cap: db 0,0,0,0,0,0,0,0
    .len: db 0,0,0,0,0,0,0,0
    .ptr: db 0,0,0,0,0,0,0,0
debmsg: db "DBMSG",0