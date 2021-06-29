global start
extern check_cpuid, check_multiboot, check_long_mode
section .text
bits 32
start:
    mov     esp, stack_top
    
    ; MUST be called first
    call    check_multiboot
    mov     edi, msg_multiboot
    cmp     eax, 0
    jz      error
    
    call    check_cpuid
    mov     edi, msg_cpuid
    cmp     eax, 0
    jz      error

    call    check_long_mode
    mov     edi, msg_longmode
    cmp     eax, 0
    jz      error

    hlt

error:
    mov     ecx, 0xB8000
.L1:
    cmp     ecx, 0xB8FA0
    je      .L2
    mov     BYTE [ecx+eax], 0
    inc     eax
    jmp     .L1
    mov     eax, 0xB8000
.L2:
    mov     al, BYTE [edi]
    cmp     al, 0
    jz      .ERROR_DONE
    mov     BYTE [ecx], al
    mov     BYTE [ecx+1], 0b01000111
    add     ecx, 2
    inc     edi
    jmp     .L1

.ERROR_DONE:
    hlt


                section     .data
msg_longmode:   db          "Long Mode check failed", 0
msg_cpuid:      db          "CPUID check failed", 0
msg_multiboot   db          "Multiboot check failed", 0

section .bss
stack_bottom:
    resb 128
stack_top: