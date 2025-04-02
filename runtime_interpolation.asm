section .data
newline: db 0x0A   ;; defines newline
    int_buffer: times 16 db 0  ;; Buffer for integer/float string conversion
    str_0: db "Initial counter value: ", 0
    str_1: db "Counter after +1: ", 0
    str_2: db "Counter after +5 more: ", 0
    str_3: db "Enter a number to add to counter: ", 0
    str_4: db "Counter after your input: ", 0
    str_5: db "Final counter value (doubled): ", 0

section .bss
    num_buffer: resb 20   ;; reserve 20 bytes for num_buffer
    var_0: resq 1   ;; counter is non-string, 8 bytes

section .text
global _start
_start:
    ;; _start: program entry point
    ;; Variable 'counter' already declared in data/bss section
    ;; Initialize variable 'counter'
    mov qword [var_0], 0   ;; set value of 'counter'
    ;; Print text segment "Initial counter value: "
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_0   ;; address of string
    mov rdx, 23   ;; length of string
    syscall
    ;; Print runtime variable value 'counter'
    mov rax, [var_0]   ;; load integer variable at runtime
    call print_int
    mov rax, 1   ;; syscall: write newline
    mov rdi, 1   ;; file descriptor
    mov rsi, newline   ;; address of newline
    mov rdx, 1   ;; length of newline
    syscall
    ;; Math operation on variable 'counter'
    mov rax, [var_0]   ;; load variable
    add rax, 1   ;; add literal value
    mov [var_0], rax   ;; store result in variable
    ;; Print text segment "Counter after +1: "
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_1   ;; address of string
    mov rdx, 18   ;; length of string
    syscall
    ;; Print runtime variable value 'counter'
    mov rax, [var_0]   ;; load integer variable at runtime
    call print_int
    mov rax, 1   ;; syscall: write newline
    mov rdi, 1   ;; file descriptor
    mov rsi, newline   ;; address of newline
    mov rdx, 1   ;; length of newline
    syscall
    ;; Math operation on variable 'counter'
    mov rax, [var_0]   ;; load variable
    add rax, 5   ;; add literal value
    mov [var_0], rax   ;; store result in variable
    ;; Print text segment "Counter after +5 more: "
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_2   ;; address of string
    mov rdx, 23   ;; length of string
    syscall
    ;; Print runtime variable value 'counter'
    mov rax, [var_0]   ;; load integer variable at runtime
    call print_int
    ;; Print text segment "Enter a number to add to counter: "
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_3   ;; address of string
    mov rdx, 34   ;; length of string
    syscall
    mov rax, 1   ;; syscall: write newline
    mov rdi, 1   ;; file descriptor
    mov rsi, newline   ;; address of newline
    mov rdx, 1   ;; length of newline
    syscall
    ;; Reading input into variable 'counter'
    ;; Reading numeric input
    mov rax, 0   ;; syscall: read
    mov rdi, 0   ;; file descriptor: stdin
    mov rsi, num_buffer   ;; buffer to store input
    mov rdx, 19   ;; max bytes to read
    syscall   ;; call read syscall
    mov rdi, num_buffer   ;; prepare string for trimming
    call trim_newline   ;; remove trailing newline
    ;; Convert string to integer
    mov rsi, num_buffer   ;; buffer with number string
    xor rax, rax   ;; clear accumulator
    xor rcx, rcx   ;; clear counter
.convert_loop_counter:
    mov bl, [rsi + rcx]   ;; get next character
    cmp bl, 0   ;; check for end of string
    je .done_convert_counter   ;; if at end, we're done
    sub bl, '0'   ;; convert ASCII to digit value
    imul rax, 10   ;; multiply accumulator by 10
    add rax, rbx   ;; add new digit
    inc rcx   ;; move to next character
    jmp .convert_loop_counter
.done_convert_counter:
    mov [var_0], rax   ;; store numeric value in variable
    ;; Print text segment "Counter after your input: "
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_4   ;; address of string
    mov rdx, 26   ;; length of string
    syscall
    ;; Print runtime variable value 'counter'
    mov rax, [var_0]   ;; load integer variable at runtime
    call print_int
    mov rax, 1   ;; syscall: write newline
    mov rdi, 1   ;; file descriptor
    mov rsi, newline   ;; address of newline
    mov rdx, 1   ;; length of newline
    syscall
    ;; Math operation on variable 'counter'
    mov rax, [var_0]   ;; load variable
    imul rax, 2   ;; multiply by literal value
    mov [var_0], rax   ;; store result in variable
    ;; Print text segment "Final counter value (doubled): "
    mov rax, 1   ;; syscall: write
    mov rdi, 1   ;; file descriptor
    mov rsi, str_5   ;; address of string
    mov rdx, 31   ;; length of string
    syscall
    ;; Print runtime variable value 'counter'
    mov rax, [var_0]   ;; load integer variable at runtime
    call print_int
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
