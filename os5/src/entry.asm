    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 4 * 1024  // 16 MB
    .globl boot_stack_top
boot_stack_top: