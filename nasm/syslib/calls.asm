bits 64

;;;; SYSCALL LIBRARIES

; FILE obj fmt
; fd<8>, flags<2>, bufptr?<8>, _PADDING<6>
; FILE -> flags
; 0b0000000000000001 -> using buffer, if false, bufptr should be NULL
; 0b0000000000000010 -> readable
; 0b0000000000000100 -> writable
; 0b0000000000001000 -> executable

;; global derived functions
global __memalloc
global __memfree

;; global functions (raw syscall wrappers)
global __open
global __close
global __read
global __write
global __fork
global __exit
global __getpid
global __kill
global __ppid
; global __mmap
; global __munmap

;; global variables / macros
%ifdef MACH64
%define O_RDONLY 0
%define O_WRONLY 1
%define O_RDWR   2
%endif

;; BSD MASK 0x2000000

section .text

%ifdef MACH64
__exit: ; CODE 1, exit code in rdi, does not return
    mov rax, 0x2000001
    syscall
__open: ; CODE 5
    mov rax, 0x2000005
__close: ; CODE 6
    mov rax, 0x2000006
__read: ; CODE 3
    mov rax, 0x2000003
__write: ; CODE 4
    mov rax, 0x2000004
__lseek: ; CODE 199
    mov rax, 0x20000c7
__ftrucate: ; CODE 201
    mov rax, 0x20000c9
__fork: ; CODE 2
    mov rax, 0x2000002
__getpid: ; CODE 20
    mov rax, 0x2000014
__ppid: ; CODE 39
    mov rax, 0x2000027
__kill: ; CODE 37
    mov rax, 0x2000025
__mmap: ; CODE 197
    mov rax, 0x20000c5
__munmap: ; CODE 73
    mov rax, 0x2000049
%endif