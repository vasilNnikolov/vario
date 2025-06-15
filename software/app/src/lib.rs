#![no_std]

use cortex_m_rt::exception;
use defmt::info;
use defmt_rtt as _;
pub use stm32l0::stm32l0x2 as pac;

#[exception]
unsafe fn DefaultHandler(irq_num: i16) {
    info!(
        "The interrupt with number {} went to DefaultHandler",
        irq_num
    );
    loop {}
}
