section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))


    ; MULTIBOOT_MEMORY_INFO
    dd 0x00000000    ; header_addr 
    dd 0x00000000    ; load_addr 
    dd 0x00000000    ; load_end_addr 
    dd 0x00000000    ; bss_end_addr 
    dd 0x00000000    ; entry_addr 

    ; MULTIBOOT_VIDEO_MODE
    dd 0x00000000    ; mode_type 
    dd 1280          ; width 
    dd 1024          ; height 
    dd 32            ; depth 

    ; required end tag
;    dw 0    ; type
;    dw 0    ; flags
;    dd 8    ; size
header_end: