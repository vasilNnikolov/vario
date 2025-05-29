#![no_std]
#![no_main]
use panic_halt as _;

use bme280_bindings_rs;
use cortex_m_rt::{entry, exception};
use defmt::info;
use defmt_rtt as _;
use stm32l0::stm32l0x2 as pac;

pub mod systick {
    use super::*;

    pub const SYSTICK_FREQ: u32 = 16_000_000;
    /// must be modified ONLY in the SysTick exception handler
    static mut SYSTICK_TICKS: u64 = 0;

    pub fn get_systic_ticks(_cs: critical_section::CriticalSection<'_>) -> u64 {
        unsafe { SYSTICK_TICKS }
    }

    #[exception]
    fn SysTick() {
        critical_section::with(|_| unsafe {
            SYSTICK_TICKS = SYSTICK_TICKS.wrapping_add(1);
        })
    }

    pub fn init_systick(systick: &mut pac::SYST, reload_value: u32) {
        if reload_value > 0x00ffffff {
            defmt::error!(
                "the SYST reload value must be <= 0x00ffffff, tried to set {}",
                reload_value
            );
        }
        systick.set_reload(reload_value & 0x00ffffff);
        systick.clear_current();
        systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        systick.enable_interrupt();
        systick.enable_counter();
    }
}

#[entry]
fn main() -> ! {
    info!("Start");
    let mut core_p = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    systick::init_systick(&mut core_p.SYST, 16_000_000);
    let dev = bme280_bindings_rs::bindings::bme280_dev::default();

    loop {
        let ticks = critical_section::with(|cs| systick::get_systic_ticks(cs));
        info!("SysTick has ticked {} times", ticks);
        // add busy loop to check systick is indeed increasing
        if ticks > 10 {
            for _ in 0..16_000_000 {
                cortex_m::asm::nop();
            }
        }
        cortex_m::asm::wfi();
    }
}
