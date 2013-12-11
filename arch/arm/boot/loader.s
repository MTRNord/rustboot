.text
.code 32
.syntax unified
.fpu softvfp

.global start
.global memcpy
.global memcmp
.global malloc
.global free
.global abort
.global __aeabi_memcpy
.global __aeabi_memset
.global __modsi3
.global __aeabi_idiv
.global vectors
.global vectors_end
.global irq_handler
.global k_font

.type start, %function

start:
    mov sp, 0x18000
    bl main
memcpy:
memcmp:
malloc:
free:
abort:
__aeabi_memcpy:
__aeabi_memset:
__modsi3:
__aeabi_idiv:
    b .

.type vectors, %object
.size vectors, vectors_end-vectors

vectors:
    ldr pc, start_addr
    b .
    b .
    b .
    b .
    b .
    b .
    b .

start_addr:
.word start

vectors_end:
