// Rust on BareMetal
// rustc -O --crate-type lib -o rust.o --emit obj rust.rs
// ld -T app.ld -o rust.app rust.o

// do not include the standard library
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

// Main entry
#[unsafe(no_mangle)]
pub extern "C" fn _start()
{
	let s = "\nHello world from Rust!";

	// Output the string via the kernel API
	unsafe
	{
		asm!(
			"mov rsi, {0}", // string address
			"mov rcx, {1}", // string length
			"call [0x00100018]", // b_output
			in(reg) s.as_ptr(),
			in(reg) s.len()
		);
	}

}

// This function is called during panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
        loop {}
}
