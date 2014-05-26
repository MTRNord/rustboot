use super::DtReg;

pub static PRESENT:   u8 = 1 << 7;
pub static INTR_GATE: u8 = 0b1110;
pub static TRAP_GATE: u8 = 0b1111;

pub type IdtReg = DtReg<IdtEntry>;

#[packed]
pub struct IdtEntry {
    addr_ll: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    addr_lh: u16,
    addr_hi: u32,
    zero2: u32
}

impl IdtEntry {
    pub fn new(func: extern fn(), sel: u16, flags: u8) -> IdtEntry {
        let addr = func as uint;
        let (addr_hi, addr_lh, addr_ll) = (
            (addr & 0xFFFFFFFF00000000) >> 32,
            (addr & 0x________FFFF0000) >> 16,
            (addr & 0x____________FFFF)
        );
        IdtEntry {
            addr_ll: addr_ll as u16,
            sel: sel,
            zero: 0,
            flags: flags | 0b110,
            addr_lh: addr_lh as u16,
            addr_hi: addr_hi as u32,
            zero2: 0
        }
    }
}

impl super::DtReg<IdtEntry> {
    #[inline]
    pub fn load(&self) {
        unsafe {
            asm!("lidt [$0]" :: "A"(self) :: "intel");
        }
    }
}
