/* Raspberry Pi 4 boots at 0x80000 in 64-bit mode */
__rpi4_binary_load_start = 0x80000;

/* 16KiB stack */
STACK_SIZE = 16K;
PAGE_SIZE = 4K;

/* 1MiB of VA space and 3 pages (L4, L3, L2) */
INITIAL_PAGES_NUM = 256 + 3;

ENTRY(_start)

PHDRS
{
    segment_code    PT_LOAD FLAGS(5); /* RX */
    segment_data    PT_LOAD FLAGS(6); /* RW */
    segment_rodata  PT_LOAD FLAGS(4); /* RO */
    segment_guard   PT_LOAD FLAGS(4); /* RO */
}

SECTIONS
{
    .text : AT(__rpi4_binary_load_start) {
        KEEP(*(.text.boot))
        KEEP(*(.text.kinit))
        *(.text*)
    }:segment_code

    .rodata : ALIGN(8) {
        *(.rodata*)
    }:segment_rodata

    .data : ALIGN(8) {
        *(.data*)
    }:segment_data

    .got : ALIGN(8) {
        __got_start = .;
        *(.got*)
        __got_end = .;
    }

    .rela.dyn : ALIGN(8) {
        __rela_dyn_start = .;
        *(.rela.dyn*)
        __rela_dyn_end = .;
    }

    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }:segment_data

    .initial_pages (NOLOAD) : ALIGN(PAGE_SIZE) {
        . += PAGE_SIZE * INITIAL_PAGES_NUM;
    }:segment_data

    .guard (NOLOAD) : ALIGN(PAGE_SIZE) {
        . += PAGE_SIZE;
    }:segment_guard

    .stack (NOLOAD) : ALIGN(16) {
        . += STACK_SIZE;
        __boot_stack_top = .;
        *(.stack*)
        . = ALIGN(16);
    }:segment_data

    .guard (NOLOAD) : ALIGN(PAGE_SIZE) {
        . += PAGE_SIZE;
    }:segment_guard
}
