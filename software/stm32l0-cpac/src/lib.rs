#![no_std]
use cpac_macros::peripheral;
pub use volatile_register;

pub mod c_bindings;

peripheral!("c_bindings.rs", RCC_TypeDef, RCC_BASE, "RCC_", rcc);
peripheral!("c_bindings.rs", GPIO_TypeDef, GPIOA_BASE, "GPIO_", gpio_a);
peripheral!("c_bindings.rs", GPIO_TypeDef, GPIOB_BASE, "GPIO_", gpio_b);
peripheral!(
    "c_bindings.rs",
    SysTick_Type,
    SysTick_BASE,
    "SysTick_",
    systick
);
peripheral!(
    "c_bindings.rs",
    DBGMCU_TypeDef,
    DBGMCU_BASE,
    "DBGMCU_",
    dbgmcu
);
// peripheral!("c_bindings.rs", RCC_TypeDef, RCC_BASE, "RCC_", rcc);
// peripheral!("c_bindings.rs", ADC_TypeDef, ADC_BASE, "ADC_", adc);

/// modifies a register.
/// `clear_msk` is a mask at the position of the bits you want to clear
/// `pos`: position of the value in the register
/// `value`: what you want written to the register.
///
/// if the `safe-modify-reg` feature is enabled, the function will check if `value<<pos` is fully inside `clear_mask`, and if not, will panic
#[inline(always)]
pub fn modify_reg(reg: &mut volatile_register::RW<u32>, clear_mask: u32, pos: u32, value: u32) {
    #[cfg(feature = "safe-modify-reg")]
    if (value << pos) & (!clear_mask) != 0 {
        panic!(
            "tried to modify a register with clear_mask: {}, position: {}, value: {}",
            clear_mask, pos, value
        );
    }

    unsafe { reg.modify(|x| (x & (!clear_mask)) | (value << pos)) }
}

/// modifies a register field.
/// `field_mask` is a mask that shows the field (ex 0x00000010)
/// `value`: what you want written to the register.
///
/// if the `safe-modify-reg` feature is enabled, the function will check if `value` is fully inside `clear_mask`, and if not, will panic
#[inline(always)]
pub fn modify_field(reg: &mut volatile_register::RW<u32>, field_mask: u32, value: u32) {
    let pos = field_mask.trailing_zeros(); // zero-cost abstraction if clear_mask is a constant

    #[cfg(feature = "safe-modify-reg")]
    if (value << pos) & (!field_mask) != 0 {
        panic!(
            "tried to modify a register with clear_mask: {}, position: {}, value: {}",
            field_mask, pos, value
        );
    }

    unsafe { reg.modify(|x| (x & (!field_mask)) | (value << pos)) }
}
