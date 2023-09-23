; defines stuff for buffers
bits 64

global _buf.new
global _buf.drop

extern _calloc
extern _free

_buf:

.new:
    ; creates a new buffer initialized with zeros
    ; rdi must contain an address to 24 bytes of contiguous memory
    ; rsi must contain the desired capacity of the buffer, cannot be zero or negative
    ; returns 0 on success, 1 on null pointer, and 2 non positive capacity
    ; on success, rdi will contain the capacity in the upper 8 bytes, the length in the middle 8, and the pointer to memory in the lower 8
    cmp rsi, 0 ; check that capacity is positive
    jg .new.nonzerocap
    mov rax, 2
    ret
    .new.nonzerocap:
    mov [rdi], rsi ; store capacity
    xor rax, rax ; set length to zero
    mov [rdi+8], rax
    push rbx ; preserve rbx
    mov rbx, rdi
    mov rdi, rsi ; mov capacity to correct register
    mov rsi, 1 ; each item is 1 byte
    call _calloc
    cmp rax, 0 ; check for null pointer
    jnz .new.nnpe
    mov rax, 1
    ret
    .new.nnpe:
    mov [rbx+16], rax ; store address
    xor rax, rax
    pop rbx
    ret

.drop:
    ; destructs a buffer
    ; rdi must contain the address of 24 bytes, the lower 8 of which must be the pointer to the memory
    ; returns 0 on success and zeros all 24 bytes, 1 on failure
    mov rcx, [rdi+16]
    cmp rcx, 0
    jnz .drop.cont ; check for null pointer
    mov rax, 1
    ret
    .drop.cont:
    push rbx ; preserve rbx
    mov rbx, rdi ; save value in rdi
    mov rdi, [rdi+16] ; free memory
    call _free
    ; zero attributes
    xor rcx, rcx
    mov [rbx], rcx
    mov [rbx+8], rcx
    mov [rbx+16], rcx
    xor rax, rax
    pop rbx
    ret