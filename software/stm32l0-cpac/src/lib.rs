#![no_std]
use cpac_macros::peripheral;
pub use volatile_register;

pub mod c_bindings;

peripheral!("c_bindings.rs", RCC_TypeDef, RCC_BASE, "RCC_", rcc);
peripheral!("c_bindings.rs", GPIO_TypeDef, GPIOA_BASE, "GPIO_", gpio_a);
peripheral!("c_bindings.rs", GPIO_TypeDef, GPIOB_BASE, "GPIO_", gpio_b);
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
