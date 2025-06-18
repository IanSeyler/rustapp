// Rust on bare metal - Tested with Rust 1.0.0
// Uses Pure64 to get into a 64-bit SMP state
// rustc -O --crate-type lib -o kernel64.o --emit obj kernel64.rs
// ld -T app.ld -o kernel64.sys kernel64.o

// do not include the standard library
#![no_std]

//use core::panic::PanicInfo;
use core::arch::asm;

// This function is called during panic
//#[panic_handler]
//fn panic(_info: &PanicInfo) -> !
//{
//    loop {}
//}

// Startup message
static MESSAGE: &[u8] = b"\nThis is a free-standing program written entirely in Rust!";

// Main entry
#[unsafe(no_mangle)]
pub fn main() {

	// Assign a pointer to the string
	let ptr = MESSAGE.as_ptr();

	// Output the string via the kernel API
	unsafe
	{
		asm!(
			"mov rsi, {0}",
			"mov rcx, 58",
			"call [0x00100018]",
			in(reg) ptr,
		);
	}

	// debug
	//loop { }
}
