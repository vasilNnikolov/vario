#![no_std]
use cpac_macros::peripheral;

pub mod c_bindings;

peripheral!("c_bindings.rs", RCC_TypeDef, RCC_BASE, "RCC", rcc);

const A: rcc::RCC_TypeDef = rcc::RCC_TypeDef {
    CR: 0,
    ICSCR: 0,
    CRRCR: 0,
    CFGR: 0,
    CIER: 0,
    CIFR: 0,
    CICR: 0,
    IOPRSTR: 0,
    AHBRSTR: 0,
    APB2RSTR: 0,
    APB1RSTR: 0,
    IOPENR: 0,
    AHBENR: 0,
    APB2ENR: 0,
    APB1ENR: 0,
    IOPSMENR: 0,
    AHBSMENR: 0,
    APB2SMENR: 0,
    APB1SMENR: 0,
    CCIPR: 0,
    CSR: 0,
};

const C: u32 = rcc::RCC_AHBENR_CRCEN;
const D: u32 = rcc::RCC_AHBENR_CRCEN_Msk;

// /// #TODO some peripherals, ex GPIO, have one type, GPIO_TypeDef, but multiple instances (GPIO_A, GPIO_B, etc.)
//
// macro_rules! peripheral {
//     ($name:ty, $addr:expr) => {
//         impl $name {
//             pub fn new_ref() -> &'static mut $name {
//                 unsafe { &mut *($addr as *mut $name) }
//             }
//         }
//     };
// }

// peripheral!(c_bindings::FLASH_TypeDef, c_bindings::FLASH_R_BASE);
// peripheral!(c_bindings::CRC_TypeDef, c_bindings::CRC_BASE);
// peripheral!(c_bindings::FIREWALL_TypeDef, c_bindings::FIREWALL_BASE);
// peripheral!(PWR_TypeDef, PWR_BASE);
// peripheral!(c_bindings::RCC_TypeDef, c_bindings::RCC_BASE);
// peripheral!(CRS_TypeDef, CRS_BASE);

// fn test_x() {
//     let a = c_bindings::RCC_TypeDef::new_ref();
//     a.CR |= c_bindings::RCC_CR_PLLRDY_Msk;
//     let flash = c_bindings::FLASH_TypeDef::new_ref();
//     while (a.CIFR & c_bindings::RCC_CIFR_CSSLSEF_Msk) >> c_bindings::RCC_CIFR_CSSLSEF_Pos == 0 {}
//     let pwr = PWR_TypeDef::new_ref();
// }
