#![no_std]
#![no_main]

use panic_halt as _;

use bsp;

use cortex_m_rt::entry;
use defmt::info;

use cpac::{gpio_b, modify_field, rcc};
use stm32l0_cpac as cpac;

#[inline(always)]
fn enter_sleep() {
    // TODO investigate further
    cortex_m::asm::dsb();
    cortex_m::asm::wfi();
    cortex_m::asm::isb();
}

fn init_dbg() {
    let dbgmcu = cpac::dbgmcu::DBGMCU_TypeDef::new_static_ref();
    modify_field(&mut dbgmcu.CR, cpac::dbgmcu::CR_DBG_SLEEP_Msk, 1);
    modify_field(&mut dbgmcu.CR, cpac::dbgmcu::CR_DBG_STANDBY_Msk, 1);
    modify_field(&mut dbgmcu.CR, cpac::dbgmcu::CR_DBG_STOP_Msk, 1);
    let rcc = rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.AHBENR, cpac::rcc::AHBENR_DMA1EN, 1);
}

#[entry]
fn main() -> ! {
    info!("Start");
    bsp::clocks::init_HSE();
    bsp::clocks::init_lse_RTC(bsp::clocks::RTCOUT::On1Hz);
    bsp::systick::init_systick(bsp::CPU_FREQ - 1);
    bsp::switches::init_switches();

    init_dbg();
    let rcc = rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.IOPENR, rcc::IOPENR_IOPBEN_Msk, 1);

    let gpio_b = cpac::gpio_b::GPIO_TypeDef::new_static_ref();
    modify_field(&mut gpio_b.MODER, gpio_b::MODER_MODE12, 0b01); // set output mode
    modify_field(&mut gpio_b.OTYPER, gpio_b::OTYPER_OT_12, 0b0); // set output type to push-pull
    modify_field(&mut gpio_b.PUPDR, gpio_b::PUPDR_PUPD12, 0b00); // set no pull-up & no pull-down

    let mut i = 0;
    let rtc = cpac::rtc::RTC_TypeDef::new_static_ref();
    loop {
        if i % 2 == 0 {
            // turn PB12 on
            modify_field(&mut gpio_b.BSRR, gpio_b::BSRR_BS_12, 1);
        } else {
            // turn PB12 off
            modify_field(&mut gpio_b.BSRR, gpio_b::BSRR_BR_12, 1);
        }
        i += 1;
        info!("Uptime {}s", bsp::systick::get_systic_ticks());

        let date_reg = cpac::read_field(&mut rtc.DR, u32::MAX);
        let time_reg = cpac::read_field(&mut rtc.TR, u32::MAX);
        info!("date reg: {=u32:x}", date_reg);
        info!("time reg: {=u32:x}", time_reg);

        let sw1_state = bsp::switches::get_sw1();
        info!("SW1 state: {}", sw1_state);
        enter_sleep();
    }
}
