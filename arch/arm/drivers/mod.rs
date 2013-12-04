use super::cpu::interrupt;
use super::io;
use core::option::{Option, None};

pub unsafe fn init(table: interrupt::table) {
    table.enable(6, keypress as u32);
}

pub static mut keydown: Option<extern fn(char)> = None;

#[no_mangle]
pub unsafe fn keypress() {
    keydown.map(|f| {
        let x = *io::UART0 as u8 as char;
	if(x != '\r') {f(x);}
	else {f('\n');}
    });

    asm!("pop {r11, lr}
          subs pc, lr, #4");
}
