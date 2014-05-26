use core::ptr::{copy_nonoverlapping_memory, set_memory};
use core::intrinsics::offset;
use core::mem::transmute;
use core::option::{Option, Some, None};
use core::str::StrSlice;
use core;

use kernel::process::Process;
use kernel::mm;
use util::ptr::mut_offset;

use self::elf32::Ehdr;
use self::elf64::Ehdr;

mod elf32;
mod elf64;

#[repr(u32)]
enum HeaderType {
    PT_NULL = 0,
    PT_LOAD = 1,
    PT_DYNAMIC = 2,
    PT_INTERP = 3,
    PT_NOTE = 4,
    PT_SHLIB = 5,
    PT_PHDR = 6,
    PT_TLS = 7,
    PT_LOOS = 0x60000000,
    PT_HIOS = 0x6fffffff,
    PT_LOPROC = 0x70000000,
    PT_HIPROC = 0x7fffffff
}

pub trait Ehdr {
    unsafe fn load(&self) -> extern "C" fn();
}

#[packed]
struct ELFIdent {
    ei_mag: [u8, ..4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8, ..7]
}

impl self::Ehdr {
    pub unsafe fn spawn_process(&self) -> Process {
        let mut task = Process::new();
        //TODO: Verify file integrity
        let buffer: *u8 = transmute(self);
        let ph_size = self.e_phentsize as int;
        let ph_base = offset(buffer, self.e_phoff as int);

        let mut stack_flags = mm::RW;

        int::range(0, self.e_phnum as uint, |i| {
            let pheader = offset(ph_base, ph_size * i as int) as *Phdr;

            match (*pheader).p_type {
                PT_NULL => {}
                PT_LOAD => (*pheader).load(&task, buffer),
                PT_DYNAMIC => (*pheader).load(&task, buffer),
                PT_GNU_STACK => {
                    if !((*pheader).p_flags & !PT_X).is_zero() {
                        // We don't need an executable stack
                        stack_flags = mm::Flags::zero();
                    }
                },
                _ => {}
            }
        });

        static stack_bottom: u32 = 0xC0000000;
        let stack_vaddr = (stack_bottom - 0x1000) as *mut u8;
        task.mmap(stack_vaddr, 0x1000, stack_flags);
        let stack_ptr = mut_offset(stack_bottom as *mut u8, -(((4 + 5 + 15) & !0xF) + 8 + 4 + 4 + 4));
        let argv_ptr = stack_ptr as *mut *mut u8;
        let envp_ptr = mut_offset(argv_ptr, 2);
        let auxv_ptr = mut_offset(argv_ptr, 1) as *mut Auxv;
        let str_ptr = mut_offset(stack_bottom as *mut u8, -(4 + 5));

        *mut_offset(argv_ptr, 1) = transmute(0);
        *envp_ptr = transmute(0);
        *auxv_ptr = Auxv { a_type: AT_NULL, a_un: AuxvValue { data: 0 } };

        let (strs, len): (*u8, uint) = transmute("test\0");
        copy_nonoverlapping_memory(str_ptr, strs, len);
        *argv_ptr = str_ptr;

        copy_nonoverlapping_memory(vaddr, offset(buffer, file_pos), file_size);
        set_memory(mut_offset(vaddr, file_pos + file_size as int), 0, mem_size - file_size);
    }
}

impl elf64::Elf64_Phdr {
    unsafe fn load(&self, buffer: *u8) {
        let vaddr = self.p_vaddr as *mut u8;
        let mem_size = self.p_memsz as uint;
        let file_pos = self.p_offset as int;
        let file_size = self.p_filesz as uint;

        let flags = if !(self.p_flags & PT_W).is_zero() {
            mm::RW
        } else {
            mm::Flags::zero()
        };

        task.mmap(vaddr, mem_size, flags);

        copy_nonoverlapping_memory(vaddr, offset(buffer, file_pos), file_size);
        set_memory(mut_offset(vaddr, file_pos + file_size as int), 0, mem_size - file_size);
    }
}

impl ELFIdent {
    /*
    unsafe fn file(&self) -> Option<&Ehdr> {
        // TODO: check validity, check endianness
        let e32: &Elf32_Ehdr = transmute(self);
        let e64: &Elf64_Ehdr = transmute(self);
        match self.ei_class {
            1 => Some(e32 as &Ehdr),
            2 => Some(e64 as &Ehdr),
            _ => None
        }
    }
    */
    unsafe fn load(&self) -> Option<extern "C" fn()> {
        // TODO: check validity, check endianness
        let e32: &Elf32_Ehdr = transmute(self);
        let e64: &Elf64_Ehdr = transmute(self);
        match self.ei_class {
            1 => Some(e32.load()),
            2 => Some(e64.load()),
            _ => None
        }
    }
}

pub fn exec(buffer: *u8) {
    unsafe {
        let ident: &ELFIdent = transmute(buffer);
        /*ident.file().map(|header| {
            // jump into the module
            header.load()();
        });*/
        ident.load().map(|e| { e() });
    }
}
