; handles program io
bits 64

%define STDIN   0
%define STDOUT  1
%define STDERR  2

; base c io functions
extern _fgetc
extern _fputc
extern _fputs
extern _fsetvbuf
extern _fopen
extern _fclose
extern _close
extern _fdopen
extern _puts

; extern _get_errno
; extern _EBADF
extern ___error

; constants
; extern _IOFBF
; common _stdin 8
; common _stdout 8
; common _stderr 8
global _stdin
global _stdout
global _stderr

; buffers
extern _buf.new
extern _buf.drop

; drivers
global _io_init
global __writechar
; global __readchar
; global __writebuf
; global __readbuf
; global __open
; global __close

section .text

_get_errno:
    mov rax, [qword _errno_ptr]
    ; mov rax, [rax]
    ret

_io_init:
    mov rdx, initialized
    mov cl, [rdx]
    cmp cl, cl
    jnz .inited ; check if already initialized
    mov cl, 1
    mov [rdx], cl ; set initialized flag
    call ___error
    mov rdx, _errno_ptr
    mov [rdx], rax
    mov rdi, STDOUT
    mov rsi, writ
    call _fdopen ; get stdout
    cmp rax, rax ; check success
    jz .npe
    mov rdx, _stdout ; test stdout
    mov [rdx], rax
    mov rsi, rax
    mov rdi, outdb
    call _fputs
    .inited:
    xor rax, rax
    ret
    .npe: ; print error message
    call _get_errno
    mov rsi, _EBADF
    cmp rax, 9
    jne .encf
    mov rdi, ebadfmsg
    call _puts
    .encf:
    mov rdi, badfdopen
    call _puts
    mov rax, 10
    ret

__writechar:
    ; writes a character to specified stream
    ; rdi contains the character
    ; rsi contains the stream pointer, if zero, the set stream will be used if set
    ; cmp rsi, rsi
    ; jnz .nz
    ; mov rsi, _stdout
    ; .nz:
    call _fputc
    ret

section .data

initialized: db 0

_EBADF: db 9

_errno_ptr: db 0,0,0,0,0,0,0,0

_stdout: db 0,0,0,0,0,0,0,0
_stderr: db 0,0,0,0,0,0,0,0
_stdin:  db 0,0,0,0,0,0,0,0
outdb: db "STDOUT INIT DONE",0

writ: db "w",0
badfdopen: db "BAD FDOPEN",0
ebadfmsg: db "EBADF",0