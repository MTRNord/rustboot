use core::mem::transmute;

use platform::io;
use cpu::Context;

#[repr(u8)]
pub enum Fault {
    DIVIDE_ERROR = 0,
    NMI = 2,
    BREAKPOINT = 3,
    OVERFLOW = 4,
    BOUND_EXCEEDED = 5,
    INVALID_OPCODE = 6,
    NO_MATH_COPROCESSOR = 7,
    DOUBLE_FAULT = 8,
    COPROCESSOR_SEGMENT_OVERUN = 9,
    INVALID_TSS = 10,
    SEGMENT_NOT_PRESENT = 11,
    STACK_SEGMENT_FAULT = 12,
    GENERAL_PROTECTION = 13,
    PAGE_FAULT = 14,
    FLOATING_POINT_ERROR = 16,
    ALIGNMENT_CHECK = 17,
    MACHINE_CHECK = 18,
    SIMD_FP_EXCEPTION = 19,
}

static Exceptions: &'static [&'static str] = &[
    "Divide-by-zero Error",
    "Debug",
    "Non-maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device Not Available",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Invalid TSS",
    "Segment Not Present",
    "Stack-Segment Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",
    "x87 Floating-Point Exception",
    "Alignment Check",
    "Machine Check",
    "SIMD Floating-Point Exception",
    "Virtualization Exception",
];

#[no_split_stack]
#[inline(never)]
unsafe fn blue_screen(stack: &Context) {
    io::puts("Exception ");
    io::puts(Exceptions[stack.int_no]);
    asm!("hlt");
}

#[no_split_stack]
#[inline(never)]
pub unsafe fn exception_handler() -> extern "C" unsafe fn() {
    asm!("jmp skip_exception_handler
      exception_handler_asm:"
        :::: "volatile", "intel");

    // Points to the data on the stack
    let stack_ptr = Context::save();

    if stack_ptr.int_no as u8 == transmute(BREAKPOINT) {
        asm!("debug:" :::: "volatile")
    }
    else {
        blue_screen(stack_ptr);
    }

    Context::restore();

    asm!("skip_exception_handler:"
        :::: "volatile", "intel");

    exception_handler_asm
}

extern { pub fn exception_handler_asm(); }
