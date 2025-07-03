#![no_std]

use cortex_m_rt::exception;
use cpac::{modify_field, read_field};
use defmt::info;
use defmt_rtt as _;
pub use stm32l0::stm32l0x2 as pac; // do not remove, the stm32l0 crate is needed for compilation and filling in interrupts
pub use stm32l0_cpac as cpac;

pub const CPU_FREQ: u32 = 16_000_000;
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

include!(concat!(env!("OUT_DIR"), "/compiled_time.rs"));

pub mod clocks {
    use super::*;
    use cpac::pwr;
    use cpac::rcc;
    use cpac::rtc;
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

    fn decimalToBcd(num: u8) -> u32 {
        (((num / 10) << 4) | (num % 10)) as u32
    }

    pub enum RTCOUT {
        ON_512Hz,
        ON_1Hz,
    }
    pub fn init_lse_RTC(rtc_out: RTCOUT) {
        let rcc = rcc::RCC_TypeDef::new_static_ref();
        modify_field(&mut rcc.APB1ENR, rcc::APB1ENR_PWREN_Msk, 1);

        let pwr = pwr::PWR_TypeDef::new_static_ref();
        modify_field(&mut pwr.CR, pwr::CR_DBP_Msk, 1);
        // modify_field(&mut rcc.CSR, rcc::CSR_LSEDRV_Msk, 0b01);

        modify_field(&mut rcc.CSR, rcc::CSR_LSEON_Msk, 1);

        loop {
            let lse_rdy = (rcc.CSR.read() & rcc::CSR_LSERDY_Msk) >> rcc::CSR_LSERDY_Pos;
            info!("LSE RDY: {}", lse_rdy);
            if lse_rdy == 1 {
                break;
            }
            cortex_m::asm::delay(1000);
        }
        info!("LSE started");

        modify_field(&mut rcc.CSR, rcc::CSR_RTCSEL_Msk, 0b01); // LSE
        modify_field(&mut rcc.CSR, rcc::CSR_RTCEN_Msk, 1);

        let rtc = cpac::rtc::RTC_TypeDef::new_static_ref();

        // enable writes to RTC registers
        modify_field(&mut rtc.WPR, rtc::WPR_KEY_Msk, 0xCA);
        modify_field(&mut rtc.WPR, rtc::WPR_KEY_Msk, 0x53);

        // calendar initialization
        modify_field(&mut rtc.ISR, rtc::ISR_INIT_Msk, 1);
        info!("polling INITF bit");
        while read_field(&rtc.ISR, rtc::ISR_INITF_Msk) == 0 {}
        info!("INITF bit = 1");

        modify_field(&mut rtc.TR, rtc::TR_PM_Msk, 0);
        let time_value = decimalToBcd(HOUR) << rtc::TR_HU_Pos
            | decimalToBcd(MINUTE) << rtc::TR_MNU_Pos
            | decimalToBcd(SECOND) << rtc::TR_SU_Pos;

        let time_mask = rtc::TR_HT_Msk
            | rtc::TR_HU_Msk
            | rtc::TR_MNT_Msk
            | rtc::TR_MNU_Msk
            | rtc::TR_ST_Msk
            | rtc::TR_SU_Msk;

        modify_field(&mut rtc.TR, time_mask, time_value as u32);

        let date_value = decimalToBcd(YEAR) << rtc::DR_YU_Pos
            | WEEKDAY << rtc::DR_WDU_Pos
            | decimalToBcd(MONTH) << rtc::DR_MU_Pos
            | decimalToBcd(DAY) << rtc::DR_DU_Pos;

        let date_mask = rtc::DR_YT_Msk
            | rtc::DR_YU_Msk
            | rtc::DR_WDU_Msk
            | rtc::DR_MT_Msk
            | rtc::DR_MU_Msk
            | rtc::DR_DT_Msk
            | rtc::DR_DU_Msk;

        modify_field(&mut rtc.DR, date_mask, date_value as u32);
        info!(
            "RTC initialized to {}.{}.20{}T{}H{}M{}s, weekday {}",
            DAY, MONTH, YEAR, HOUR, MINUTE, SECOND, WEEKDAY
        );

        // set RTC OUT on pin PC13

        let cosel_value = if let RTCOUT::ON_512Hz = rtc_out { 0 } else { 1 };

        modify_field(&mut rtc.CR, rtc::CR_OSEL_Msk, 0);
        modify_field(&mut rtc.CR, rtc::CR_COSEL_Msk, cosel_value);
        modify_field(&mut rtc.CR, rtc::CR_COE_Msk, 1);
        modify_field(&mut rtc.OR, rtc::OR_RTC_OUT_RMP, 0);

        // stop the init mode
        modify_field(&mut rtc.ISR, rtc::ISR_INIT_Msk, 0);

        // enable write protection back
        modify_field(&mut rtc.WPR, rtc::WPR_KEY_Msk, 0xFF);
    }
}

/// not accurate
struct BusyLoopDelayNs;

impl embedded_hal::delay::DelayNs for BusyLoopDelayNs {
    fn delay_ns(&mut self, ns: u32) {
        let d_cycles = ns as u64 * CPU_FREQ as u64 / 1_000_000_000 as u64;
        cortex_m::asm::delay(d_cycles as u32);
    }
}

/// module to handle the push switches
/// SW1 on pin PB5, SW2 on PA0, SW3 on PB6
pub mod switches {
    use super::*;
    pub fn init_switches() {
        let rcc = cpac::rcc::RCC_TypeDef::new_static_ref();
        modify_field(&mut rcc.IOPENR, cpac::rcc::IOPENR_IOPBEN_Msk, 1);

        let pb = cpac::gpio_b::GPIO_TypeDef::new_static_ref();

        // SW1
        modify_field(&mut pb.MODER, cpac::gpio_b::MODER_MODE5, 0b00); // set to input
        modify_field(&mut pb.PUPDR, cpac::gpio_b::PUPDR_PUPD5, 0b00); // set no pull-up, no pull-down
    }

    pub fn get_sw1() -> bool {
        let pb = cpac::gpio_b::GPIO_TypeDef::new_static_ref();

        read_field(&pb.IDR, cpac::gpio_b::IDR_ID5_Msk) == 1
    }
}
