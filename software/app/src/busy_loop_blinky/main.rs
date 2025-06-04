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
        let d_cycles = ns as u64 * CPU_FREQ as u64 / 1_000_000_000 as u64;
        cortex_m::asm::delay(d_cycles as u32);
    }
}

#[entry]
fn main() -> ! {
    info!("Start");
    let _core_p = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    p.RCC.iopenr.modify(|_, w| w.iopben().set_bit());

    // LEDs are PB12,13,14
    let gpiob = &p.GPIOB;

    gpiob.moder.modify(|_, w| w.mode12().output());
    gpiob.otyper.modify(|_, w| w.ot12().clear_bit());
    gpiob.pupdr.modify(|_, w| w.pupd12().floating());

    gpiob.moder.modify(|_, w| w.mode13().output());
    gpiob.otyper.modify(|_, w| w.ot13().clear_bit());
    gpiob.pupdr.modify(|_, w| w.pupd13().floating());

    gpiob.moder.modify(|_, w| w.mode14().output());
    gpiob.otyper.modify(|_, w| w.ot14().clear_bit());
    gpiob.pupdr.modify(|_, w| w.pupd14().floating());

    let mut bld = BusyLoopDelayNs;
    let mut i = 0;
    loop {
        info!("Counter: {}", i);
        p.GPIOB.bsrr.write(|w| w.br12().set_bit());
        p.GPIOB.bsrr.write(|w| w.bs13().set_bit());
        p.GPIOB.bsrr.write(|w| w.br14().set_bit());

        bld.delay_ms(100);

        p.GPIOB.bsrr.write(|w| w.bs12().set_bit());
        p.GPIOB.bsrr.write(|w| w.br13().set_bit());
        p.GPIOB.bsrr.write(|w| w.bs14().set_bit());

        bld.delay_ms(100);
        i += 1;
    }
}
