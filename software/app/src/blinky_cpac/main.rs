#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

use bsp as _; // do not remove, the stm32l0 crate is needed for compilation and filling in interrupts
use cortex_m_rt::entry;
use defmt::info;

use cpac::{gpio_b, modify_field, rcc};
use stm32l0_cpac as cpac;

struct BusyLoopDelayNs;

const CPU_FREQ: u32 = 16_000_000;
impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        let d_cycles = ns as u64 * CPU_FREQ as u64 / 1_000_000_000 as u64;
        cortex_m::asm::delay(d_cycles as u32);
    }
}

fn init_HSE() {
    let rcc = rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.CR, rcc::CR_HSEON_Msk, 1);
    loop {
        info!("Getting HSE status");
        let is_hse_on = (rcc.CR.read() & rcc::CR_HSERDY_Msk) >> rcc::CR_HSERDY_Pos;
        if is_hse_on == 1 {
            info!("HSE ready");
            break;
        }
        info!("HSE not ready")
    }
    modify_field(&mut rcc.CFGR, rcc::CFGR_SW, 0b10);
    loop {
        let hse_switch_status = (rcc.CFGR.read() & rcc::CFGR_SWS) >> rcc::CFGR_SWS_Pos;
        info!("status of HSE switch: {}", hse_switch_status);
        if hse_switch_status == 0b10 {
            break;
        }
    }
    info!("switched to HSE");
}

#[entry]
fn main() -> ! {
    info!("Start");
    init_HSE();
    let rcc = rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.IOPENR, rcc::IOPENR_IOPBEN_Msk, 1);

    let gpio_b = cpac::gpio_b::GPIO_TypeDef::new_static_ref();
    // set output mode
    modify_field(&mut gpio_b.MODER, gpio_b::MODER_MODE12, 0b01);
    // set output type to push-pull
    modify_field(&mut gpio_b.OTYPER, gpio_b::OTYPER_OT_12, 0b0);

    // set no pull-up & no pull-down
    modify_field(&mut gpio_b.PUPDR, gpio_b::PUPDR_PUPD12, 0b00);

    let mut bld = BusyLoopDelayNs;
    let mut i = 0;
    loop {
        info!("Counter: {}", i);
        // turn PB12 on
        modify_field(&mut gpio_b.BSRR, gpio_b::BSRR_BS_12, 1);
        bld.delay_ms(100);

        // turn PB12 off
        modify_field(&mut gpio_b.BSRR, gpio_b::BSRR_BR_12, 1);
        bld.delay_ms(100);
        i += 1;
    }
}
