/* kernel::sgash.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::vec::Vec;
use core::mem::Allocator;
use kernel::*;
use super::super::platform::*;

pub static mut buffer: cstr = cstr {
				p: 0 as *mut u8,
				p_cstr_i: 0,
				max: 0
			      };
pub fn putchar(key: char) {
    unsafe {
	/*
	 * We need to include a blank asm call to prevent rustc
	 * from optimizing this part out
	 */
	asm!("");
	io::write_char(key, io::UART0);
    }
}

#[lang = "exchange_free"]
fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	putchar(*c as char);
    }
}

pub unsafe fn output(s: cstr)
{
    let mut x = 0;
    while *(((s.p as uint)+x) as *char) != '\0'
    {
	putchar(*(((s.p as uint)+x) as *char));
	x += 1;
    }
}

pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard
    if (true) {
	match x {
	    13		=>	{ prompt(); }
	    127		=>	{ 
		if (buffer.delete_char()) { 
		    putchar('');
		    putchar(' ');
		    putchar(''); 
		}
	    }
	    _		=>	{ 
		if (buffer.add_char(x)) { putchar(x as char); }
	    }
	}
    }
    else {
	keycode(x);
    }
}

fn keycode(x: u8) {
    let mut x = x;
    while ( x != 0 ) {
	putchar((x%10+ ('0' as u8) ) as char);
	x = x/10;
    }
    putchar(' ');
}

pub unsafe fn init() {
    buffer = cstr::new(256);
    prompt();
}

unsafe fn prompt() {
    if (buffer.eq(&cstr::from_str(&"ls"))) { putstr( &"\na\tb") };
    match buffer.getarg(' ', 0) {
	Some(y)	=> {
	    if(y.eq(&cstr::from_str(&"cat"))) { 
		match buffer.getarg(' ', 1) {
		    Some(x)	=> {
			if(x.eq(&cstr::from_str(&"a"))) { putstr( &"\nHello"); }
			if(x.eq(&cstr::from_str(&"b"))) { putstr( &"\nworld!"); }
		    }
		    None	=> { }
		};
	    }
	}
	None	=> { }
    };

    putstr(&"\nsgash > ");
    buffer.reset();
}

/* BUFFER MODIFICATION FUNCTIONS */

struct cstr {
    p: *mut u8,
    p_cstr_i: uint,
    max: uint 
}

impl cstr {
    pub unsafe fn new(size: uint) -> cstr {
	let (x, y) = memory::allocator.alloc(size);
	let this = cstr {
	    p: x,
	    p_cstr_i: 0,
	    max: y
	};
	this
    }

    unsafe fn from_str(s: &str) -> cstr {
	let mut this = cstr::new(256);
	for c in slice::iter(as_bytes(s)) {
	    this.add_char(*c);
	};
	this
    }
    
    unsafe fn add_char(&mut self, x: u8) -> bool{
	if (self.p_cstr_i == self.max) { return false; }
	*(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
	self.p_cstr_i += 1;
	*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
	true
    }

    unsafe fn delete_char(&mut self) -> bool {
	if (self.p_cstr_i == 0) { return false; }
	self.p_cstr_i -= 1;
	*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
	true
    }

    unsafe fn reset(&mut self) {
	self.p_cstr_i = 0; 
	*(self.p as *mut char) = '\0';
    }

    unsafe fn eq(&self, other: &cstr) -> bool {
	if (self.p_cstr_i != other.p_cstr_i) { return false; }
	else {
	    let mut x = 0;
	    let mut selfp: uint = self.p as uint;
	    let mut otherp: uint = other.p as uint;
	    while (x < self.p_cstr_i) {
		if (*(selfp as *char) != *(otherp as *char)) { return false; }
		selfp += 1;
		otherp += 1;
		x += 1;
	    }
	    true
	}
    }

    unsafe fn getarg(&self, delim: char, mut k: uint) -> Option<cstr> {
	let mut ind: uint = 0;
	let mut found = k == 0;
	let mut selfp: uint = self.p as uint;
	let mut s = cstr::new(256);
	loop {
	    if (*(selfp as *char) == '\0') { 
		// End of string
		if (found) { return Some(s); }
		else { return None; }

	    };
	    if (*(selfp as *u8) == delim as u8) { 
		if (found) { return Some(s); }
		k -= 1;
	    };
	    if (found) {
		s.add_char(*(selfp as *u8));
	    };
	    found = k == 0;
	    selfp += 1;
	    ind += 1;
	    if (ind == self.max) { 
		putstr(&"\nSometing broke!");
		return None; 
	    }
	}
    }

}

