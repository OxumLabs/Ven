section .data

    name db 'jay', 0
    message_0 db 'hello', 0
    message_1 db ' how are you', 0
    message_2 db 0x0A, 0

section .text
global _start

_start:
    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    mov rsi, message_0
    xor rdx, rdx
    mov rcx, rsi
    find_length_3:
        cmp byte [rcx], 0
        je done_length_3
        inc rcx
        inc rdx
        jmp find_length_3
done_length_3:
    syscall

    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    mov rsi, name
    xor rdx, rdx
    mov rcx, rsi
    find_length_4:
        cmp byte [rcx], 0
        je done_length_4
        inc rcx
        inc rdx
        jmp find_length_4
done_length_4:
    syscall

    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    mov rsi, message_1
    xor rdx, rdx
    mov rcx, rsi
    find_length_5:
        cmp byte [rcx], 0
        je done_length_5
        inc rcx
        inc rdx
        jmp find_length_5
done_length_5:
    syscall

    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    mov rsi, message_2
    xor rdx, rdx
    mov rcx, rsi
    find_length_6:
        cmp byte [rcx], 0
        je done_length_6
        inc rcx
        inc rdx
        jmp find_length_6
done_length_6:
    syscall

    mov rax, 60         ; sys_exit
    xor rdi, rdi
    syscall
