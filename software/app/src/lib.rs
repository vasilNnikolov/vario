#![no_std]

use cortex_m_rt::exception;
use cpac::modify_field;
use defmt::info;
use defmt_rtt as _;
pub use stm32l0::stm32l0x2 as pac; // do not remove, the stm32l0 crate is needed for compilation and filling in interrupts
pub use stm32l0_cpac as cpac;

#[exception]
unsafe fn DefaultHandler(irq_num: i16) {
    info!(
        "The interrupt with number {} went to DefaultHandler",
        irq_num
    );
    loop {}
}

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

pub mod clocks {
    use super::*;
    use cpac::rcc;
    pub fn init_HSE() {
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
    pub fn init_lse() {}
}
