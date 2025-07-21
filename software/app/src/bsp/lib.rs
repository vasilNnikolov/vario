#![no_std]

use cortex_m_rt::exception;
use cpac::modify_field;
use defmt::info;
use defmt_rtt as _;
// dev note
// do not remove, the stm32l0 crate is needed for compilation and filling in interrupts
pub use stm32l0::stm32l0x2 as pac;
pub use stm32l0_cpac as cpac;

pub mod clocks;
pub mod leds;

/// module to handle the push switches
/// SW1 on pin PB5, SW2 on PA0, SW3 on PB6
pub mod switches;
pub mod systick;

pub mod usb;
pub const CPU_FREQ: u32 = 16_000_000;

#[exception]
unsafe fn DefaultHandler(irq_num: i16) {
    defmt::debug!(
        "IRQ or event with number {} went to DefaultHandler, looping",
        irq_num
    );
    loop {}
}

#[inline(always)]
pub fn enter_sleep() {
    // TODO investigate further
    cortex_m::asm::dsb();
    cortex_m::asm::wfi();
    cortex_m::asm::isb();
}

fn init_dbg() {
    let dbgmcu = cpac::dbgmcu::DBGMCU_TypeDef::new_static_ref();
    modify_field(&mut dbgmcu.CR, cpac::dbgmcu::CR_DBG_SLEEP_Msk, 1);
    modify_field(&mut dbgmcu.CR, cpac::dbgmcu::CR_DBG_STOP_Msk, 1);
    modify_field(&mut dbgmcu.CR, cpac::dbgmcu::CR_DBG_STANDBY_Msk, 0);

    let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.AHBENR, cpac::rcc::AHBENR_DMA1EN, 1);
}

pub fn init() {
    clocks::init_hse();
    clocks::init_lse_rtc(clocks::RTCOut::On1Hz);
    leds::init_leds();
    systick::init_systick(CPU_FREQ - 1);
    switches::init_switches();
    // init_dbg();
}

pub fn configure_standby_mode() {
    info!("entering stop mode");
    let mut cp = unsafe { pac::CorePeripherals::steal() };
    cp.SCB.set_sleepdeep();
    cp.SCB.clear_sleeponexit();
    let pwr = cpac::pwr::PWR_TypeDef::new_static_ref();
    modify_field(&mut pwr.CR, cpac::pwr::CR_DBP_Msk, 1);
    modify_field(&mut pwr.CR, cpac::pwr::CR_PDDS_Msk, 1);
    modify_field(&mut pwr.CSR, cpac::pwr::CSR_WUF_Msk, 0);
    modify_field(&mut pwr.CSR, cpac::pwr::CSR_EWUP1_Msk, 1);
}

/// not accurate
pub struct BusyLoopDelayNs;

impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        let d_cycles = ns as u64 * CPU_FREQ as u64 / 1_000_000_000 as u64;
        cortex_m::asm::delay(d_cycles as u32);
    }
}
