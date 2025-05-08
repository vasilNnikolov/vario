#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;

use bsp::pac;

struct BusyLoopDelayNs;

impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        const CPU_FREQ: u32 = 2_000_000;
        cortex_m::asm::delay(ns * CPU_FREQ / 1_000_000_000);
    }
}

#[entry]
fn main() -> ! {
    info!("Start");
    let _core_p = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    // the middle LED (LED2 in kicad) is PB13
    p.RCC.iopenr.modify(|_, w| w.iopben().set_bit());
    p.GPIOB.moder.modify(|_, w| w.mode13().output());
    p.GPIOB.otyper.modify(|_, w| w.ot13().push_pull());

    let mut bld = BusyLoopDelayNs;
    let mut i = 0;
    loop {
        info!("Counter: {}", i);
        p.GPIOA.bsrr.write(|w| w.bs8().set_bit());
        bld.delay_ms(500);
        p.GPIOA.bsrr.write(|w| w.br8().set_bit());
        bld.delay_ms(500);
        i += 1;
    }
}
