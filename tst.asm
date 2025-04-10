section .data
newline: db 0x0A   ;; defines newline
    int_buffer: times 16 db 0  ;; Buffer for integer/float string conversion
    str_0: db "hello! world", 0
    str_1: db "nice to meet u ", 0

section .bss
    num_buffer: resb 20   ;; reserve 20 bytes for num_buffer
    var_0: resb 256   ;; name is string, reserve 256 bytes

section .text
global _start
_start:
    ;; _start: program entry point
    ;; Print string literal: "hello! world"
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_0   ;; address of string
    mov rdx, 12   ;; length of string
    syscall
    mov rax, 1   ;; syscall: write newline
    mov rdi, 1   ;; file descriptor
    mov rsi, newline   ;; address of newline
    mov rdx, 1   ;; length of newline
    syscall
    ;; Variable 'name' already declared in data/bss section
    ;; Initialize variable 'name'
    mov qword [var_0], joy   ;; set value of 'name'
    ;; Reading input into variable 'name'
    ;; Reading string input
    mov rax, 0   ;; syscall: read
    mov rdi, 0   ;; file descriptor: stdin
    lea rsi, [var_0]   ;; buffer to store input
    mov rdx, 255   ;; max bytes to read
    syscall   ;; call read syscall
    lea rdi, [var_0]   ;; prepare string for trimming
    call trim_newline   ;; remove trailing newline
    ;; Print text segment "nice to meet u "
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_1   ;; address of string
    mov rdx, 15   ;; length of string
    syscall
    ;; Print runtime variable value 'name'
    lea rsi, [var_0]   ;; load address of string variable
    call print_str
    mov rax, 1   ;; syscall: write newline
    mov rdi, 1   ;; file descriptor
    mov rsi, newline   ;; address of newline
    mov rdx, 1   ;; length of newline
    syscall

    ;; Exiting program
    mov rax, 60   ;; syscall: exit
    xor rdi, rdi   ;; exit status 0
    syscall

print_int:
    mov rsi, num_buffer + 19   ;; set pointer to end of num_buffer
    mov byte [rsi], 0   ;; null-terminate
    mov rbx, 10   ;; divisor for conversion
.int_to_str:
    dec rsi   ;; move pointer left
    xor rdx, rdx   ;; clear remainder register
    div rbx   ;; divide rax by 10
    add dl, '0'   ;; convert remainder to ASCII digit
    mov [rsi], dl   ;; store digit
    test rax, rax   ;; check if quotient is zero
    jnz .int_to_str   ;; loop if not zero
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor: stdout
    mov rdx, num_buffer + 19
    sub rdx, rsi   ;; compute length of converted string
    syscall   ;; write string
    ret

print_str:
    mov rcx, 0   ;; initialize counter
.count_loop:
    cmp byte [rsi + rcx], 0   ;; check for null terminator
    je .check_newline
    inc rcx
    jmp .count_loop
.check_newline:
    cmp rcx, 0
    je .done
    dec rcx
    cmp byte [rsi + rcx], 0x0A   ;; check if last character is newline
    jne .print
    jmp .done
.print:
    inc rcx
    mov rax, 1
    mov rdi, 1
    mov rdx, rcx   ;; length of string
    syscall
.done:
    ret

trim_newline:
    push rbx
    mov rcx, 0
.trim_loop:
    mov al, [rdi + rcx]
    cmp al, 0   ;; end of string check
    je .done
    cmp al, 0x0A   ;; check for newline
    je .replace
    inc rcx
    jmp .trim_loop
.replace:
    mov byte [rdi + rcx], 0   ;; replace newline with null
.done:
    pop rbx
    ret
