ENTRY(_start)

SECTIONS {
    . = 0x40000000;

    kernel_start = .;

    .text.init : {
        boot.o(.text)
    }

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss COMMON)
    }
    . = ALIGN(8);

    . += 0x400000;
    stack_top = .;

    kernel_end = .;
}
