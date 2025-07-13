use crate::build_time::*;
use crate::cpac;
use cpac::{modify_field, pwr, rcc, read_field, rtc};
use defmt::info;

pub fn init_hse() {
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

fn decimal_to_bcd(num: u8) -> u32 {
    (((num / 10) << 4) | (num % 10)) as u32
}

pub enum RTCOut {
    On512Hz,
    On1Hz,
}

pub fn init_lse_rtc(rtc_out: RTCOut) {
    let rcc = rcc::RCC_TypeDef::new_static_ref();
    modify_field(&mut rcc.APB1ENR, rcc::APB1ENR_PWREN_Msk, 1);

    let pwr = pwr::PWR_TypeDef::new_static_ref();
    modify_field(&mut pwr.CR, pwr::CR_DBP_Msk, 1);

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
    let time_value = decimal_to_bcd(HOUR) << rtc::TR_HU_Pos
        | decimal_to_bcd(MINUTE) << rtc::TR_MNU_Pos
        | decimal_to_bcd(SECOND) << rtc::TR_SU_Pos;

    let time_mask = rtc::TR_HT_Msk
        | rtc::TR_HU_Msk
        | rtc::TR_MNT_Msk
        | rtc::TR_MNU_Msk
        | rtc::TR_ST_Msk
        | rtc::TR_SU_Msk;

    modify_field(&mut rtc.TR, time_mask, time_value as u32);

    let date_value = decimal_to_bcd(YEAR) << rtc::DR_YU_Pos
        | WEEKDAY << rtc::DR_WDU_Pos
        | decimal_to_bcd(MONTH) << rtc::DR_MU_Pos
        | decimal_to_bcd(DAY) << rtc::DR_DU_Pos;

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

    let cosel_value = if let RTCOut::On512Hz = rtc_out { 0 } else { 1 };

    modify_field(&mut rtc.CR, rtc::CR_OSEL_Msk, 0);
    modify_field(&mut rtc.CR, rtc::CR_COSEL_Msk, cosel_value);
    modify_field(&mut rtc.CR, rtc::CR_COE_Msk, 1);
    modify_field(&mut rtc.OR, rtc::OR_RTC_OUT_RMP, 0);

    // stop the init mode
    modify_field(&mut rtc.ISR, rtc::ISR_INIT_Msk, 0);

    // enable write protection back
    modify_field(&mut rtc.WPR, rtc::WPR_KEY_Msk, 0xFF);
}
