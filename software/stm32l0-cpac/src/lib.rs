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
/// Depending on whether the `safe-modify-reg` feature is enabled, the function has different behavior when `value` does not overlap correctly with `clear_mask`
/// **Feature enabled**:
/// the value is ANDed with `clear_mask`
///
///
/// **Feature disabled**:
/// If `value` is more bits than the `clear_mask`, the extra bits will get OR-ed with the original value
/// ```
/// reg:        0b00000000000000000000000000000000
/// clear_mask: 0b00000000000000000000000001111000
/// value:      0b00000000000000000000000000010101
/// pos: 3
/// reg after:  0b00000000000000000000000010101000
/// ```
#[inline(always)]
pub fn modify_reg(reg: &mut volatile_register::RW<u32>, clear_mask: u32, pos: u32, value: u32) {
    #[cfg(feature = "safe-modify-reg")]
    unsafe {
        reg.modify(|x| (x & (!clear_mask)) | ((value << pos) & (!clear_mask)))
    }
    #[cfg(not(feature = "safe-modify-reg"))]
    unsafe {
        reg.modify(|x| (x & (!clear_mask)) | (value << pos))
    }
}

// fn test_a() {
//     let mut r = rcc::RCC_TypeDef::new_static_ref();
//     let _ = rcc::AHBENR_CRCEN;

//     let mut a = adc::ADC_TypeDef::new_static_ref();
//     let _ = a.RESERVED5.read();
// }

// peripheral!(c_bindings::FLASH_TypeDef, c_bindings::FLASH_R_BASE);
// peripheral!(c_bindings::CRC_TypeDef, c_bindings::CRC_BASE);
// peripheral!(c_bindings::FIREWALL_TypeDef, c_bindings::FIREWALL_BASE);
// peripheral!(PWR_TypeDef, PWR_BASE);
// peripheral!(c_bindings::RCC_TypeDef, c_bindings::RCC_BASE);
// peripheral!(CRS_TypeDef, CRS_BASE);
