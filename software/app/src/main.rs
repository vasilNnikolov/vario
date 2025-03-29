#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use stm32l0::stm32l0x1;

fn log_resets(rcc: &stm32l0x1::RCC) {
    let low_power_reset = rcc.csr.read().porrstf().bit_is_set();
    info!("POR true/false: {}", low_power_reset);
}

struct BusyLoopDelayNs;

impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        for _ in 0..(ns * 8) {
            asm::nop();
        }
    }
}

#[entry]
fn main() -> ! {
    info!("Start");
    let _core_p = cortex_m::Peripherals::take().unwrap();
    let p = stm32l0x1::Peripherals::take().unwrap();

    log_resets(&p.RCC);
    p.RCC.iopenr.modify(|_, w| w.iopaen().bit(true));
    p.GPIOA.moder.modify(|_, w| w.mode8().output());
    p.GPIOA.otyper.modify(|_, w| w.ot8().push_pull());

    let mut bld = BusyLoopDelayNs;
    let mut i = 0;
    loop {
        info!("Counter: {}", i);
        if i & 1 == 0 {
            // turn on
            p.GPIOA.bsrr.write(|w| w.bs8().set_bit());
        } else {
            // turn off
            p.GPIOA.bsrr.write(|w| w.br8().set_bit());
        }

        i += 1;
        bld.delay_ms(1000);
    }
}
