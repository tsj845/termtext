; defines stuff for strings
bits 64

global _string.new
global _string.drop

extern _buf.new
extern _buf.drop

section .text

_string:

.new: ; creates space for a new string, pointer to memory is passed through rdi, this memory should be 24 bytes, the lower 8 will contain the string's address in memory, the middle eight will contain the string's actual length, and the upper 8 will contain the string's allocated capacity, rsi should contain the desired length of the string in characters
    ; returns 0 on success, 1 on null pointer
    cmp rsi, 1 ; see if length given is zero
    jge .new.yeslen
    mov rsi, 1
    .new.yeslen:
    call _buf.new
    ret

.drop: ; destructor
    call _buf.drop
    ret