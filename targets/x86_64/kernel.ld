ENTRY(_start)

SECTIONS
{
    . = 0x100000;
    . = ALIGN(4096);

    _kernel_start = .;

    /* Text section (code) */
    .text :
    {
        *(.text)
        *(.text.*)
    }

    /* Read-only data */
    .rodata :
    {
        *(.rodata)
        *(.rodata.*)
    }

    /* Initialized data */
    .data :
    {
        *(.data)
        *(.data.*)
    }

    /* Uninitialized data (statics) */
    .bss (NOLOAD) :
    {
        *(.bss)
        *(.bss.*)
        *(COMMON)
    }

    /* Symbol marking the end of the kernel */
    _kernel_end = ALIGN(4096);
}
