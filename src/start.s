.section .text
.global _start

_start:

.L_mask_cores:
    mrs	x1, MPIDR_EL1
    ldr x2, =CORE_MASK
	and	x1, x1, x2
    ldr x2, =BOOT_CORE
	cmp	x1, x2
	b.ne .L_parking_loop

.L_kernel_init:
    b kinit

.L_parking_loop:
	wfe
	b .L_parking_loop
