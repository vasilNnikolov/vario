#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

use bsp as _; // do not remove, the stm32l0 crate is needed for compilation and filling in interrupts

use cortex_m_rt::{entry, exception};
use defmt::info;

use cpac::{gpio_b, modify_field, rcc};
use stm32l0_cpac as cpac;
use systick::{get_systic_ticks, init_systick};

const CPU_FREQ: u32 = 16_000_000;

pub mod systick {
    use super::*;

    /// must be modified ONLY in the SysTick exception handler
    static mut SYSTICK_TICKS: u64 = 0;

    pub fn get_systic_ticks() -> u64 {
        unsafe { SYSTICK_TICKS }
    }

    #[exception]
    fn SysTick() {
        unsafe {
            SYSTICK_TICKS = SYSTICK_TICKS.wrapping_add(1);
        }
    }

    pub fn init_systick(reload_value: u32) {
        if reload_value > 0x00ffffff {
            defmt::error!(
                "the SYST reload value must be <= 0x00ffffff, tried to set {}",
                reload_value
            );
        }
        let st = cpac::systick::SysTick_Type::new_static_ref();
        modify_field(&mut st.LOAD, cpac::systick::LOAD_RELOAD_Msk, reload_value);
        modify_field(&mut st.VAL, cpac::systick::VAL_CURRENT_Msk, 0);
        modify_field(&mut st.CTRL, cpac::systick::CTRL_CLKSOURCE_Msk, 1);
        modify_field(&mut st.CTRL, cpac::systick::CTRL_TICKINT_Msk, 1);
        modify_field(&mut st.CTRL, cpac::systick::CTRL_ENABLE_Msk, 1);
    }
}
struct BusyLoopDelayNs;

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
    init_systick(CPU_FREQ - 1);
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
        // turn PB12 on
        modify_field(&mut gpio_b.BSRR, gpio_b::BSRR_BS_12, 1);
        bld.delay_ms(100);

        // turn PB12 off
        modify_field(&mut gpio_b.BSRR, gpio_b::BSRR_BR_12, 1);
        bld.delay_ms(900);
        i += 1;
        info!("Counter {}, Uptime {}s", i, get_systic_ticks());
    }
}
