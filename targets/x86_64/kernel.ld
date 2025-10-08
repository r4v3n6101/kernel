ENTRY(_start)

SECTIONS
{
    . = 0x100000;
    . = ALIGN(4096);

    _kernel_start = .;

    /* Text section (code) */
    .text : ALIGN(4096)
    {
        *(.text)
        *(.text.*)
    }

    /* Read-only data */
    .rodata : ALIGN(4096)
    {
        *(.rodata)
        *(.rodata.*)
    }

    /* Initialized data */
    .data : ALIGN(4096)
    {
        *(.data)
        *(.data.*)
    }

    /* Uninitialized data (statics) */
    .bss : ALIGN(4096)
    {
        _bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        _bss_end = .;
    }

    /* Symbol marking the end of the kernel */
    _kernel_end = ALIGN(4096);
}
