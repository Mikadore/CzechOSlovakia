global start
extern check_cpuid, check_multiboot, check_long_mode
section .text
bits 32

start:
    mov     esp, stack_top
    push    ebx
    push    eax
    
    ; clear screen
    call    screen_clear

    ; need eax's magic numbe
    pop     eax
    call    check_multiboot
    cmp     eax, 0
    je      check_multiboot_fail
    
    call    check_cpuid
    cmp     eax, 0
    je      check_cpuid_fail

    call    check_long_mode
    cmp     eax, 0
    je      check_long_mode_fail

    hlt 

check_multiboot_fail:
    mov     esi, msg_multiboot
    mov     bl, [color_error]
    call    print_color    
    hlt
check_cpuid_fail:
    mov     esi, msg_cpuid
    mov     bl, [color_error]
    call    print_color    
    hlt
check_long_mode_fail:
    mov     esi, msg_longmode
    mov     bl, [color_error]
    call    print_color    
    hlt

;;;;;;;;;;;;;;;;;;;;;;;
; void screen_clear() ;
;;;;;;;;;;;;;;;;;;;;;;;
screen_clear:
    mov     ecx, 0xB8000
.L1_CLEAR:
    cmp     ecx, 0xB8FA0
    je      .L2_CLEAR
    mov     DWORD [ecx], 0
    add     ecx, 4
    jmp     .L1_CLEAR
.L2_CLEAR:
    ret

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; void print_color(bl: u8 color, esi: u8* text) ;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
print_color:
; print the text
    mov     ecx, 0xB8000
.L1_PRINT:
    mov     al, BYTE [esi]
    cmp     al, 0
    je      .L2_PRINT
    mov     BYTE [ecx], al
    mov     BYTE [ecx+1], bl
    add     ecx, 2
    inc     esi
    jmp     .L1_PRINT

.L2_PRINT:
    ret

                section     .data
color_default:  db          0b11110000
color_error:    db          0b01000111

msg_longmode:   db          "Long Mode check failed", 0
msg_cpuid:      db          "CPUID check failed", 0
msg_multiboot   db          "Multiboot check failed", 0

section .bss
stack_bottom:
    resb 128
stack_top: