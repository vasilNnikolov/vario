#![no_std]
#![no_main]

use panic_halt as _;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::{entry, exception};
use defmt::info;
use defmt_rtt as _;
use stm32l0::stm32l0x2 as pac;

static SYST_TICKS: AtomicU32 = AtomicU32::new(0);

fn init_systick(systick: &mut pac::SYST, reload_value: u32) {
    if reload_value > 0x00ffffff {
        defmt::error!(
            "the SYST reload value must be <= 0x00ffffff, tried to set {}",
            reload_value
        );
    }
    systick.set_reload(reload_value);
    systick.clear_current();
    systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    systick.enable_interrupt();
    systick.enable_counter();
}

#[entry]
fn main() -> ! {
    info!("Start");
    let mut core_p = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    init_systick(&mut core_p.SYST, 2_000_000);

    loop {
        info!(
            "SysTick has ticked {} times",
            SYST_TICKS.load(Ordering::Relaxed)
        );
        cortex_m::asm::wfi();
    }
}

#[exception]
fn SysTick() {
    cortex_m::interrupt::free(|_| {
        SYST_TICKS.store(SYST_TICKS.load(Ordering::Relaxed) + 1, Ordering::Relaxed)
    })
}
