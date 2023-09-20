; defines stuff for strings

global _string.new
extern _calloc
extern _free

section .text

_string:

.new: ; creates space for a new string, pointer to memory is passed through rdi, this memory should be 8 bytes and will contain the string's allocated length, rsi should contain the desired length of the string
    cmp rsi, 0 ; see if length given is zero
    jne .new.yeslen
    mov rsi, 2
    .new.yeslen:
    ; mov [rdi], rsi
    mov rdi, rsi
    mov rsi, 1
    call _calloc
    ret