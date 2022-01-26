#![no_std]
#![no_main]
// For allocator
#![feature(lang_items)]
#![feature(alloc_error_handler)]

use cortex_m;
use cortex_m::asm;
use cortex_m_rt::{exception, entry, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use ioc_compiled::init_DMA2_Stream0;
extern crate stm32f4xx_hal;
extern crate panic_halt;
use core::alloc::Layout;


mod ioc_compiled;


#[entry]
fn main() -> ! {
    
    loop {
        
    }
}

/// For a full list of available exception handlers see
/// the documentation of `cortex-m-rt::exception`
#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    panic!("Exception: {}", _irqn);
}

#[exception]
unsafe fn MemoryManagement() -> ! {
    hprintln!("A memory management exception occured.").unwrap();
    asm::bkpt();
    loop {}
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();
    loop {}
}