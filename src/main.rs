#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use nrf52840_hal::prelude::*;
//use panic_itm;
use cortex_m::{iprintln, peripheral::ITM};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut itm = init();

    loop {
        iprintln!(&mut itm.stim[0], "Blinky button demo starting");
    }
}
