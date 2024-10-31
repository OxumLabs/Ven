section .data
jay db 'jay', 0
message_0 db 'hi', 0
message_len_0 equ $ - message_0

section .text
global _start

_start:
    mov eax, 4
    mov ebx, 1
    mov ecx, message_0
    mov edx, message_len_0
    int 0x80

    mov eax, 1
    xor ebx, ebx
    int 0x80
