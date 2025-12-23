/* Raspberry Pi 4 boots at 0x80000 in 64-bit mode */
__rpi4_binary_load_start = 0x80000;

/* Mask for 4-cores processor (0b11) */
CORE_MASK = 0x3;

/* Boot from core_id=0 */
BOOT_CORE = 0;

/* 16KiB stack */
STACK_SIZE = 16K;
PAGE_SIZE = 4K;

ENTRY(_start)

PHDRS
{
    segment_code    PT_LOAD FLAGS(5); /* RX */
    segment_data    PT_LOAD FLAGS(6); /* RO */
    segment_guard   PT_LOAD FLAGS(6); /* RO */
}

SECTIONS
{
    . = __rpi4_binary_load_start;

    .text : {
        KEEP(*(.text._start))
        KEEP(*(.text.kinit))
        *(.text*)
    }:segment_code

    .rodata : ALIGN(8) {
        *(.rodata*)
    }:segment_code

    .data : {
        *(.data*)
    }:segment_data

    /* Write by 2 words, i.e. 2x8 bytes */
    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }:segment_data

    .guard (NOLOAD) : ALIGN (PAGE_SIZE) {
        . += PAGE_SIZE;
    }:segment_guard

    .stack (NOLOAD) : ALIGN(8) {
        . += STACK_SIZE;

        __stack_top = .;
    }
}
