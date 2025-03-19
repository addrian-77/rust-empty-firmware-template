#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print a panic message
    hprintln!("Panic occurred: {:?}", info);

    // Enter an infinite loop to halt the system after a panic
    loop {}
}