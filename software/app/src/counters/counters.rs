#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32l0::stm32l0x2::{self as pac, lptim::cfgr::COUNTMODE_R};

// TODO think of an abstraction to always call enable_flash and disable_flash when writing the counters

pub fn enable_flash_write(p: &mut pac::Peripherals) {
    p.RCC.ahbenr.modify(|_, w| w.mifen().set_bit());

    p.FLASH.pekeyr.write(|w| w.bits(0x89ABCDEF)); // First key
    p.FLASH.pekeyr.write(|w| w.bits(0x02030405)); // Second key

    while p.FLASH.sr.read().bsy().bit_is_set() {}
}
pub fn disable_flash_write(p: &mut pac::Peripherals) {}

#[derive(Clone, Copy)]
pub struct Counters {
    total_reset: u32,
    /// power on counter
    por: u32,
    /// user reset counter
    user_reset: u32,
}

#[used]
#[link_section = ".persistent_counters"]
pub static mut COUNTERS: Counters = Counters {
    total_reset: 0,
    por: 0,
    user_reset: 0,
};

fn log_resets(rcc: &pac::RCC) {
    let low_power_reset = rcc.csr.read().porrstf().bit_is_set();
    info!("POR true/false: {}", low_power_reset);
    unsafe {
        COUNTERS.total_reset += 1;
        if low_power_reset {
            COUNTERS.por += 1;
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Start");
    let _core_p = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    log_resets(&p.RCC);
    loop {}
}
