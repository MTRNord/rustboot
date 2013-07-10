#[allow(ctypes)];
#[no_std];
#[no_core];

use drivers::keyboard;

pub mod zero;
mod drivers {
    pub mod keyboard;
}

#[inline]
pub fn size_of_val<T>(_val: *mut T) -> uint {
    unsafe { zero::size_of::<T>() }
}

#[packed]
struct idt_reg {
    size: u16,
    addr: *mut [idt_entry, ..256],
}

static Present: u8 = 1 << 7;
static PM32Bit: u8 = 1 << 3;

#[packed]
struct idt_entry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_hi: u16
}

fn idt_entry(proc: u32, sel: u16, flags: u8) -> idt_entry {
    idt_entry {
        addr_lo: (proc & 0xffff) as u16,
        sel: sel,
        zero: 0,
        flags: flags | 0b110,
        addr_hi: (proc >> 16) as u16
    }
}

enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

fn range(lo: uint, hi: uint, it: &fn(uint)) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

unsafe fn clear_screen(background: Color) {
    range(0, 80*25, |i| {
        *((0xb8000 + i * 2) as *mut u16) = (background as u16) << 12;
    });
}


unsafe fn pic_remap() {
    asm!("
        mov al, 0x11
        mov dx, 0x20
        out dx, al
        mov dx, 0xA0
        out dx, al

        mov al, 0x20
        mov dx, 0x21
        mov bx, 0xA1
        out dx, al
        mov al, 0x28
        xchg bx, dx
        out dx, al

        mov al, 4
        xchg bx, dx
        out dx, al
        mov al, 2
        xchg bx, dx
        out dx, al

        mov al, 1
        xchg bx, dx
        out dx, al
        xchg bx, dx
        out dx, al

        mov al, 0xff
        xchg bx, dx
        out dx, al
        xchg bx, dx
        out dx, al"
        ::: "al", "bx", "dx" : "volatile", "intel");
}

#[inline(never)]
unsafe fn pic_enable(irq: u8) {
    let port: u16 = if (irq & 0b1000) == 0 { 0x21 } else { 0xa1 };
    let mask: u8 = !(1u8 << (irq & 0b111));

    asm!("
        mov dx, $0
        in al, dx
        and al, $1
        out dx, al"
        :: "r"(port), "r"(mask) : "al", "dx" : "intel")
}

#[no_mangle]
extern "C" fn keyup(code: u32) { }

pub static ascii_table: &'static str = "\
\x00\x1B1234567890-=\x08\
\tqwertyuiop[]\n\
\x00asdfghjkl;'`\
\x00\\zxcvbnm,./\x00\
*\x00 ";

fn keydown(code: u32) {
    let screen = 0xb8000 as *mut [u16, ..2000];
    // mutable statics are incorrectly dereferenced in PIC!
    static mut pos: u32 = 0;

    if(code & (1 << 7) == 0) {
        unsafe {
            let char = ascii_table[code];
            if char == 8 && pos > 0 {
                pos -= 1;
                (*screen)[pos] &= 0xff00;
            } else if char == '\n' as u8 {
                pos += 80 - pos % 80;
            } else {
                (*screen)[pos] |= char as u16;
                pos += 1;
            }
        }
    }
}

#[no_mangle]
pub unsafe fn main() {
    clear_screen(LightRed);
    // invalid deref when &fn?
    keyboard::callback = keyboard::Some(keydown);

    let idt = 0x100000 as *mut [idt_entry, ..256];

    (*idt)[keyboard::IRQ] = idt_entry(keyboard::isr_addr(), 1 << 3, PM32Bit | Present);

    let idt_table = 0x100800 as *mut idt_reg;
    *idt_table = idt_reg {
        addr: idt,
        size: size_of_val(idt) as u16
    };

    pic_remap();
    pic_enable(keyboard::IRQ);

    asm!("
        lidt [$0]
        sti"
        :: "n"(idt_table) :: "intel");
}
