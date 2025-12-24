.section .text._start
.global	_start
.size	_start, . - _start
.type	_start, function

.macro ADR_REL register, symbol
	adrp    \register, \symbol
	add     \register, \register, #:lo12:\symbol
.endm

_start:

.L_drop_privilege:
    mrs x1, CurrentEL

    cmp x1, #0b1100
    beq .L_el3_to_el1

    cmp x1, #0b1000
    beq .L_el2_to_el1

    b .L_el1

.L_el3_to_el1:
    ADR_REL x1, __stack_top
    msr SP_EL1, x1

    mrs x1, SCR_EL3
    orr x1, x1, #(1 << 10)
    msr SCR_EL3, x1
    isb

    mov x1, #(0b0101)
    orr x1, x1, #(0b1111 << 6)
    msr SPSR_EL3, x1

    adr x1, .L_el1
    msr ELR_EL3, x1

    eret

.L_el2_to_el1:
    ADR_REL x1, __stack_top
    msr SP_EL1, x1

    mov x1, #(0b0101)
    orr x1, x1, #(0b1111 << 6)
    msr SPSR_EL2, x1

    adr x1, .L_el1
    msr elr_el2, x1

    eret

.L_el1:
    ADR_REL x1, __stack_top
    mov sp, x1

.L_mask_cores:
    mrs	x1, MPIDR_EL1
	and	x1, x1, {CORE_MASK}
	cmp	x1, {BOOT_CORE}
	b.ne .L_parking_loop

.L_load_bss_addrs:
    ADR_REL	x1, __bss_start
	ADR_REL x2, __bss_end

.L_bss_zeroize:
	cmp	x1, x2
	b.eq	.L_kernel_init
	stp	xzr, xzr, [x1], #16
	b	.L_bss_zeroize

.L_kernel_init:
    b kinit

.L_parking_loop:
	wfe
	b .L_parking_loop
