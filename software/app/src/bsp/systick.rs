use super::*;

/// number of times the SysTick peripheral has ticked
/// SAFETY:  must be modified ONLY in the SysTick exception handler
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
    {
        use cpac::systick::*;
        modify_field(&mut st.LOAD, VAL_CURRENT_Msk, reload_value);
        modify_field(&mut st.VAL, VAL_CURRENT_Msk, 0);
        modify_field(&mut st.CTRL, CTRL_CLKSOURCE_Msk, 1);
        modify_field(&mut st.CTRL, CTRL_TICKINT_Msk, 1);
        modify_field(&mut st.CTRL, CTRL_ENABLE_Msk, 1);
    }
}
