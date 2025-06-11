#![no_std]
use cpac_macros::peripheral;
pub use volatile_register;

pub mod c_bindings;

peripheral!("c_bindings.rs", RCC_TypeDef, RCC_BASE, "RCC_", rcc);
peripheral!("c_bindings.rs", ADC_TypeDef, ADC_BASE, "ADC_", adc);

fn test_a() {
    let mut r = rcc::RCC_TypeDef::new_static_ref();
    let _ = rcc::AHBENR_CRCEN;

    let mut a = adc::ADC_TypeDef::new_static_ref();
    let _ = a.RESERVED5.read();
}

// peripheral!(c_bindings::FLASH_TypeDef, c_bindings::FLASH_R_BASE);
// peripheral!(c_bindings::CRC_TypeDef, c_bindings::CRC_BASE);
// peripheral!(c_bindings::FIREWALL_TypeDef, c_bindings::FIREWALL_BASE);
// peripheral!(PWR_TypeDef, PWR_BASE);
// peripheral!(c_bindings::RCC_TypeDef, c_bindings::RCC_BASE);
// peripheral!(CRS_TypeDef, CRS_BASE);
