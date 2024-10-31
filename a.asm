section .data
jay db 'jay', 0
message_0 db 'hi', 0
message_len_0 equ $ - message_0

section .text
global _start

_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, message_0
    mov rdx, message_len_0
    syscall

    mov rax, 60
    xor rdi, rdi
    syscall
