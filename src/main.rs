#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate nrf52840_hal;
extern crate cortex_m_log;

use core;
use cortex_m_rt::{entry, exception};
/*
use cortex_m_log::{println};
use cortex_m_log::printer::Dummy;
*/
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello world").unwrap();
    loop {
    hprintln!("LOOP").unwrap();
    }
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
