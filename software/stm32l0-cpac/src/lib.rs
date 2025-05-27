pub mod c_bindings;
use c_bindings::*;

/// #TODO some peripherals, ex GPIO, have one type, GPIO_TypeDef, but multiple instances (GPIO_A, GPIO_B, etc.)
macro_rules! peripheral {
    ($name:ty, $addr:expr) => {
        impl $name {
            pub fn new_ref() -> &'static mut $name {
                unsafe { &mut *($addr as *mut $name) }
            }
        }
    };
}

peripheral!(c_bindings::FLASH_TypeDef, c_bindings::FLASH_R_BASE);
peripheral!(c_bindings::CRC_TypeDef, c_bindings::CRC_BASE);
peripheral!(c_bindings::FIREWALL_TypeDef, c_bindings::FIREWALL_BASE);
peripheral!(PWR_TypeDef, PWR_BASE);
peripheral!(c_bindings::RCC_TypeDef, c_bindings::RCC_BASE);
peripheral!(CRS_TypeDef, CRS_BASE);

fn test_x() {
    let a = c_bindings::RCC_TypeDef::new_ref();
    a.CR |= c_bindings::RCC_CR_PLLRDY_Msk;
    let flash = c_bindings::FLASH_TypeDef::new_ref();
    while (a.CIFR & c_bindings::RCC_CIFR_CSSLSEF_Msk) >> c_bindings::RCC_CIFR_CSSLSEF_Pos == 0 {}
    let pwr = PWR_TypeDef::new_ref();
}
