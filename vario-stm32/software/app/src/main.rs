#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32l0::stm32l0x1;

#[entry]
fn main() -> ! {
    let core_p = cortex_m::Peripherals::take().unwrap();
    let p = stm32l0x1::Peripherals::take().unwrap();
    info!("abcd");

    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
        asm::nop();
    }
}
