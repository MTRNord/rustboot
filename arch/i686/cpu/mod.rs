use core::mem::size_of;
use core::mem::uninit;
use core::option::{Option, None, Some};
use core::slice;
use core;

pub use self::idt::IdtEntry;
use util::rt;
use cpu::exception::{Breakpoint, exception_handler};
use kernel::heap;
use kernel::Kernel;
use kernel::mm::physical;
use kernel;
// use vec::Vec;

mod gdt;
mod idt;
mod tss;
pub mod interrupt;
pub mod io;
mod exception;
pub mod mmu;

macro_rules! cpuid(
    ($n:expr, $s1:expr, $s2:expr, $s3:expr, $s4:expr) => (
        asm!("cpuid"
            : "=A"($s1),
              "={ebx}"($s2),
              "={edx}"($s3),
              "={ecx}"($s4)
            : "A"($n) :: "intel");
    );
    ($n:expr, *$s1:expr) => (
        cpuid!($n, (*$s1)[0], (*$s1)[1], (*$s1)[2], (*$s1)[3]);
    );
    ($e:expr) => (
        {
            let mut eax: u32 = $e as u32;
            let ebx: u32;
            let ecx: u32;
            let edx: u32;
            asm!("cpuid"
                : "+A"(eax), "={ebx}"(ebx), "={edx}"(edx), "={ecx}"(ecx)
                ::: "intel")
            (eax, ebx, edx, ecx)
        }
    );
)

// TODO: apic

// call TrapFrame / TrapCallStack?
// TODO: make push_dummy push ds?
// exception info and processor state saved on stack
pub struct Context {
    // Registers saved by the ISR (in reverse order)
    pub edi: u32, pub esi: u32, pub ebp: u32, pub esp: u32,
    pub ebx: u32, pub edx: u32, pub ecx: u32, pub eax: u32,
    pub ds:  u32, pub es:  u32, pub fs:  u32, pub gs: u32,
    pub int_no: u32,   // added by ISRs
    pub err_code: u32, // added by some exceptions
    pub call_stack: IsrCallStack
}

// the cpu adds these when calling the ISR
struct IsrCallStack {
    eip: u32, cs: u32, eflags: u32, esp: u32, ss: u32
}

impl Context {
    pub fn syscall_args(&self) -> [u32, ..6] {
        [self.ebx, self.ecx, self.edx, self.esi, self.edi, self.ebp]
    }

    unsafe fn save() -> &mut Context {
        let this: &mut Context;
        asm!("push gs
              push fs
              .byte 0x06 // push es
              .byte 0x1e // push ds
              pusha"
            : "={esp}"(this) ::: "volatile", "intel");
        this
    }

    unsafe fn restore() {
        asm!("popa
              .byte 0x1f // pop ds
              .byte 0x07 // pop es
              pop fs
              pop gs
              add esp, 8
              iretd"
            :::: "volatile", "intel");
    }

    unsafe fn switch(&self, data_desc: u16, tss_desc: u16) { //-> ! {
        // TODO: ensure is pushed
        // use platform::runtime;
        rt::breakpoint();
        // let call_stack = self.call_stack; //uninit()
        asm!("//mov eax, $0
              // ctx.ss: stack (data) descriptor
              //push dword (7 shl 3) + 3
              // ctx.call_stack.esp: stack pointer
              //push eax
              // ctx.eflags
              //pushf
              // ctx.eflags |= IF (user interrupt enable: sti)
              //or dword[ss:esp], 200h
              //mov eax, dword[ss:esp]
              //or eax, 200h
              //mov dword[TSS.START + 9*4], 0x3202
              // ; code descriptor
              // push dword (6 shl 3) + 3
              // push ring3

              // load TSS
              // mov ax, (3 shl 3) ;+3
              ltr $1 // ref http://www.rz.uni-karlsruhe.de/rz/docs/VTune/reference/

              // mov ax, (7 shl 3) + 3
              mov ds, $0
              mov es, $0
              mov fs, $0
              mov gs, $0

              iretd" // http://faydoc.tripod.com/cpu/iretd.htm
            :: "r"(data_desc), "r"(tss_desc), "{esp}"(&self.call_stack) // could it work?
            : "ax" : "volatile", "intel")
        // which clobber
        // ctx.useresp vs ctx.esp??
        // forget(tmp);
    }
}

struct LocalSegment {
    ts: tss::TssEntry,
    kernel: *Kernel,
    // &cpu info
    // &proc
}

pub static mut desc_table: Option<gdt::Gdt> = None;

pub fn init(kernel: &mut Kernel) -> physical::Phys<mmu::PageDirectory> {
    use cpu::gdt::{Gdt, GdtEntry, SIZE_32, STORAGE, CODE_READ, DATA_WRITE, DPL3};

    let local_data = unsafe {
        heap::zero_alloc::<LocalSegment>(1)
    };
    let tls = unsafe {
        let seg = heap::zero_alloc::<u32>(32);
        *seg = local_data as u32;
        // *(mut_offset(seg, 12)) = 0; // TODO: record stack bottom later
        seg
    };

    let t = Gdt::new();
    t.enable(1, GdtEntry::flat(STORAGE | CODE_READ, SIZE_32));
    t.enable(2, GdtEntry::flat(STORAGE | DATA_WRITE, SIZE_32));
    t.enable(3, GdtEntry::flat(STORAGE | CODE_READ | DPL3, SIZE_32));
    t.enable(4, GdtEntry::flat(STORAGE | DATA_WRITE | DPL3, SIZE_32));
    t.enable(5, GdtEntry::new(tls as u32, 32 * 4, STORAGE | DPL3, SIZE_32));
    unsafe {
        t.enable(6, (*local_data).ts.gdt_entry());
    }
    t.load(1 << 3, 2 << 3, 5 << 3);

    unsafe {
        desc_table = Some(t);

        kernel.interrupts.set_isr(Breakpoint, false, exception_handler());

        mmu::init(kernel)

        // (*ctx).switch((5 << 3) + 3, (3 << 3));
    }
}

pub fn info() {
    unsafe {
        use platform::io;
        let (a, _, _, _) = cpuid!(0);
        io::puti(a as int);
    }
}
