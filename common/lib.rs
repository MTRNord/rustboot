#![crate_id = "main#0.2.2"]
#![crate_type = "lib"]
#![no_std]
#![feature(asm, macro_rules, default_type_params, phase)]

#[phase(syntax, link)]
extern crate core;

#[cfg(target_arch = "x86")]
pub use platform = arch::i686;
#[cfg(target_arch = "x86")]
pub use arch::i686::cpu;
#[cfg(target_arch = "x86")]
pub use platform::runtime::{memset, memcpy, memmove};

#[cfg(target_arch = "x86_64")]
use platform = self::arch::x86_64;
#[cfg(target_arch = "x86_64")]
pub use platform::efi;
#[cfg(target_arch = "x86_64")]
pub use platform::runtime;

#[cfg(target_arch = "arm")]
use platform = arch::arm;
#[cfg(target_arch = "arm")]
pub use rust_core::support::{memcpy, memmove};

pub use arch::common;
pub use kernel::util;
// visibility trick
// use core::{fail, mem, clone, cmp, ops, option, slice, container, iter, ptr, uint};
// pub mod heap {
//     pub use kernel::{malloc_raw, free, realloc_raw};
// }
// #[path = "rust-core/core/heap_closure.rs"]
// pub mod heap_closure;
// #[path = "rust-core/core/vec.rs"]
// pub mod vec;

mod macros;

pub mod kernel;

#[macro_escape]
mod rust_core;

mod arch {
    pub mod common;

    #[cfg(target_arch = "x86")]
    pub mod i686 {
        pub mod cpu;
        pub mod io;
        pub mod drivers;
        #[allow(dead_code)]
        pub mod runtime;
    }

    #[cfg(target_arch = "x86_64")]
    pub mod x86_64 {
        pub mod cpu;
        pub mod io;
        pub mod drivers;
        pub mod runtime;
        pub mod efi;
    }

    #[cfg(target_arch = "arm")]
    pub mod arm {
        pub mod cpu;
        pub mod io;
        pub mod drivers;
    }
}